# pyret-rs

> An implementation of the [Pyret programming language](https://www.pyret.org).

## Running

```console
git submodule update --init --recursive
cargo run test.arr
```

## TODO

- Priority
  - [ ] Finish the specs (see [`crates/lexer`](crates/lexer))
- Internal
  - [ ] Unit testing
- Language
  - [ ] Static and runtime type checking
  - [ ] Implement FFI/language bindings
- Expandability
  - [ ] Code coverage for Pyret
  - [ ] JIT compiler
