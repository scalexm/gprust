//! A module defining the `cl_context` related types, such as the high-level `Context` type.

use wrapper::ffi;
use wrapper::information::InformationResult;
use wrapper::types::platform::Platform;
use wrapper::types::device::Device;
use errors::*;
use std::iter::IntoIterator;

pub mod information {
    //! A module containing the information marker types for `Context`.

    use wrapper::ffi;
    use wrapper::information::*;
    use wrapper::types::device::Device;

    /// A trait implemented by marker types for retrieving information through `clGetContextInfo`.
    pub trait ContextInformation: Information<ffi::cl_context_info> { }

    macro_rules! info_impl {
        ($type: ident, $result: ty, $id: expr, $id_name: expr, $test_fun: ident) => {
            generic_info_impl!(ContextInformation, ffi::cl_context_info, $type, $result, $id, $id_name);

            #[test]
            fn $test_fun() {
                use wrapper::types::platform::Platform;
                use wrapper::types::device;
                use wrapper::types::context::{self, Context};

                for p in Platform::list() {
                    for d in p.get_devices(device::TypeBuilder::new().cpu().gpu().finish()) {
                        let context = Context::create(context::Properties::new(), vec![d]).unwrap();
                        let _ = context.get_info::<$type>();
                    }
                }
            }
        };
    }

    info_impl!(ReferenceCount, ffi::cl_uint, ffi::CL_CONTEXT_REFERENCE_COUNT, "CL_CONTEXT_REFERENCE_COUNT", test_reference_count);
    info_impl!(NumDevices, ffi::cl_uint, ffi::CL_CONTEXT_NUM_DEVICES, "CL_CONTEXT_NUM_DEVICES", test_num_devices);
    info_impl!(Devices, Vec<Device>, ffi::CL_CONTEXT_DEVICES, "CL_CONTEXT_DEVICES", test_devices);
    info_impl!(Properties, super::Properties, ffi::CL_CONTEXT_PROPERTIES, "CL_CONTEXT_PROPERTIES", test_properties);
}

/// Describe context properties to be passed to `clCreateContext` a.k.a `Context::create`.
/// It is to be used as a builder.
///
/// # Examples
/// ```
/// # extern crate gprust;
/// # use gprust::context;
/// # use gprust::Platform;
/// # fn main() {
/// # let platform = Platform::list().pop().unwrap();
/// // `platform` is an object of type `Platform`.
/// let properties = context::Properties::new().set_interop_user_sync()
///                                            .set_platform(platform);
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Properties {
    platform: Option<Platform>,
    interop_user_sync: bool,
}

impl Properties {
    /// Return an empty properties.
    pub fn new() -> Self {
        Properties {
            platform: None,
            interop_user_sync: false,
        }
    }

    /// Specify the platform to use (useful in case of multiple devices from different platforms).
    pub fn set_platform(mut self, platform: Platform) -> Self {
        self.platform = Some(platform);
        self
    }

    /// Specifiy that the user is responsible for synchronization between OpenCL and other APIs
    /// (see OpenCL specification).
    pub fn set_interop_user_sync(mut self) -> Self {
        self.interop_user_sync = true;
        self
    }

    fn into_ffi(self) -> Vec<ffi::cl_context_properties> {
        let mut properties = vec![];

        if let Some(platform) = self.platform {
            properties.push(ffi::CL_CONTEXT_PLATFORM);
            properties.push(platform.underlying() as _);
        }

        if self.interop_user_sync {
            properties.push(ffi::CL_CONTEXT_INTEROP_USER_SYNC);
            properties.push(ffi::CL_TRUE as _);
        }

        properties.push(0);
        properties
    }
}

impl InformationResult<usize> for Properties {
    type Item = ffi::cl_context_properties;

    unsafe fn ask_info<F>(function: F) -> Result<Self>
        where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
    {
        let mut properties: Vec<ffi::cl_context_properties> = InformationResult::ask_info(function)?;
        if properties.len() >= 1 {
            let _ = properties.pop().unwrap(); // Remove the trailing `0`.
        }

        let mut hl_properties = Properties::new();
        let mut iter = properties.into_iter();
        while let Some(property) = iter.next() {
            if property == ffi::CL_CONTEXT_PLATFORM {
                hl_properties = hl_properties.set_platform(
                    Platform::from_ffi(iter.next().unwrap() as _)
                );
            } else if property == ffi::CL_CONTEXT_INTEROP_USER_SYNC {
                if iter.next().unwrap() != 0 {
                    hl_properties = hl_properties.set_interop_user_sync();
                }
            }
        }
        
        Ok(hl_properties)
    }
}

/// `Context` is a high-level type which maps to the low-level `cl_context` OpenCL type.
/// An object of type `Context` acts as a reference to an OpenCL context. Hence, cloning
/// a context is a shallow copy.
/// The reference counter of a context is incremented on cloning and decrementing on dropping.
#[derive(PartialEq, Eq)]
pub struct Context {
    context: ffi::cl_context,
}

mod creation_error {
    error_chain! {
        types {
            CreationError, CreationErrorKind, ResultExt, Result;
        }

        errors {
            /// If no device was provided.
            NoDevice {
                description("no device was provided")
            }

            /// If the `CL_CONTEXT_INTEROP_USER_SYNC` property is not supported, typically for
            /// devices which support an OpenCL / OpenGL sharing extension
            /// (cf https://www.khronos.org/registry/OpenCL/specs/opencl-1.2-extensions.pdf, p43).
            InteropUserSyncNotSupported {
                description("CL_CONTEXT_INTEROP_USER_SYNC property is not supported")
            }

            /// If no platform was specified, and a platform could not be selected automatically.
            CannotSelectPlatform {
                description("could not select a platform")
            }

            /// One of the devices was not available (can be checked through
            /// `Device::get_info::<device::information::Available>`).
            DeviceNotAvailable {
                description("one of the devices was not available")
            }
        }
    }
}

pub use self::creation_error::{CreationError, CreationErrorKind};

impl Context {
    /// Create a context with one or more devices.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// # use gprust::{Platform, Context, device, context};
    /// # fn main() {
    /// # let platform = Platform::list().pop().unwrap();
    /// let devices = platform.get_devices(device::TypeBuilder::new().gpu().finish());
    /// let context = Context::create(context::Properties::new(), devices);
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `CreationErrorKind::NoDevice` if `devices` is empty.
    /// * `InteropUserSyncNotSupported` if `set_interop_user_sync` was called on `properties`,
    /// and the device does not support it (e.g. a device supporting an OpenCL / OpenGL sharing
    /// extension).
    /// * `CannotSelectPlatform` if no platform were specified in `properties`, and a platform
    /// could not be selected automatically.
    /// * `DeviceNotAvailable` if one of the devices was not available.
    ///
    /// # Panics
    /// Panic if the host or a device fails to allocate resources.
    pub fn create<I: IntoIterator<Item = Device>>(properties: Properties, devices: I)
        -> creation_error::Result<Self>
    {
        use std::ptr;

        // Be sure to keep this alive so that the device ids are not released.
        let devices: Vec<_> = devices.into_iter().collect();
        let device_ids: Vec<_> = devices.iter().map(|d| unsafe { d.underlying() }).collect();

        if devices.is_empty() {
            return Err(CreationErrorKind::NoDevice.into());
        }

        let mut error = 0;
        let properties = properties.into_ffi();
        let context = unsafe {
            ffi::clCreateContext(
                // `properties.len() == 1` means that `properties == [0]`, and since
                // OpenCL specification and implementations are very inconsistent,
                // this is not accepted as an empty list on all platforms.
                if properties.len() == 1 { ptr::null() } else { properties.as_ptr() },

                device_ids.len() as _,
                device_ids.as_ptr(),
                None,
                ptr::null_mut(),
                &mut error
            )
        };

        if error == ffi::CL_INVALID_PLATFORM {
            // The OpenCL specification says that this error can happen if no platform were
            // specified and a platform could not be automatically selected, or if the platform
            // given in `properties` were invalid. The latter *should not happen* since our
            // `Platform` type should only carry valid platforms. So necessarily, this is because
            // of the former reason.

            return Err(CreationErrorKind::CannotSelectPlatform.into());
        } else if error == ffi::CL_DEVICE_NOT_AVAILABLE {
            return Err(CreationErrorKind::DeviceNotAvailable.into());
        } else if error == ffi::CL_INVALID_VALUE || error == ffi::CL_INVALID_PROPERTY {
            // The only possible invalid thing is the fact that `CL_INTEROP_USER_SYNC` may be
            // unsupported.
            // Note that according to the specification, the error in this case should be
            // `CL_INVALID_PROPERTY`. Thanks to Apple, we also have to check `CL_INVALID_VALUE`.
            return Err(CreationErrorKind::InteropUserSyncNotSupported.into());
        }

        // Other errors will cause panic.
        let result = catch_ffi(error).map(|()| Context { context });
        Ok(expect!(result, ffi::CL_OUT_OF_RESOURCES, ffi::CL_OUT_OF_HOST_MEMORY))
    }

    /// Query an information to the context. `T` should be a marker type from the `information`
    /// module.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// # use gprust::{Platform, Context, device, context};
    /// # fn main() {
    /// # let device = Platform::list().pop().unwrap().get_devices(device::ALL).pop().unwrap();
    /// # let context = Context::create(context::Properties::new(), vec![device]).unwrap();
    /// // `context` is an object of type `Context`.
    /// let num_devices = context.get_info::<context::information::NumDevices>();
    /// # }
    /// ```
    ///
    /// # Panics
    /// Panic if the host or a device fails to allocate resources, or if an invalid information
    /// param is passed (should only happen when a user incorrectly implements
    /// `ContextInformation` on their own or if the information is not supported on the context
    /// and cargo features have not been set correctly, otherwise it is a bug).
    pub fn get_info<T: information::ContextInformation>(&self) -> T::Result {
        let result = unsafe {
            InformationResult::ask_info(|size, value, ret_size| {
                ffi::clGetContextInfo(
                    self.context,
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

impl Clone for Context {
    fn clone(&self) -> Self {
        catch_ffi(unsafe { ffi::clRetainContext(self.context) }).unwrap();

        Context {
            context: self.context,
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        catch_ffi(unsafe { ffi::clReleaseContext(self.context) }).unwrap();
    }
}

#[test]
fn test_relation_to_devices() {
    use wrapper::types::platform::Platform;
    use wrapper::types::device;

    for p in Platform::list() {
        let devices = p.get_devices(device::TypeBuilder::new().cpu().gpu().finish());
        let context = Context::create(Properties::new(), devices.clone()).unwrap();
        assert_eq!(
            devices.len(),
            context.get_info::<information::NumDevices>() as usize
        );
    }
}

#[test]
fn test_relation_to_properties() {
    use wrapper::types::platform::Platform;
    use wrapper::types::device;

    for p in Platform::list() {
        for d in p.get_devices(device::TypeBuilder::new().cpu().gpu().finish()) {
            let context = Context::create(Properties::new(), vec![d.clone()]).unwrap();
            assert_eq!(
                Properties::new(),
                context.get_info::<information::Properties>()
            );

            let properties = Properties::new().set_platform(p.clone());
            let context = Context::create(properties.clone(), vec![d]).unwrap();
            assert_eq!(
                properties,
                context.get_info::<information::Properties>()
            );
        }
    }
}
