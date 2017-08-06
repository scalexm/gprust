//! A module for wrapping OpenCL functions and types.

/// A macro for panicking on some expected errors.
macro_rules! expect {
    ($result: expr, $($error: expr),*) => {
        match $result {
            Ok(value) => value,
            $(
            Err(err @ RawError($error)) => panic!("{}", err),
            )*
            Err(err) => panic!("unexpected error, this is a bug: {}", err),
        }
    };
}

#[allow(dead_code, non_camel_case_types, non_upper_case_globals)] pub mod ffi;
pub mod types;
mod information;
