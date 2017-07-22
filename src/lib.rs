#[macro_use] extern crate error_chain;

mod wrapper;
mod errors;

pub use errors::{Error, ErrorKind, ResultExt, Result};
pub use wrapper::types::platform::{self, Platform};
pub use wrapper::types::device::{self, Device};
