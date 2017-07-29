//! A module defining the `cl_program` related types, such as the high-level `Program` type.

use wrapper::ffi;
use wrapper::types::context::Context;
use wrapper::information::InformationResult;
use errors::*;
use std::ptr;

enumz!(
    BuildStatus,
    ffi::cl_build_status,
    "cl_build_status",
    None => [ffi::CL_BUILD_NONE, "CL_BUILD_NONE"],
    Error => [ffi::CL_BUILD_ERROR, "CL_BUILD_ERROR"],
    Success => [ffi::CL_BUILD_SUCCESS, "CL_BUILD_SUCCESS"],
    InProgress => [ffi::CL_BUILD_IN_PROGRESS, "CL_BUILD_IN_PROGRESS"]
);

enumz!(
    BinaryType,
    ffi::cl_program_binary_type,
    "cl_program_binary_type",
    None => [ffi::CL_PROGRAM_BINARY_TYPE_NONE, "CL_PROGRAM_BINARY_TYPE_NONE"],
    CompiledObject => [ffi::CL_PROGRAM_BINARY_TYPE_COMPILED_OBJECT, "CL_PROGRAM_BINARY_TYPE_COMPILED_OBJECT"],
    Library => [ffi::CL_PROGRAM_BINARY_TYPE_LIBRARY, "CL_PROGRAM_BINARY_TYPE_LIBRARY"],
    Executable => [ffi::CL_PROGRAM_BINARY_TYPE_EXECUTABLE, "CL_PROGRAM_BINARY_TYPE_LIBRARY"]
);

pub mod information {
    //! A module containing the information marker types for programs.

    use wrapper::ffi;
    use wrapper::information::*;
    use wrapper::types::context;
    use wrapper::types::device;

    /// A trait implemented by marker types for retrieving information through `clGetProgramInfo`.
    pub trait ProgramInformation: Information<ffi::cl_program_info> { }

    macro_rules! test_fun {
        ($test_fun: ident, $type: ident, $info_fun: ident) => {
            #[test]
            fn $test_fun() {
                let context = context::Context::default().unwrap();
                let program = super::Builder::create_with_sources(
                    Some("__kernel void addFFT(__global float * filter, __global float * temp, float coeff) {
                        int ind = get_global_id(0);
                        filter[2 * ind] += temp[ind] * coeff;
                    }"),
                    &context
                ).unwrap();
                let program = program.build().unwrap();
                let _ = program.$info_fun::<$type>();
            }
        };
    }

    macro_rules! info_impl {
        ($type: ident, $result: ty, $id: expr, $id_name: expr, $test_fun: ident) => {
            general_info_impl!(ProgramInformation, ffi::cl_program_info, $type, $result, $id, $id_name);

            test_fun!($test_fun, $type, get_info);
        };
    }

    info_impl!(ReferenceCount, ffi::cl_uint, ffi::CL_PROGRAM_REFERENCE_COUNT, "CL_PROGRAM_REFERENCE_COUNT", test_reference_count);
    info_impl!(Context, context::Context, ffi::CL_PROGRAM_CONTEXT, "CL_PROGRAM_CONTEXT", test_context);
    info_impl!(NumDevices, ffi::cl_uint, ffi::CL_PROGRAM_NUM_DEVICES, "CL_PROGRAM_NUM_DEVICES", test_num_devices);
    info_impl!(Devices, Vec<device::Device>, ffi::CL_PROGRAM_DEVICES, "CL_PROGRAM_DEVICES", test_devices);
    info_impl!(Source, String, ffi::CL_PROGRAM_SOURCE, "CL_PROGRAM_SOURCE", test_source);
    info_impl!(BinarySizes, Vec<usize>, ffi::CL_PROGRAM_BINARY_SIZES, "CL_PROGRAM_BINARY_SIZES", test_binary_sizes);
    // ProgramBinaries
    info_impl!(NumKernels, usize, ffi::CL_PROGRAM_NUM_KERNELS, "CL_PROGRAM_NUM_KERNELS", test_num_kernels);
    info_impl!(KernelNames, String, ffi::CL_PROGRAM_KERNEL_NAMES, "CL_PROGRAM_KERNEL_NAMES", test_kernel_names);

    /// A trait implemented by marker types for retrieving information through `clGetProgramBuildInfo`.
    pub trait BuildInformation: Information<ffi::cl_program_build_info> { }

    macro_rules! build_info_impl {
        ($type: ident, $result: ty, $id: expr, $id_name: expr, $test_fun: ident) => {
            general_info_impl!(BuildInformation, ffi::cl_program_build_info, $type, $result, $id, $id_name);

            test_fun!($test_fun, $type, get_build_info);
        };
    }

    build_info_impl!(BuildStatus, super::BuildStatus, ffi::CL_PROGRAM_BUILD_STATUS, "CL_PROGRAM_BUILD_STATUS", test_build_status);
    build_info_impl!(BuildOptions, String, ffi::CL_PROGRAM_BUILD_OPTIONS, "CL_PROGRAM_BUILD_OPTIONS", test_build_options);
    build_info_impl!(BuildLog, String, ffi::CL_PROGRAM_BUILD_LOG, "CL_PROGRAM_BUILD_LOG", test_build_log);
    build_info_impl!(BinaryType, super::BinaryType, ffi::CL_PROGRAM_BINARY_TYPE, "CL_PROGRAM_BINARY_TYPE", test_binary_type);
}

/// `Program` is a high-level type which maps to the low-level `cl_program` OpenCL type.
/// An object of type `Program` acts as a ref-counted reference to an OpenCL program.
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
    /// Start creating a program from an iterator of source strings for a given context.
    ///
    /// # Errors
    /// * `SourceErrorKind::NoSources` if `sources` does not produce any elements.
    ///
    /// # Panics
    /// Panics if the host or a device fails to allocate resources.
    pub fn create_with_sources<'a, I>(sources: I, context: &Context) -> source_error::Result<Builder>
        where I: IntoIterator<Item = &'a str>
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

    /// Build a program (i.e. compile + link) with specified `options`.
    ///
    /// # Examples
    /// ```rust
    /// # extern crate gprust;
    /// use gprust::{Context, program};
    ///
    /// # fn main_() -> Result<(), &'static str> {
    /// let context = Context::default().ok_or("no default context")?;
    /// let program = program::Builder::create_with_sources(
    ///     Some("__kernel void my_kernel(__global float * buffer) {
    ///         buffer[get_global_id(0)] *= 2;
    ///     }"),
    ///     &context
    /// ).expect("I did provide a source");
    /// if let Ok(program) = program.build_with_options("") {
    ///     /* do something with `program` */
    /// }
    /// # Ok(())
    /// # }
    /// # fn main() { main_().unwrap(); }
    /// ```
    ///
    /// # Errors
    /// * `BuildErrorKind::CompilerNotAvailable` if one of the devices does not have an available compiler.
    /// * `BuildErrorKind::InvalidBuildOptions` if the options string contained invalid options.
    /// * `BuildErrorKind::BuildProgramFailure(log)` if the build failed. The build log can be get
    /// through `log` or `get_build_info::<program::information::BuildLog>`.
    ///
    /// # Panics
    /// Panics if the host or a device fails to allocate resources.
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
            return Err(
                BuildErrorKind::BuildProgramFailure(
                    self.program.get_build_info::<information::BuildLog>()
                ).into()
            );
        }

        let result = catch_ffi(err).map(move |()| self.program );
        Ok(expect!(result, ffi::CL_OUT_OF_HOST_MEMORY, ffi::CL_OUT_OF_RESOURCES))
    }

    /// Call `build_with_options` with an empty options string.
    ///
    /// # Errors
    /// * `BuildErrorKind::CompilerNotAvailable` if one of the devices does not have an available compiler.
    /// * `BuildErrorKind::InvalidBuildOptions` if the options string contained invalid options.
    /// * `BuildErrorKind::BuildProgramFailure(log)` if the build failed. The build log can be get
    /// through `log` or `get_build_info::<program::information::BuildLog>`.
    ///
    /// # Panics
    /// Panics if the host or a device fails to allocate resources.
    pub fn build(self) -> build_error::Result<Program> {
        self.build_with_options("")
    }
}

impl Program {
    /// Query an information to the program. `T` should be a marker type from the `information`
    /// module implementing `ProgramInformation`.
    ///
    /// # Panics
    /// Panic if the host or a device fails to allocate resources, or if an invalid information
    /// param is passed (should only happen when a user incorrectly implements
    /// `ProgramInformation` on their own or if the information is not supported on the program
    /// and cargo features have not been set correctly, otherwise it is a bug).
    pub fn get_info<T: information::ProgramInformation>(&self) -> T::Result {
        let result = unsafe {
            InformationResult::get_info(|size, value, ret_size| {
                ffi::clGetProgramInfo(
                    self.program,
                    T::id(),
                    size,
                    value as _,
                    ret_size
                )
            })
        };

        expect!(
            result,
            ffi::CL_OUT_OF_RESOURCES,
            ffi::CL_OUT_OF_HOST_MEMORY,
            ffi::CL_INVALID_VALUE
        )
    }

    /// Query a build information to the program. `T` should be a marker type from the `information`
    /// module implementing `BuildInformation`.
    ///
    /// # Panics
    /// Panic if the host or a device fails to allocate resources, or if an invalid information
    /// param is passed (should only happen when a user incorrectly implements
    /// `BuildInformation` on their own or if the information is not supported on the program
    /// and cargo features have not been set correctly, otherwise it is a bug).
    pub fn get_build_info<T: information::BuildInformation>(&self) -> T::Result {
        let result = unsafe {
            InformationResult::get_info(|size, value, ret_size| {
                ffi::clGetProgramBuildInfo(
                    self.program,
                    self.get_info::<information::Devices>().first().unwrap().underlying(),
                    T::id(),
                    size,
                    value as _,
                    ret_size
                )
            })
        };

        expect!(
            result,
            ffi::CL_OUT_OF_RESOURCES,
            ffi::CL_OUT_OF_HOST_MEMORY,
            ffi::CL_INVALID_VALUE
        )
    }
}

impl Clone for Program {
    fn clone(&self) -> Self {
        catch_ffi(unsafe { ffi::clRetainProgram(self.program) }).unwrap();

        Program {
            program: self.program,
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        catch_ffi(unsafe { ffi::clReleaseProgram(self.program) }).unwrap();
    }
}
