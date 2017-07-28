//! A module defining the `cl_platform_id` related types, such as the high-level `Platform` type.

use wrapper::ffi;
use wrapper::types::device::{Type, Device};
use wrapper::information::InformationResult;
use std::fmt;
use errors::*;

pub mod information {
    //! A module containing the information marker types for `Platform`.

    use wrapper::ffi;
    use wrapper::information::*;

    /// A trait implemented by marker types for retrieving information through `clGetPlatformInfo`.
    pub trait PlatformInformation: Information<ffi::cl_platform_info> { }

    macro_rules! info_impl {
        ($type: ident, $result: ty, $id: expr, $id_name: expr, $test_fun: ident) => {
            general_info_impl!(PlatformInformation, ffi::cl_platform_info, $type, $result, $id, $id_name);

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
    platform_id: ffi::cl_platform_id,
}

impl Platform {
    pub(super) unsafe fn from_ffi(platform_id: ffi::cl_platform_id, _: bool) -> Self {
        Platform {
            platform_id,
        }
    }

    pub(super) fn underlying(self) -> ffi::cl_platform_id {
        self.platform_id
    }

    /// Return a default platform if any, namely the first platform given by `Platform::list`.
    ///
    /// # Panics
    /// Same as `list`.
    pub fn default() -> Option<Platform> {
        Platform::list().into_iter().next()
    }

    /// Return a list of available OpenCL platforms.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// use gprust::platform::Platform;
    ///
    /// # fn main() {
    /// for p in Platform::list() {
    ///     println!("{:?}", p);
    /// }
    /// # }
    /// ```
    ///
    /// # Panics
    /// Panic if the host fails to allocate resources.
    pub fn list() -> Vec<Platform> {
        let result = unsafe {
            InformationResult::get_info(|num_entries, platforms, num_platforms| {
                ffi::clGetPlatformIDs(num_entries, platforms, num_platforms)
            })
        };

        expect!(result, ffi::CL_OUT_OF_HOST_MEMORY)
    }

    /// Return a list of extensions supported by the platform.
    ///
    /// # Panics
    /// Same as `get_info`.
    pub fn extensions(&self) -> Vec<String> {
        self.get_info::<information::Extensions>()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect()
    }

    /// Query an information to the platform. `T` should be a marker type from the `information`
    /// module.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// use gprust::{platform, Platform};
    ///
    /// # fn main_() -> Result<(), &'static str> {
    /// let platform = Platform::default().ok_or("no default platform")?;
    /// let name = platform.get_info::<platform::information::Name>();
    /// # Ok(())
    /// # }
    /// # fn main() { main_().unwrap(); }
    /// ```
    ///
    /// # Panics
    /// Panic if the host fails to allocate resources, or if an invalid information param is
    /// passed (should only happen when a user incorrectly implements `PlatformInformation` on
    /// their own or if the information is not supported on the device and cargo features have not
    /// been set correctly, otherwise it is a bug).
    pub fn get_info<T: information::PlatformInformation>(&self) -> T::Result {
        let result = unsafe {
            InformationResult::get_info(|size, value, ret_size| {
                ffi::clGetPlatformInfo(
                    self.platform_id,
                    T::id(),
                    size,
                    value as _,
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
    /// use gprust::{device, Platform};
    ///
    /// # fn main_() -> Result<(), &'static str> {
    /// let platform = Platform::default().ok_or("no default platform")?;
    /// // Query all devices.
    /// let devices = platform.get_devices(device::ALL);
    /// # Ok(())
    /// # }
    /// # fn main() { main_().unwrap(); }
    /// ```
    ///
    /// ```
    /// # extern crate gprust;
    /// use gprust::{device, Platform};
    ///
    /// # fn main_() -> Result<(), &'static str> {
    /// let platform = Platform::default().ok_or("no default platform")?;
    /// // Query only devices which type is `CL_DEVICE_TYPE_GPU` or `CL_DEVICE_TYPE_ACCELERATOR`.
    /// let devices = platform.get_devices(
    ///     device::TypeBuilder::new().gpu().accelerator().finish()
    /// );
    /// # Ok(())
    /// # }
    /// # fn main() { main_().unwrap(); }
    /// ```
    ///
    /// # Panics
    /// Panic if the host or a device fails to allocate resources.
    pub fn get_devices(&self, ty: Type) -> Vec<Device> {
        // `InformationResult` will retain the devices, but since these are root-devices, this
        // will have no effect so it's fine.
        let result = unsafe {
            InformationResult::get_info(|num_entries, devices, num_devices| {
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

    /// Return the first device which name contains `pattern` if any.
    /// Not case-sentitive.
    ///
    /// # Panics
    /// Same as `Platform::get_devices`.
    pub fn match_device(&self, pattern: &str) -> Option<Device> {
        use wrapper::types::device;

        let lowercase = pattern.to_lowercase();
        self.get_devices(device::ALL)
            .into_iter()
            .find(|d| d.get_info::<device::information::Name>()
                       .to_lowercase()
                       .contains(&lowercase)
            )
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
