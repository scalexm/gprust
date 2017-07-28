//! A module defining the `cl_mem` related types, such as the high-level `Buffer` type.

use wrapper::ffi;
use wrapper::types::context::Context;
use wrapper::information::InformationResult;
use errors::*;
use std::mem;
use std::iter::{IntoIterator, ExactSizeIterator};

bitfield_builder!(
    [Flags, FlagsBuilder, "Flags"],
    "cl_mem_flags",
    [read_write, "read_write"] => ffi::CL_MEM_READ_WRITE,
    [read_only, "read_only"] => ffi::CL_MEM_READ_ONLY,
    [write_only, "write_only"] => ffi::CL_MEM_WRITE_ONLY,
    [host_write_only, "host_write_only"] => ffi::CL_MEM_HOST_WRITE_ONLY,
    [host_read_only, "host_read_only"] => ffi::CL_MEM_HOST_READ_ONLY,
    [host_no_access, "host_no_access"] => ffi::CL_MEM_HOST_NO_ACCESS
);

pub mod information {
    use wrapper::ffi;
    use wrapper::information::*;
    use wrapper::types::context;
    
    pub trait MemInformation: Information<ffi::cl_mem_info> { }

    macro_rules! info_impl {
        ($type: ident, $result: ty, $id: expr, $id_name: expr, $test_fun: ident) => {
            generic_info_impl!(MemInformation, ffi::cl_mem_info, $type, $result, $id, $id_name);

            #[test]
            fn $test_fun() {
                let context = context::Context::default().unwrap();
                let data = vec![1, 2, 3, 4];
                let buffer = super::Buffer::create(data, &context, super::Flags::new()).unwrap();
                let _ = buffer.get_info::<$type>();
            }
        };
    }

    // MemType
    info_impl!(Flags, super::Flags, ffi::CL_MEM_FLAGS, "CL_MEM_FLAGS", test_flags);
    info_impl!(Size, usize, ffi::CL_MEM_SIZE, "CL_MEM_SIZE", test_size);
    // HostPtr
    info_impl!(MapCount, ffi::cl_uint, ffi::CL_MEM_MAP_COUNT, "CL_MEM_MAP_COUNT", test_map_count);
    info_impl!(ReferenceCount, ffi::cl_uint, ffi::CL_MEM_REFERENCE_COUNT, "CL_MEM_REFERENCE_COUNT", test_reference_count);
    info_impl!(Context, context::Context, ffi::CL_MEM_CONTEXT, "CL_MEM_CONTEXT", test_context);
    // AssociatedObject
    // Offset
}

#[derive(PartialEq, Eq)]
pub struct Buffer {
    buffer: ffi::cl_mem,
}

mod creation_error {
    error_chain! {
        types {
            CreationError, CreationErrorKind, ResultExt, Result;
        }

        errors {
            EmptySlice {
                description("no data was provided (ZST not supported)")
            }

            InvalidFlags(s: &'static str) {
                description("invalid flags")
                display("{}", s)
            }

            AllocationFailure {
                description("failed to allocate memory")
            }
        }
    }
}

pub use self::creation_error::{CreationError, CreationErrorKind};

impl Buffer {
    unsafe fn from_ffi(buffer: ffi::cl_mem, retain: bool) -> Self {
        if retain {
            catch_ffi(ffi::clRetainMemObject(buffer)).unwrap();
        }

        Buffer {
            buffer,
        }
    }

    unsafe fn underlying(&self) -> ffi::cl_mem {
        self.buffer
    }

    pub fn create<I>(data: I, context: &Context, mut flags: Flags) -> creation_error::Result<Self>
        where I: IntoIterator, I::IntoIter: ExactSizeIterator
    {
        use wrapper::types::context;

        if (flags.read_write() && flags.read_only()) || (flags.read_write() && flags.write_only())
            || (flags.read_only() && flags.write_only())
        {
            return Err(
                CreationErrorKind::InvalidFlags(
                    "`read_write`, `read_only` and `write_only` are mutually exclusive"
                ).into()
            );
        }

        if (flags.host_no_access() && flags.host_read_only()) || (flags.host_no_access() && flags.host_write_only())
            || (flags.host_read_only() && flags.host_write_only())
        {
            return Err(
                CreationErrorKind::InvalidFlags(
                    "`host_no_access`, `host_read_only` and `host_write_only` are mutually exclusive"
                ).into()
            );
        }

        let data = data.into_iter();
        let size = mem::size_of::<I::Item>() * data.len();

        if size == 0 {
            return Err(CreationErrorKind::EmptySlice.into());
        }

        let data: Vec<_> = data.collect();
        flags.bitfield |= ffi::CL_MEM_COPY_HOST_PTR;

        let mut error = 0;
        let buffer = unsafe {
            ffi::clCreateBuffer(
                context.underlying(),
                flags.bitfield,
                size,
                data.as_ptr() as _,
                &mut error
            )
        };

        if error == ffi::CL_INVALID_BUFFER_SIZE || error == ffi::CL_MEM_OBJECT_ALLOCATION_FAILURE {
            return Err(CreationErrorKind::AllocationFailure.into());
        }

        let result = catch_ffi(error).map(|()| Buffer { buffer });
        Ok(expect!(result, ffi::CL_OUT_OF_RESOURCES, ffi::CL_OUT_OF_HOST_MEMORY))
    }

    pub fn get_info<T: information::MemInformation>(&self) -> T::Result {
        let result = unsafe {
            InformationResult::ask_info(|size, value, ret_size| {
                ffi::clGetMemObjectInfo(
                    self.buffer,
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

impl Clone for Buffer {
    fn clone(&self) -> Self {
        catch_ffi(unsafe { ffi::clRetainMemObject(self.buffer) }).unwrap();

        Buffer {
            buffer: self.buffer,
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        catch_ffi(unsafe { ffi::clReleaseMemObject(self.buffer) }).unwrap();
    }
}
