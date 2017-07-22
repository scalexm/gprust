//! A module defining the `cl_platform_id` related types, such as the high-level `Platform` type.

use wrapper::ffi;
use wrapper::types::device::{Type, Device};
use wrapper::information::InformationResult;
use std::fmt;
use errors::*;

pub mod information {
    //! A module containing the information marker types for `Platform`.

    use wrapper::ffi;

    pub(super) mod private {
        use wrapper::ffi;
        use wrapper::information::InformationResult;

        pub trait PlatformInformation {
            type Result: InformationResult<usize>;

            fn id() -> ffi::cl_platform_info;
        }
    }

    macro_rules! info_impl {
        ($type: ident, $result: ty, $id: expr, $test_fun: ident) => {
            pub struct $type;

            impl private::PlatformInformation for $type {
                type Result = $result;

                fn id() -> ffi::cl_platform_info {
                    $id
                }
            }

            #[test]
            fn $test_fun() {
                for p in super::Platform::list() {
                    let _ = p.get_info::<$type>();
                }
            }
        };
    }

    info_impl!(Profile, String, ffi::CL_PLATFORM_PROFILE, test_profile);
    info_impl!(Version, String, ffi::CL_PLATFORM_VERSION, test_version);
    info_impl!(Name, String, ffi::CL_PLATFORM_NAME, test_name);
    info_impl!(Vendor, String, ffi::CL_PLATFORM_VENDOR, test_vendor);
    info_impl!(Extensions, String, ffi::CL_PLATFORM_EXTENSIONS, test_extensions);
}

/// `Platform` is a high-level type which maps to the low-level `cl_platform_id` OpenCL type.
/// An object of type `Platform` acts as a reference to a physical platform. Hence, cloning a
/// platform is a shallow copy.
#[derive(Clone, PartialEq, Eq)]
pub struct Platform {
    pub(super) platform_id: ffi::cl_platform_id,
}

impl Platform {
    fn from_ffi(platform_id: ffi::cl_platform_id) -> Self {
        Platform {
            platform_id,
        }
    }

    /// Return the list of available OpenCL platforms.
    ///
    /// # Panics
    /// Panic if the host fails to allocate resources.
    pub fn list() -> Vec<Platform> {
        let result = unsafe {
            InformationResult::ask_info(|num_entries, platforms, num_platforms| {
                ffi::clGetPlatformIDs(num_entries, platforms, num_platforms)
            })
        };

        expect!(result, ffi::CL_OUT_OF_HOST_MEMORY)
    }

    /// Query an information to the platform. `T` should be a marker type from the `information`
    /// module.
    ///
    /// # Panics
    /// Panic if the host fails to allocate resources.
    pub fn get_info<T: information::private::PlatformInformation>(&self) -> T::Result {
        use std::os::raw::c_void;

        let result = unsafe {
            InformationResult::ask_info(|size, value, ret_size| {
                ffi::clGetPlatformInfo(
                    self.platform_id,
                    T::id(),
                    size,
                    value as *mut c_void,
                    ret_size
                )
            })
        };

        expect!(result, ffi::CL_OUT_OF_HOST_MEMORY)
    }

    /// Return the list of available devices for this platform.
    ///
    /// # Panics
    /// Panic if the host or a device fails to allocate resources.
    pub fn get_devices(&self, ty: Type) -> Vec<Device> {
        let result = unsafe {
            InformationResult::ask_info(|num_entries, devices, num_devices| {
                ffi::clGetDeviceIDs(
                    self.platform_id,
                    ty.get_bitfield(),
                    num_entries,
                    devices,
                    num_devices
                )
            })
        };

        if let &Err(Error(ErrorKind::RawError(ffi::CL_DEVICE_NOT_FOUND), _)) = &result {
            return Vec::new();
        }

        expect!(result, ffi::CL_OUT_OF_HOST_MEMORY, ffi::CL_OUT_OF_RESOURCES)
    }
}

map_ffi_impl!(Platform, ffi::cl_platform_id);

impl fmt::Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Platform")
         .field("name", &self.get_info::<information::Name>())
         .finish()
    }
}
