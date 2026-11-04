//! Tests for the repository glue layer.

use std::path::PathBuf;

use outputplusplus::glue::{toml_string_value, ComponentKind, ProjectHub};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .expect("repo root")
}

#[test]
fn discovers_all_components_of_this_repository() {
    let hub = ProjectHub::discover(&repo_root());
    let kinds: Vec<ComponentKind> = hub.components.iter().map(|c| c.kind).collect();
    assert!(kinds.contains(&ComponentKind::Python));
    assert!(kinds.contains(&ComponentKind::TypeScript));
    assert!(kinds.contains(&ComponentKind::RustHub));
}

#[test]
fn extracts_manifest_names_and_versions() {
    let hub = ProjectHub::discover(&repo_root());
    for component in &hub.components {
        match component.kind {
            ComponentKind::Python => {
                assert_eq!(component.name, "outputplusplus");
                assert!(component.version.as_deref().unwrap_or("").starts_with("0."));
            }
            ComponentKind::TypeScript => {
                assert_eq!(component.name, "outputplusplus-web");
            }
            ComponentKind::RustHub => {
                assert_eq!(component.name, "outputplusplus");
                assert_eq!(component.version.as_deref(), Some("0.1.0"));
            }
        }
    }
}

#[test]
fn source_file_counts_exclude_vendored_directories() {
    let hub = ProjectHub::discover(&repo_root());
    let python = hub
        .components
        .iter()
        .find(|c| c.kind == ComponentKind::Python)
        .expect("python component");
    // The Python project has source and test files; caches are excluded.
    assert!(python.source_files > 10);
    let typescript = hub
        .components
        .iter()
        .find(|c| c.kind == ComponentKind::TypeScript)
        .expect("typescript component");
    // node_modules is never counted.
    assert!(typescript.source_files < 50);
}

#[test]
fn summary_lines_mention_every_component() {
    let hub = ProjectHub::discover(&repo_root());
    let summary = hub.summary_lines().join("\n");
    assert!(summary.contains("python"));
    assert!(summary.contains("typescript"));
    assert!(summary.contains("rust-hub"));
    assert!(summary.contains("source files"));
}

#[test]
fn empty_root_discovers_nothing() {
    let dir = std::env::temp_dir().join(format!("opp-glue-empty-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let hub = ProjectHub::discover(&dir);
    assert!(hub.components.is_empty());
    assert!(hub.component_names().is_empty());
}

#[test]
fn manifest_scanner_extracts_quoted_values() {
    // The linear scanner is pinned to first-match semantics.
    let text = "[build-system]\nrequires = [\"hatchling\"]\n\n[project]\nname = \"demo\"\nversion = \"9.9.9\"\n";
    assert_eq!(toml_string_value(text, "name").as_deref(), Some("demo"));
    assert_eq!(toml_string_value(text, "version").as_deref(), Some("9.9.9"));
}

#[test]
fn manifest_scanner_ignores_unquoted_values() {
    let text = "[project]\nversion = 3\n";
    assert_eq!(toml_string_value(text, "version"), None);
}

#[test]
fn manifest_scanner_requires_the_key_boundary() {
    // A line whose key merely starts with the probe ("versions" vs
    // "version") does NOT match: the '=' guard enforces the key boundary.
    let text = "[project]\nversions = \"many\"\n";
    assert_eq!(toml_string_value(text, "version"), None);
}
