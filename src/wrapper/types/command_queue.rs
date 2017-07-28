//! A module defining `cl_command_queue` related types, such as the high-level `CommandQueue` type.

use wrapper::ffi;
use wrapper::types::context::Context;
use wrapper::types::device::Device;
use wrapper::information::InformationResult;
use errors::*;
use std::fmt;

pub mod information {
    //! A module containing the information marker types for `CommandQueue`.

    use wrapper::ffi;
    use wrapper::information::*;
    use wrapper::types::context;
    use wrapper::types::device;

    /// A trait implemented by marker types for retrieving information through `clCommandQueueInfo`.
    pub trait CommandQueueInformation: Information<ffi::cl_command_queue_info> { }

    macro_rules! info_impl {
        ($type: ident, $result: ty, $id: expr, $id_name: expr, $test_fun: ident) => {
            generic_info_impl!(CommandQueueInformation, ffi::cl_command_queue_info, $type, $result, $id, $id_name);

            #[test]
            fn $test_fun() {
                let queue = super::CommandQueue::default().unwrap();
                let c = queue.get_info::<$type>();
            }
        };
    }

    info_impl!(Context, context::Context, ffi::CL_QUEUE_CONTEXT, "CL_QUEUE_CONTEXT", test_context);
    info_impl!(Device, device::Device, ffi::CL_QUEUE_DEVICE, "CL_QUEUE_DEVICE", test_device);
    info_impl!(ReferenceCount, ffi::cl_uint, ffi::CL_QUEUE_REFERENCE_COUNT, "CL_QUEUE_REFERENCE_COUNT", test_reference_count);
    info_impl!(Properties, super::Properties, ffi::CL_QUEUE_PROPERTIES, "CL_QUEUE_PROPERTIES", test_properties);
}

bitfield_builder!(
    [Properties, PropertiesBuilder, "Properties"],
    "cl_command_queue_properties",
    [out_of_order_exec, "out_of_order_exec"] => ffi::CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE,
    [profiling, "profiling"] => ffi::CL_QUEUE_PROFILING_ENABLE
);

/// `CommandQueue` is a high-level type which maps to the low-level `cl_command_queue` OpenCL type.
/// An object of type `CommandQueue` acts as a reference to an OpenCL command queue. Hence, cloning
/// a command queue is a shallow copy.
/// The reference counter of a command queue is incremented on cloning and decrementing on dropping.
#[derive(PartialEq, Eq)]
pub struct CommandQueue {
    queue: ffi::cl_command_queue,
}

mod creation_error {
    error_chain! {
        types {
            CreationError, CreationErrorKind, ResultExt, Result;
        }

        errors {
            /// One of the specified properties is not supported on the device (can be checked
            /// through `Device::get_info::<device::information::QueueProperties>()`).
            PropertyNotSupported {
                description("property not supported")
            }

            /// Provided device was not associated with provided context.
            InvalidDevice {
                description("provided device was not associated with provided context")
            }
        }
    }
}

pub use self::creation_error::{CreationError, CreationErrorKind};

impl CommandQueue {
    /// Create a command queue on a device associated with a context.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// use gprust::{Device, Context, CommandQueue, command_queue};
    ///
    /// # fn main_() -> Result<(), &'static str> {
    /// let context = Context::default().ok_or("no default context")?;
    /// let device = Device::default().ok_or("no default device")?;
    /// if let Ok(queue) = CommandQueue::create(
    ///     &context,
    ///     &device,
    ///     command_queue::PropertiesBuilder::new().out_of_order_exec().finish()
    /// ) {
    ///     /* work with queue */
    /// }
    /// # Ok(())
    /// # }
    /// # fn main() { main_().unwrap(); }
    /// ```
    ///
    /// # Errors
    /// * `CreationErrorKind::PropertyNotSupported` if one of the specified properties is not
    /// supported on the device.
    /// * `CreationErrorKind::InvalidDevice` if the provided device is not associated with the
    /// provided context.
    ///
    /// # Panics
    /// Panic if the host or the device fails to allocate resources.
    pub fn create(context: &Context, device: &Device, properties: Properties)
        -> creation_error::Result<CommandQueue>
    {
        use wrapper::types::context;

        let mut error = 0;
        let queue = unsafe {
            ffi::clCreateCommandQueue(
                context.underlying(),
                device.underlying(),
                properties.bitfield,
                &mut error
            )
        };

        if error == ffi::CL_INVALID_QUEUE_PROPERTIES || error == ffi::CL_INVALID_VALUE {
            return Err(CreationErrorKind::PropertyNotSupported.into());
        } else if error == ffi::CL_INVALID_DEVICE {
            return Err(CreationErrorKind::InvalidDevice.into());
        }

        // Other errors will cause panic.
        let result = catch_ffi(error).map(|()| CommandQueue { queue });
        Ok(expect!(result, ffi::CL_OUT_OF_RESOURCES, ffi::CL_OUT_OF_HOST_MEMORY))
    }

    /// Return a default command queue if any, namely a command queue with empty properties for
    /// the context returned by `Context::default` and the device returned by `Device::default`.
    ///
    /// # Panics
    /// Same as `CommandQueue::create`.
    pub fn default() -> Option<CommandQueue> {
        Device::default().and_then(|d| Context::default().map(|c| (c, d)))
                         .and_then(|(c, d)| CommandQueue::create(&c, &d, Properties::new()).ok())
    }

    /// Query an information to the command queue. `T` should be a marker type from the
    /// `information` module.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// use gprust::{CommandQueue, command_queue};
    ///
    /// # fn main_() -> Result<(), &'static str> {
    /// let queue = CommandQueue::default().ok_or("no default command queue")?;
    /// let device = queue.get_info::<command_queue::information::Device>();
    /// # Ok(())
    /// # }
    /// # fn main() { main_().unwrap(); }
    /// ```
    ///
    /// # Panics
    /// Panic if the host or a device fails to allocate resources, or if an invalid information
    /// param is passed (should only happen when a user incorrectly implements
    /// `CommandQueueInformation` on their own or if the information is not supported on the
    /// context and cargo features have not been set correctly, otherwise it is a bug).
    pub fn get_info<T: information::CommandQueueInformation>(&self) -> T::Result {
        let result = unsafe {
            InformationResult::ask_info(|size, value, ret_size| {
                ffi::clGetCommandQueueInfo(
                    self.queue,
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

impl Clone for CommandQueue {
    fn clone(&self) -> Self {
        catch_ffi(unsafe { ffi::clRetainCommandQueue(self.queue) }).unwrap();

        CommandQueue {
            queue: self.queue,
        }
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        catch_ffi(unsafe { ffi::clReleaseCommandQueue(self.queue) }).unwrap();
    }
}

impl fmt::Debug for CommandQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CommandQueue")
         .field("context", &self.get_info::<information::Context>())
         .field("device", &self.get_info::<information::Device>())
         .finish()
    }
}

#[test]
fn test_relation_to_context_and_device() {
    let context = Context::default().unwrap();
    let device = Device::default().unwrap();
    let queue = CommandQueue::create(&context, &device, Properties::new()).unwrap();

    assert_eq!(context, queue.get_info::<information::Context>());
    assert_eq!(device, queue.get_info::<information::Device>());
}

#[test]
fn test_relation_to_properties() {
    let properties = PropertiesBuilder::new().profiling().finish();
    let queue = CommandQueue::create(
        &Context::default().unwrap(),
        &Device::default().unwrap(),
        properties.clone()
    );
    
    // In case `CL_QUEUE_PROFILING_ENABLE` is not supported...
    if let Ok(queue) = queue {
        assert_eq!(properties, queue.get_info::<information::Properties>());
    }
}
