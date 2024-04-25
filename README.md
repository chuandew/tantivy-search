# Tantivy Search

[![coverage](https://git.moqi.ai/mqdb/tantivy-search/badges/tantivy_0.21.1/coverage.svg?job=CodeCoverage)]()
[![pipeline status](https://git.moqi.ai/mqdb/tantivy-search/badges/tantivy_0.21.1/pipeline.svg)](https://git.moqi.ai/mqdb/tantivy-search/-/pipelines?page=1&scope=all&ref=tantivy_0.21.1)
[![codecov](https://codecov.io/gh/MochiXu/tantivy-search/graph/badge.svg?token=DRUQZXPYRP)](https://codecov.io/gh/MochiXu/tantivy-search)



This library is designed to integrate [tantivy](https://github.com/quickwit-oss/tantivy/) into [ClickHouse](https://github.com/ClickHouse/ClickHouse) and [MyScale](https://git.moqi.ai/mqdb/ClickHouse/).

## Development

All FFI (Foreign Function Interface) functions are exposed in [lib.rs](./src/lib.rs). Developers need to regenerate the header file after making any changes to the relevant code:

```bash
cbindgen . -o include/tantivy_search_cbindgen.h --config cbindgen.toml
cxxbridge src/lib.rs --header > include/tantivy_search_cxx.h
```

If developers do not add, delete, or modify the names of FFI functions, there is no need to execute the above command.

If developers do not have cbindgen or cxxbridge, install like this:

```bash
cargo install cbindgen
cargo install cxxbridge-cmd
```

## Work in progress

- [x] Added unit tests in C++ to test Rust FFI functions.
- [x] Add additional unit tests within Rust FFI functions.
- [ ] Refactor the `tantivy_search` code using object-oriented principles.

## How to build?

You can use `cargo` to build this library, use this command:

```bash
cargo build --release
```

If you need to test FFI function in C++, run:

```bash
mkdir build
cd build && cmake ..
make -j
```

You can use `vscode` or other compilers to make the build process more elegant.

## How to test?

Test in Rust:

```bash
cargo test
```

Here is an example to run unit test in C++:

```bash
cd build/tests/unit_test
./unit_test
```
