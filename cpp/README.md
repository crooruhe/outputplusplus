# outputplusplus (native core)

The C++ implementation: the shared string contracts (same results as
the Python, TypeScript, Rust, and Go implementations), the C bridge the
other stacks can link, and the repo-wide parity harness.

- Project overview: [`../README.md`](../README.md)
- Setup and usage: [`../USAGE.md`](../USAGE.md)

## Build and test

```bash
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build -j
./build/opp_tests
./build/opp-cpp shout hello
./build/opp-parity   # aggregates the stack parity verifiers
```

Read [../AGENTS.md](../AGENTS.md) before modifying the code, especially
`src/outputplusplus/compat.cpp` and `src/outputplusplus/bridge.cpp`.
