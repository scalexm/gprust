//! A module defining the trait `InformationResult`.

use wrapper::ffi;
use errors::*;
use std::{mem, ptr};

/// A trait for retrieving information through OpenCL functions.
/// A value of a high-level type implementing this trait can be retrieved through an appropriate
/// OpenCL function, using the more primitive type `InformationResult::Item`.
///
/// The `SizeType` parameter is there for vectors in order to differentiate between OpenCL
/// functions asking for the full size of the allocated vector (i.e. `N * size_of::<T>()` where
/// `N` is the len of a `Vec<T>`) and functions asking for the elements count (i.e. just `N`).
/// Functions asking for the full size will use `usize` while functions asking for the elements
/// count will use `cl_uint == u32`.
///
/// # Examples
///
/// ```notrust
/// use wrapper::ffi::*;
/// use std::os::raw::c_void;
///
/// let platform_id = /* a cl_platform_id */;
///
/// let name: String = unsafe {
///     InformationResult::ask_info(|size, value, ret_size| {
///         clGetPlatformInfo(platform_id, CL_PLATFORM_NAME, size, value as *mut c_void, ret_size)
///     })
/// };
/// ```
pub trait InformationResult<SizeType>: Sized {
    type Item;

    /// Compute a high-level value for type `Self` from `function`.
    ///
    /// # Safety
    /// `function` should abstract an appropriate OpenCL function.
    /// The signature of `function` must respect these informal conditions:
    /// * first argument should be the size of the allocated data, or zero if the allocated data is null
    /// * second argument should be a pointer to the allocated data, or a null pointer
    /// * third argument should be a mutable pointer to a `SizeType` value
    /// which, if non null, will be updated to contain the actual size
    /// of the queried data.
    /// * return value should map errors returned by the asbtracted OpenCL function
    ///
    /// # Errors
    /// A `RawError` will be returned if the error returned by `function` is non-null.
    /// Usually, this means that an invalid OpenCL object or information param was abstracted in
    /// `function`.
    unsafe fn ask_info<F>(function: F) -> Result<Self>
        where F: Fn(SizeType, *mut Self::Item, *mut SizeType) -> ffi::cl_int;
}

impl InformationResult<usize> for String {
    type Item = u8;

    unsafe fn ask_info<F>(function: F) -> Result<Self>
        where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
    {
        // We retrieve the size in bytes of the string, including the null terminator.
        let mut size = 0;
        catch_ffi(function(0, ptr::null_mut(), &mut size))?;

        if size == 0 {
            Ok(String::new())
        } else {
            // Allocate the string and retrieve the data.
            let mut raw = vec![0; size];
            catch_ffi(function(size, raw.as_mut_ptr(), ptr::null_mut()))?;

            // Remove the null terminator.
            let _ = raw.pop().expect("should have at least 1 char here");

            Ok(String::from_utf8(raw).expect("invalid utf8"))
        }
    }
}

// `ffi::cl_bool == u32` which size in bytes is 4, so we can't directly use the `bool`
// type which size in bytes is 1.
impl InformationResult<usize> for bool {
    type Item = ffi::cl_bool;

    unsafe fn ask_info<F>(function: F) -> Result<Self>
        where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
    {
        let result: Result<ffi::cl_bool> = InformationResult::ask_info(function);
        result.map(|value| value != 0)
    }
}

/// Macro for vector of scalars boilerplate.
macro_rules! vec_result_impl {
    ($type: ty) => {
        impl InformationResult<ffi::cl_uint> for Vec<$type> {
            type Item = $type;

            unsafe fn ask_info<F>(function: F) -> Result<Self>
                where F: Fn(ffi::cl_uint, *mut Self::Item, *mut ffi::cl_uint) -> ffi::cl_int
            {
                // We retrieve the number of elements in the vector.
                let mut size = 0;
                catch_ffi(function(0, ptr::null_mut(), &mut size))?;

                if size == 0 {
                    Ok(Vec::new())
                } else {
                    // Allocate the vector and retrieve the data.
                    let mut raw = vec![0 as $type; size as usize];
                    catch_ffi(function(size, raw.as_mut_ptr(), ptr::null_mut()))?;

                    Ok(raw)
                }
            }
        }

        impl InformationResult<usize> for Vec<$type> {
            type Item = $type;

            unsafe fn ask_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
            {
                // We retrieve the *total size* in bytes of the vector.
                let mut size = 0;
                catch_ffi(function(0, ptr::null_mut(), &mut size))?;

                if size == 0 {
                    Ok(Vec::new())
                } else {
                    // Allocate the vector: since `size` is the total size in bytes, we compute
                    // the corresponding number of elements.
                    let mut raw = vec![0 as $type; size / mem::size_of::<$type>()];
                    catch_ffi(function(size, raw.as_mut_ptr(), ptr::null_mut()))?;

                    Ok(raw)
                }
            }
        }
    };
}

/// Macro for scalar types boilerplate. Also call `vec_result_impl!` for these types.
macro_rules! result_impl {
    ($type: ty) => {
        impl InformationResult<usize> for $type {
            type Item = $type;

            unsafe fn ask_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
            {
                let mut raw = 0 as $type;
                catch_ffi(function(mem::size_of::<$type>(), &mut raw, ptr::null_mut()))?;
                Ok(raw)
            }
        }

        vec_result_impl!($type);
    };
}

result_impl!(ffi::cl_int);
result_impl!(ffi::cl_uint);
result_impl!(ffi::cl_bitfield);
result_impl!(usize);
result_impl!(isize);
result_impl!(ffi::cl_platform_id);
result_impl!(ffi::cl_device_id);

/// A trait describing a piece of information.
pub trait Information<T> {
    /// Type of the information result.
    type Result: InformationResult<usize>;

    /// OpenCL constant for identifying the piece of information, e.g. `CL_DEVICE_ADDRESS_BITS`.
    fn id() -> T;
}
