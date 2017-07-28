//! `gprust` crate.

#![warn(missing_docs)]

#[macro_use] extern crate error_chain;

mod wrapper;
mod errors;
mod array;

pub use errors::{Error, ErrorKind, ResultExt, Result};
pub use wrapper::types::platform::{self, Platform};
pub use wrapper::types::device::{self, Device};
pub use wrapper::types::context::{self, Context};
pub use wrapper::types::command_queue::{self, CommandQueue};
pub use wrapper::types::program::{self, Program};
