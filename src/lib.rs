//! `gprust` crate.

#![warn(missing_docs)]

//#[macro_use] extern crate lazy_static;
extern crate futures;

pub use futures::{Future, Async};

mod wrapper;
mod errors;
//#[allow(dead_code)] mod array;

pub use wrapper::types::platform::{self, Platform};
pub use wrapper::types::device::{self, Device};
pub use wrapper::types::context::{self, Context};
pub use wrapper::types::command_queue::{self, CommandQueue};
pub use wrapper::types::program::{self, Program};
pub use wrapper::types::mem::{self, Buffer};
pub use wrapper::types::kernel::{self, Kernel};
