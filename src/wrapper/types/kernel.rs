//! A module defining the `cl_kernel` related types, such as the high-level `Kernel` type.

use wrapper::ffi;
use errors::*;

/// `Kernel` is a high-level type which maps to the low-level `cl_kernel` OpenCL type.
/// An object of type `Kernel` acts as a ref-counted reference to an OpenCL kernel.
#[derive(PartialEq, Eq)]
pub struct Kernel {
    kernel: ffi::cl_kernel,
}

unsafe impl Send for Kernel { }
unsafe impl Sync for Kernel { }

impl Kernel {
    pub(super) unsafe fn from_ffi(kernel: ffi::cl_kernel, retain: bool) -> Self {
        if retain {
            catch_ffi(ffi::clRetainKernel(kernel)).unwrap();
        }

        Kernel {
            kernel,
        }
    }
}

impl Clone for Kernel {
    fn clone(&self) -> Self {
        catch_ffi(unsafe { ffi::clRetainKernel(self.kernel) }).unwrap();

        Kernel {
            kernel: self.kernel,
        }
    }
}

impl Drop for Kernel {
    fn drop(&mut self) {
        catch_ffi(unsafe { ffi::clReleaseKernel(self.kernel) }).unwrap();
    }
}
