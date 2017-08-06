//! A module defining the `cl_program` related types, such as the high-level `Program` type.

use wrapper::ffi;
use wrapper::types::context::Context;
use wrapper::types::kernel::Kernel;
use wrapper::information::InformationResult;
use errors::*;
use std::ptr;
use futures::{Poll, Future, Async};
use std::ffi::CString;
use std::fmt;

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
                use futures::Future;

                let context = context::Context::default().unwrap();
                let program = super::Builder::create_with_sources(
                    Some("__kernel void addFFT(__global float * filter, __global float * temp, float coeff) {
                        int ind = get_global_id(0);
                        filter[2 * ind] += temp[ind] * coeff;
                    }"),
                    &context
                ).unwrap();
                let program = program.build().wait().unwrap();
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

unsafe impl Send for Program { }
unsafe impl Sync for Program { }

/// A builder struct for `Program` type which compiles OpenCL programs.
#[derive(PartialEq, Eq)]
pub struct Builder {
    program: Program,
}

unsafe impl Send for Builder { }
unsafe impl Sync for Builder { }

/// An error returned by `Builder::create_with_sources`.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum SourceError {
    /// No sources were provided.
    NoSources,
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SourceError::NoSources => write!(f, "no sources were provided"),
        }
    }
}

/// An error returned by `FutureBuild`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BuildError {
    /// One of the devices does not have an available compiler.
    CompilerNotAvailable,

    /// The options provided are invalid.
    InvalidBuildOptions,

    /// Build failed, see build log.
    BuildFailed(String),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BuildError::CompilerNotAvailable
                => write!(f, "a compiler was not available for one of the devices"),
            BuildError::InvalidBuildOptions
                => write!(f, "invalid build options"),
            BuildError::BuildFailed(ref log)
                => write!(f, "build failed, log:\n{}", log),
        }
    }
}

/// A type containing the future result of a build.
pub struct FutureBuild {
    program: Result<Program, BuildError>,
}

impl Future for FutureBuild {
    type Item = Program;
    type Error = BuildError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use std::mem;

       if let Ok(ref program) = self.program {
            match program.get_build_info::<information::BuildStatus>() {
                BuildStatus::InProgress => return Ok(Async::NotReady),
                BuildStatus::Error =>
                    return Err(
                        BuildError::BuildFailed(
                            program.get_build_info::<information::BuildLog>()
                        )
                    ),
                _ => (),
            };
        }

        let program = mem::replace(
            &mut self.program,
            Err(BuildError::CompilerNotAvailable)
        );

        program.map(|program| Async::Ready(program))
    }
}

extern "C" fn build_callback(program: ffi::cl_program, _: *mut ::std::os::raw::c_void) {
    catch_ffi(unsafe { ffi::clReleaseProgram(program) }).unwrap();
}

impl Builder {
    /// Start creating a program from an iterator of source strings for a given context.
    ///
    /// # Errors
    /// * `SourceError::NoSources` if `sources` does not produce any elements.
    ///
    /// # Panics
    /// Panics if the host or a device fails to allocate resources.
    pub fn create_with_sources<'a, I>(sources: I, context: &Context) -> Result<Builder, SourceError>
        where I: IntoIterator<Item = &'a str>
    {
        let (mut sources, lengths): (Vec<_>, Vec<_>) =
            sources.into_iter()
                   .map(|src| (src.as_ptr() as *const i8, src.len()))
                   .unzip();
        
        if sources.len() == 0 {
            return Err(SourceError::NoSources);
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

    /// Build a program (i.e. compile + link) with specified `options`. The return value is a
    /// future containing the program result.
    ///
    /// # Examples
    /// ```rust
    /// # extern crate gprust;
    /// use gprust::{Context, program, Future};
    ///
    /// # fn main_() -> Result<(), &'static str> {
    /// let context = Context::default().ok_or("no default context")?;
    /// let program = program::Builder::create_with_sources(
    ///     Some("__kernel void my_kernel(__global float * buffer) {
    ///         buffer[get_global_id(0)] *= 2;
    ///     }"),
    ///     &context
    /// ).expect("I did provide a source");
    /// if let Ok(program) = program.build_with_options("-Werror").wait() {
    ///     /* do something with `program` */
    /// }
    /// # Ok(())
    /// # }
    /// # fn main() { main_().unwrap(); }
    /// ```
    ///
    /// # Errors
    /// Errors that `FutureBuild` can return:
    /// * `BuildError::CompilerNotAvailable` if one of the devices does not have an available compiler.
    /// * `BuildError::InvalidBuildOptions` if the options string contained invalid options.
    /// * `BuildError::BuildFailed(log)` if the build failed. The build log can be get through
    /// `log` or `get_build_info::<program::information::BuildLog>`.
    ///
    /// # Panics
    /// Panics if the host or a device fails to allocate resources.
    pub fn build_with_options(self, options: &str) -> FutureBuild {
        catch_ffi(unsafe { ffi::clRetainProgram(self.program.program) }).unwrap();
        let err = unsafe {
            ffi::clBuildProgram(
                self.program.program,
                0,
                ptr::null(),
                CString::new(options).expect("should be a valid string").as_ptr(),
                Some(build_callback),
                ptr::null_mut()
            )
        };

        let result = if err == ffi::CL_INVALID_BUILD_OPTIONS {
            Err(BuildError::InvalidBuildOptions)
        } else if err == ffi::CL_COMPILER_NOT_AVAILABLE {
            Err(BuildError::CompilerNotAvailable)
        } else if err == ffi::CL_BUILD_PROGRAM_FAILURE {
            Err(
                BuildError::BuildFailed(
                    self.program.get_build_info::<information::BuildLog>()
                )
            )
        } else {
            Ok(self.program)
        };

        expect!(catch_ffi(err), ffi::CL_OUT_OF_HOST_MEMORY, ffi::CL_OUT_OF_RESOURCES);

        FutureBuild {
            program: result,
        }
    }

    /// Call `build_with_options` with an empty options string.
    ///
    /// # Errors
    /// Errors that `FutureBuild` can return:
    /// * `BuildErrorKind::CompilerNotAvailable` if one of the devices does not have an available compiler.
    /// * `BuildErrorKind::InvalidBuildOptions` if the options string contained invalid options.
    /// * `BuildErrorKind::BuildProgramFailure(log)` if the build failed. The build log can be get
    /// through `log` or `get_build_info::<program::information::BuildLog>`.
    ///
    /// # Panics
    /// Panics if the host or a device fails to allocate resources.
    pub fn build(self) -> FutureBuild {
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

    /// Return a list of kernel names the program contains.
    ///
    /// # Panics
    /// Same as `get_info`.
    pub fn kernel_names(&self) -> Vec<String> {
        self.get_info::<information::KernelNames>()
            .split(';')
            .map(|s| s.to_owned())
            .collect()
    }

    /// Create a kernel, defined in the program matching the name `kernel_name`.
    ///
    /// # Panics
    /// TO COMPLETE.
    pub fn create_kernel(&self, kernel_name: &str) -> Kernel {
        let mut err = 0;
        let kernel = unsafe {
            ffi::clCreateKernel(
                self.program,
                CString::new(kernel_name).expect("should be a valid string").as_ptr(),
                &mut err
            )
        };

        if err != 0 {
            panic!("error");
        }

        unsafe { Kernel::from_ffi(kernel, false) }
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
