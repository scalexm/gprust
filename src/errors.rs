//! A module for errors boilerplate.

use wrapper::ffi;

error_chain! {
    errors {
        /// `RawError(err)` where `err` is an error code (a negative integer) returned by an
        /// OpenCL function. This error type is only used for panicking. Other modules will chain
        /// their own error types when necessary.
        RawError(err: ffi::cl_int) {
            description("raw error")
            display("OpenCL error {}: `{}`", err, error_string(*err))
        }
    }
}

/// Convert an error code into a string.
pub fn error_string(err: ffi::cl_int) -> &'static str {
    match err {
        ffi::CL_DEVICE_NOT_FOUND => "device not found",
        ffi::CL_DEVICE_NOT_AVAILABLE => "device not available",
        ffi::CL_COMPILER_NOT_AVAILABLE => "compiler not available",
        ffi::CL_MEM_OBJECT_ALLOCATION_FAILURE => "mem object allocation failure",
        ffi::CL_OUT_OF_RESOURCES => "out of resources",
        ffi::CL_OUT_OF_HOST_MEMORY => "out of host memory",
        ffi::CL_PROFILING_INFO_NOT_AVAILABLE => "profiling info not available",
        ffi::CL_MEM_COPY_OVERLAP => "mem copy overlap",
        ffi::CL_IMAGE_FORMAT_MISMATCH => "image format mismatch",
        ffi::CL_IMAGE_FORMAT_NOT_SUPPORTED => "image format not supported",
        ffi::CL_BUILD_PROGRAM_FAILURE => "build program failure",
        ffi::CL_MAP_FAILURE => "map failure",
        ffi::CL_INVALID_VALUE => "invalid value",
        ffi::CL_INVALID_DEVICE_TYPE => "invalid device type",
        ffi::CL_INVALID_PLATFORM => "invalid platform",
        ffi::CL_INVALID_DEVICE => "invalid device",
        ffi::CL_INVALID_CONTEXT => "invalid context",
        ffi::CL_INVALID_QUEUE_PROPERTIES => "invalid queue properties",
        ffi::CL_INVALID_COMMAND_QUEUE => "invalid command queue",
        ffi::CL_INVALID_HOST_PTR => "invalid host ptr",
        ffi::CL_INVALID_MEM_OBJECT => "invalid mem object",
        ffi::CL_INVALID_IMAGE_FORMAT_DESCRIPTOR => "invalid image format descriptor",
        ffi::CL_INVALID_IMAGE_SIZE => "invalid image size",
        ffi::CL_INVALID_SAMPLER => "invalid sampler",
        ffi::CL_INVALID_BINARY => "invalid binary",
        ffi::CL_INVALID_BUILD_OPTIONS => "invalid build options",
        ffi::CL_INVALID_PROGRAM => "invalid program",
        ffi::CL_INVALID_PROGRAM_EXECUTABLE => "invalid program executable",
        ffi::CL_INVALID_KERNEL_NAME => "invalid kernel name",
        ffi::CL_INVALID_KERNEL_DEFINITION => "invalid kernel definition",
        ffi::CL_INVALID_KERNEL => "invalid kernel",
        ffi::CL_INVALID_ARG_INDEX => "invalid arg index",
        ffi::CL_INVALID_ARG_VALUE => "invalid arg value",
        ffi::CL_INVALID_ARG_SIZE => "invalid arg size",
        ffi::CL_INVALID_KERNEL_ARGS => "invalid kernel args",
        ffi::CL_INVALID_WORK_DIMENSION => "invalid work dimension",
        ffi::CL_INVALID_WORK_GROUP_SIZE => "invalid work group size",
        ffi::CL_INVALID_WORK_ITEM_SIZE => "invalid work item size",
        ffi::CL_INVALID_GLOBAL_OFFSET => "invalid global offset",
        ffi::CL_INVALID_EVENT_WAIT_LIST => "invalid event wait list",
        ffi::CL_INVALID_EVENT => "invalid event",
        ffi::CL_INVALID_OPERATION => "invalid operation",
        ffi::CL_INVALID_GL_OBJECT => "invalid gl object",
        ffi::CL_INVALID_BUFFER_SIZE => "invalid buffer size",
        ffi::CL_INVALID_MIP_LEVEL => "invalid mip level",
        ffi::CL_INVALID_GLOBAL_WORK_SIZE => "invalid global work size",
        ffi::CL_COMPILE_PROGRAM_FAILURE => "compile program failure",
        ffi::CL_LINKER_NOT_AVAILABLE => "linker not available",
        ffi::CL_LINK_PROGRAM_FAILURE => "link program failure",
        ffi::CL_DEVICE_PARTITION_FAILED => "device partition failed",
        ffi::CL_KERNEL_ARG_INFO_NOT_AVAILABLE => "kernel arg info not available",
        ffi::CL_INVALID_PROPERTY => "invalid property",
        ffi::CL_INVALID_IMAGE_DESCRIPTOR => "invalid image descriptor",
        ffi::CL_INVALID_COMPILER_OPTIONS => "invalid compiler options",
        ffi::CL_INVALID_LINKER_OPTIONS => "invalid linker options",
        ffi::CL_INVALID_DEVICE_PARTITION_COUNT => "invalid device partition count",
        _ => unreachable!(),
    }
}

/// Convert an error code in a `Result`. Error code `0` means success.
pub fn catch_ffi(err: ffi::cl_int) -> Result<()> {
    match err {
        ffi::CL_SUCCESS => Ok(()),
        _ => Err(ErrorKind::RawError(err).into()),
    }
}
