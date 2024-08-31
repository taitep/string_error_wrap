//! # `string_error_wrap`
//! Crate for easily creating error types wrapping String.
//!
//! This can be extremely useful for dealing with libraries
//! using String as an error type in some functions when you
//! use something like anyhow.
//!
//! # Usage
//! ```
//! // generates error type
//! string_error_wrap::wrapper!(SomeError);
//!
//! fn main() -> Result<(), SomeError> {
//!     // other_function returns a Result<T, String>
//!     // but that can be converted into Result<T, SomeError>
//!     // allowing the use of ?
//!     other_function().map_err(SomeError::from)?;
//! }
//! ```

/// Generates a String-wrapping error type
///
/// # Usage
/// ```
/// string_error_wrap::wrapper_gen!(NameOfErrorTypeYouWant);
/// ```
/// will define a struct that implements `Debug`, `Error` and `Clone`
/// as well as `From<String>`.
///
/// All instances of this being used results in a struct that
/// behaves exactly the same, but with the specified name.
///
/// The Display implementation will result in something like the following:
/// `ErrorIdentifier: message` where message is the contained string.
#[macro_export]
macro_rules! wrapper_gen {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        struct $name(String);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{}: {}", stringify!($name), self.0))
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl std::error::Error for $name {}
    };
}
