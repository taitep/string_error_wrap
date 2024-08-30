#[macro_export]
macro_rules! wrapper {
    ($name:ident) => {
        #[derive(Debug, thiserror::Error, Clone)]
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
    };
}
