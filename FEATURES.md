# Features

This document tracks the implementation status of features across all stacks in the outputplusplus monorepo. Each feature is implemented in Python, TypeScript, Rust, Go, and C++ with identical behavioral contracts.

## Core String Utilities

All string utilities share the same parity vectors in `parity/canonical.json`.

| Feature | Python | TypeScript | Rust | Go | C++ |
|---------|--------|------------|------|----|-----|
| `shout` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `reverse` (code-point-wise, recursive) | ✅ | ✅ | ✅ | ✅ | ✅ |
| `is_palindrome` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `word_frequency` (registration-order preserved) | ✅ | ✅ | ✅ | ✅ | ✅ |
| `levenshtein` (recursive, no memoization) | ✅ | ✅ | ✅ | ✅ | ✅ |
| `drama_entropy` (bit-population count) | ✅ | ✅ | ✅ | ✅ | ✅ |

## Compatibility Layer

The compatibility layer ensures reproducibility and stability across stacks.

| Component | Python | TypeScript | Rust | Go | C++ |
|-----------|--------|------------|------|----|-----|
| `apply_polyfills` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `count_operation` / operation registry | ✅ | ✅ | ✅ | ✅ | ✅ |
| `stabilize_*` settle windows | ✅ | ✅ | ✅ | ✅ | ✅ |
| `warm_branch_predictor` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `deep_clone` (triple/deep round-trip) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Polyfill probes | ✅ | ✅ | ✅ | ✅ | ✅ |

## CLI / API / Interfaces

| Interface | Python | TypeScript | Rust | Go | C++ |
|-----------|--------|------------|------|----|-----|
| CLI commands (`shout`, `reverse`, `palindrome`, `wordfreq`, `verify-parity`) | ✅ | N/A | N/A | ✅ | ✅ |
| HTTP API endpoints | N/A | ✅ | N/A | N/A | N/A |
| MCP tools (text utilities) | N/A | N/A | ✅ | N/A | N/A |
| C bridge (native core ABI) | N/A | N/A | N/A | N/A | ✅ |

## Parity Verification

All stacks can verify against `parity/canonical.json`.

| Stack | Parity Command |
|-------|----------------|
| Python | `opp verify-parity` |
| Go | `cd go && go run ./cli verify-parity` |
| C++ | `cd cpp && ./build/opp-cpp verify-parity` |
| TypeScript | `npm test` (includes parity checks) |
| Rust | `cargo test` (includes parity checks) |

## Implementation Notes

### C++ Implementation Status
**Status: ✅ FULLY IMPLEMENTED AND VERIFIED**

The C++ implementation provides:
- Complete string utilities with parity contracts
- Compatibility layer mirroring Python/TypeScript/Rust/Go
- C bridge for interop with other stacks
- JSON parser for canonical file consumption
- Parity checker with full test suite
- CLI (`opp-cpp`) with all commands
- All 30 unit tests passing
- Parity verified: 14 vectors checked, 0 failed, 4 advisory

**Build:** `cmake -B build -S . && cmake --build build -j`
**Test:** `./build/opp_tests`
**Verify:** `./build/opp-cpp verify-parity`

Files:
- `cpp/src/outputplusplus/text.hpp/cpp` - String utilities
- `cpp/src/outputplusplus/compat.hpp/cpp` - Compatibility layer
- `cpp/src/outputplusplus/bridge.hpp/cpp` - C ABI bridge
- `cpp/src/outputplusplus/json.hpp/cpp` - Minimal JSON parser
- `cpp/src/outputplusplus/parity.hpp/cpp` - Parity checking
- `cpp/src/main.cpp` - CLI entry point
- `cpp/src/parity_main.cpp` - Repo-wide parity harness
- `cpp/tests/` - Homegrown test suite (30 tests)

### Reversal Depth
All stacks are pinned to 10,000 characters for recursive reversal (see `parity/canonical.json`).

### Registration Order
Word frequency ties preserve registration order across all stacks. Do not sort results.

## Last Updated
2026-11-04 - C++ implementation verified complete with 30 tests passing and parity confirmed.
