//! A module defining the `cl_platform_id` related types, such as the high-level `Platform` type.

use wrapper::ffi;
use wrapper::types::device::{Type, Device};
use wrapper::information::InformationResult;
use std::fmt;
use errors::*;

pub mod information {
    //! A module containing the information marker types for `Platform`.

    use wrapper::ffi;
    use wrapper::information::InformationResult;

    /// A trait implemented by marker types for retrieving information through `clGetPlatformInfo`.
    pub trait PlatformInformation {
            /// Type of the information result.
            type Result: InformationResult<usize>;

            /// OpenCL constant for identifying the piece of information, e.g. `CL_PLATFORM_PROFILE`.
            fn id() -> ffi::cl_platform_info;
        }

    macro_rules! info_impl {
        ($type: ident, $result: ty, $id: expr, $id_name: expr, $test_fun: ident) => {
            #[doc="Marker type mapping to `"] #[doc=$id_name] #[doc="`."]
            pub struct $type;

            impl PlatformInformation for $type {
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

    info_impl!(Profile, String, ffi::CL_PLATFORM_PROFILE, "CL_PLATFORM_PROFILE", test_profile);
    info_impl!(Version, String, ffi::CL_PLATFORM_VERSION, "CL_PLATFORM_VERSION", test_version);
    info_impl!(Name, String, ffi::CL_PLATFORM_NAME, "CL_PLATFORM_NAME", test_name);
    info_impl!(Vendor, String, ffi::CL_PLATFORM_VENDOR, "CL_PLATFORM_VENDOR", test_vendor);
    info_impl!(Extensions, String, ffi::CL_PLATFORM_EXTENSIONS, "CL_PLATFORM_EXTENSIONS", test_extensions);
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
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// # use gprust::{platform, Platform};
    ///
    /// # fn main() {
    /// # let platform = Platform::list().pop().unwrap();
    /// // `platform` is an object of type `Platform`.
    /// let name = platform.get_info::<platform::information::Name>();
    /// # }
    /// ```
    ///
    /// # Panics
    /// Panic if the host fails to allocate resources, or if an invalid information param is
    /// passed (should only happen when a user incorrectly implements `PlatformInformation` on
    /// their own or if the information is not supported on the device and cargo features have not
    /// been set correctly, otherwise it is a bug).
    pub fn get_info<T: information::PlatformInformation>(&self) -> T::Result {
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

        expect!(result, ffi::CL_OUT_OF_HOST_MEMORY, ffi::CL_INVALID_VALUE)
    }

    /// Return the list of available devices for this platform which satisfy the
    /// type constraints given by the `ty` bitfield.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// # use gprust::{device, Platform};
    ///
    /// # fn main() {
    /// # let platform = Platform::list().pop().unwrap();
    /// // Query all devices.
    /// let devices = platform.get_devices(device::ALL);
    /// # }
    /// ```
    ///
    /// ```
    /// # extern crate gprust;
    /// # use gprust::{device, Platform};
    ///
    /// # fn main() {
    /// # let platform = Platform::list().pop().unwrap();
    /// // Query only devices which type is `CL_DEVICE_TYPE_GPU` or `CL_DEVICE_TYPE_ACCELERATOR`.
    /// let devices = platform.get_devices(
    ///     device::TypeBuilder::new().gpu().accelerator().finish()
    /// );
    /// # }
    /// ```
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
