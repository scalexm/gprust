//! A module for wrapping OpenCL functions and types.

macro_rules! expect {
    ($result: expr, $($error: expr),*) => {
        match $result {
            Ok(value) => value,
            $(
            Err(err @ Error(ErrorKind::RawError($error), _)) => panic!("{:?}", err),
            )*
            Err(err) => panic!("unexpected error, this is a bug: {:?}", err),
        }
    };
}

#[allow(dead_code, non_camel_case_types, non_upper_case_globals)] pub mod ffi;
pub mod types;
mod information;
