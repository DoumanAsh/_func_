# func_

![Rust](https://github.com/DoumanAsh/func_/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/func_.svg)](https://crates.io/crates/func_)
[![Documentation](https://docs.rs/func_/badge.svg)](https://docs.rs/crate/func_/)

Proc macro to insert function name within body of function because Rust is incapable of doing simple things

Once [type_name](https://github.com/rust-lang/rust/issues/63084) is stable in `const` context this macro can be replaced properly with simple function call on function type.

## Usage

```rust
use func_::_func_;

#[_func_]
fn my_func() {
    assert_eq!(__func__, "my_func");
}
```
