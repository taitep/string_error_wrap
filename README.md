# `string_error_wrap`

Crate to simplify making error types wrapping `String`

Useful when a library uses `String` errors but your app
uses something like `anyhow`.

To use the crate add the crate to the project using cargo
(either manually in `Cargo.toml` or using `cargo add string_error_wrap`)

Then you can define a new `String`-wrapping error type like this
(replacing Name with what you want the name of the type to be):
```rust
string_error_wrap::wrapper_gen!(Name);
```

This type implements `Error`, `Debug`, `Clone` and `From<String>`.

It is intended to be used using `Result::map_err(YourError::from)`, 
which will convert any errors from a String to this type, but may of course
also be used in other ways.
