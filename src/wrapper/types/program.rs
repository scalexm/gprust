//! A module defining the `cl_program` related types, such as the high-level `Program` type.

use wrapper::ffi;
use wrapper::types::context::Context;
use errors::*;
use std::ptr;

/// `Program` is a high-level type which maps to the low-level `cl_program` OpenCL type.
/// An object of type `Program` acts as a reference to an OpenCL program. Hence, cloning
/// a program is a shallow copy.
/// The reference counter of a program is incremented on cloning and decrementing on dropping.
#[derive(PartialEq, Eq)]
pub struct Program {
    program: ffi::cl_program,
}

/// A builder struct for `Program` type which compiles OpenCL programs.
pub struct Builder {
    program: Program,
}

mod source_error {
    error_chain! {
        types {
            SourceError, SourceErrorKind, ResultExt, Result;
        }

        errors {
            /// No sources were provided.
            NoSources {
                description("no sources were provided")
            }
        }
    }
}

mod build_error {
    error_chain! {
        types {
            BuildError, BuildErrorKind, ResultExt, Result;
        }

        errors {
            /// One of the devices does not have an available compiler.
            CompilerNotAvailable {
                description("a compiler is not available for one of the devices")
            }

            /// The options provided are invalid.
            InvalidBuildOptions {
                description("invalid build options")
            }

            /// Build failed, see build log.
            BuildProgramFailure(log: String) {
                description("build failed")
                display("build log: {}", log)
            }
        }
    }
}

pub use self::source_error::{SourceError, SourceErrorKind};
pub use self::build_error::{BuildError, BuildErrorKind};

impl Builder {
    /// Start creating a program from an iterator of source strings and a context.
    ///
    /// # Errors
    /// * `SourceErrorKind::NoSources` if `sources` does not produce any elements.
    ///
    /// # Panics
    /// Panics if the host or a device fails to allocate resources.
    pub fn create_with_sources<'a, I>(sources: I, context: &Context) -> source_error::Result<Builder>
        where I: Iterator<Item = &'a str>
    {
        let (mut sources, lengths): (Vec<_>, Vec<_>) =
            sources.into_iter()
                   .map(|src| (src.as_ptr() as *const i8, src.len()))
                   .unzip();
        
        if sources.len() == 0 {
            return Err(SourceErrorKind::NoSources.into());
        }
        
        let mut error = 0;
        let program = unsafe {
            ffi::clCreateProgramWithSource(
                context.underlying(),
                sources.len() as _,
                sources.as_mut_ptr(),
                lengths.as_ptr(),
                &mut error
            )
        };

        let result = catch_ffi(error).map(|()| Builder { program: Program { program } });
        Ok(expect!(result, ffi::CL_OUT_OF_RESOURCES, ffi::CL_OUT_OF_HOST_MEMORY))
    }

    pub fn build_with_options(self, options: &str) -> build_error::Result<Program> {
        use std::ffi::CString;

        let err = unsafe {
            ffi::clBuildProgram(
                self.program.program,
                0,
                ptr::null(),
                CString::new(options).expect("should be valid utf8 here")
                                     .as_ptr(),
                None,
                ptr::null_mut()
            )
        };

        if err == ffi::CL_INVALID_BUILD_OPTIONS {
            return Err(BuildErrorKind::InvalidBuildOptions.into());
        } else if err == ffi::CL_COMPILER_NOT_AVAILABLE {
            return Err(BuildErrorKind::CompilerNotAvailable.into());
        } else if err == ffi::CL_BUILD_PROGRAM_FAILURE {
            return Err(BuildErrorKind::BuildProgramFailure(String::new()).into());
        }

        let result = catch_ffi(err).map(move |()| self.program );
        Ok(expect!(result, ffi::CL_OUT_OF_HOST_MEMORY, ffi::CL_OUT_OF_RESOURCES))
    }

    pub fn build(self) -> build_error::Result<Program> {
        self.build_with_options("")
    }
}

impl Clone for Program {
    fn clone(&self) -> Self {
        catch_ffi(unsafe { ffi::clRetainProgram(self.program) });

        Program {
            program: self.program,
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        catch_ffi(unsafe { ffi::clReleaseProgram(self.program) });
    }
}
