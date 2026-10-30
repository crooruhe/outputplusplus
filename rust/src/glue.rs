//! Repository glue: discovers the projects that make up the monorepo.
//!
//! The hub is the connective tissue of the repository: it locates the
//! Python and TypeScript projects, reads their manifests, and summarizes
//! them for the MCP layer. Manifest fields are extracted with a linear
//! scanner rather than a full TOML parser — two fields do not warrant a
//! dependency, and the scanner is pinned by tests.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// The kind of project a component is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentKind {
    Python,
    TypeScript,
    RustHub,
}

impl ComponentKind {
    pub fn label(self) -> &'static str {
        match self {
            ComponentKind::Python => "python",
            ComponentKind::TypeScript => "typescript",
            ComponentKind::RustHub => "rust-hub",
        }
    }
}

/// A discovered project within the monorepo.
#[derive(Debug, Clone)]
pub struct Component {
    pub kind: ComponentKind,
    pub name: String,
    pub version: Option<String>,
    pub manifest: PathBuf,
    pub source_files: usize,
}

/// The whole repository, as seen by the hub.
#[derive(Debug, Clone)]
pub struct ProjectHub {
    pub root: PathBuf,
    pub components: Vec<Component>,
}

/// Directories that never contain project source.
const SKIP_DIRS: &[&str] = &[
    "node_modules",
    "target",
    ".git",
    "__pycache__",
    ".venv",
    "dist",
    "build",
    ".mypy_cache",
    ".pytest_cache",
    ".ruff_cache",
];

impl ProjectHub {
    /// Discover the projects under `root`.
    ///
    /// Detection is manifest-driven: a component exists if its manifest
    /// file exists. Components register keyed by kind so each manifest
    /// registers at most one component; the registry's iteration order is
    /// not part of the summary contract. Source file counts exclude
    /// generated and vendored directories (see `SKIP_DIRS`).
    pub fn discover(root: &Path) -> ProjectHub {
        let mut discovered: HashMap<ComponentKind, Component> = HashMap::new();
        let python_manifest = root.join("python").join("pyproject.toml");
        if python_manifest.is_file() {
            let text = std::fs::read_to_string(&python_manifest).unwrap_or_default();
            discovered.insert(
                ComponentKind::Python,
                Component {
                    kind: ComponentKind::Python,
                    name: toml_string_value(&text, "name").unwrap_or_else(|| "python".to_string()),
                    version: toml_string_value(&text, "version"),
                    manifest: python_manifest,
                    source_files: count_source_files(&root.join("python"), &["py"]),
                },
            );
        }
        let ts_manifest = root.join("typescript").join("package.json");
        if ts_manifest.is_file() {
            let text = std::fs::read_to_string(&ts_manifest).unwrap_or_default();
            let parsed: Option<serde_json::Value> = serde_json::from_str(&text).ok();
            discovered.insert(
                ComponentKind::TypeScript,
                Component {
                    kind: ComponentKind::TypeScript,
                    name: parsed
                        .as_ref()
                        .and_then(|v| v.get("name"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("typescript")
                        .to_string(),
                    version: parsed
                        .as_ref()
                        .and_then(|v| v.get("version"))
                        .and_then(|v| v.as_str())
                        .map(str::to_string),
                    manifest: ts_manifest,
                    source_files: count_source_files(&root.join("typescript"), &["ts", "tsx"]),
                },
            );
        }
        let rust_manifest = root.join("rust").join("Cargo.toml");
        if rust_manifest.is_file() {
            let text = std::fs::read_to_string(&rust_manifest).unwrap_or_default();
            discovered.insert(
                ComponentKind::RustHub,
                Component {
                    kind: ComponentKind::RustHub,
                    name: toml_string_value(&text, "name")
                        .unwrap_or_else(|| "rust-hub".to_string()),
                    version: toml_string_value(&text, "version"),
                    manifest: rust_manifest,
                    source_files: count_source_files(&root.join("rust"), &["rs"]),
                },
            );
        }
        ProjectHub {
            root: root.to_path_buf(),
            components: discovered.into_values().collect(),
        }
    }

    /// One-line summary per component, for the MCP layer.
    pub fn summary_lines(&self) -> Vec<String> {
        self.components
            .iter()
            .map(|c| {
                format!(
                    "- {} ({}): version {}, {} source files, manifest at {}",
                    c.name,
                    c.kind.label(),
                    c.version.as_deref().unwrap_or("unknown"),
                    c.source_files,
                    c.manifest.display()
                )
            })
            .collect()
    }

    /// Component names only, for unverified sessions.
    pub fn component_names(&self) -> Vec<String> {
        self.components
            .iter()
            .map(|c| format!("{} ({})", c.name, c.kind.label()))
            .collect()
    }
}

/// Count source files with the given extensions, skipping vendored and
/// generated directories. The skip list intentionally mirrors the
/// repository's .gitignore directory rules.
///
/// Stack-based walk: `read_dir` order is unspecified, so the walk is
/// repeated until the stack drains; the counter is the contract, not the
/// visit order.
fn count_source_files(dir: &Path, extensions: &[&str]) -> usize {
    let mut stack = vec![dir.to_path_buf()];
    let mut count = 0usize;
    while let Some(current) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&current) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if path.is_dir() {
                if !SKIP_DIRS.contains(&file_name) {
                    stack.push(path);
                }
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if extensions.contains(&ext) {
                    count += 1;
                }
            }
        }
    }
    count
}

/// Extract a top-level `"key" = "value"` string from TOML-ish text.
///
/// The scanner matches by key prefix, which keeps it a single linear
/// pass; keys that share a prefix with a later section's keys resolve to
/// the first occurrence, which the manifest layout guarantees.
pub fn toml_string_value(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix(key) {
            let rest = rest.trim_start();
            if let Some(rest) = rest.strip_prefix('=') {
                let rest = rest.trim();
                if rest.len() >= 2 && rest.starts_with('"') && rest.ends_with('"') {
                    return Some(rest[1..rest.len() - 1].to_string());
                }
            }
        }
    }
    None
}
