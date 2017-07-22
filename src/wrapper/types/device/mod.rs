//! A module defining the `cl_device_id` related types, such as the high-level `Device` type.

pub mod information;

use wrapper::ffi;
use wrapper::information::*;
use errors::*;
use std::fmt;

/// High-level bitfield mapping to `cl_device_type`.
bitfield_builder!(
    [Type, TypeBuilder],
    gpu => ffi::CL_DEVICE_TYPE_GPU,
    cpu => ffi::CL_DEVICE_TYPE_CPU,
    accelerator => ffi::CL_DEVICE_TYPE_ACCELERATOR,
    default => ffi::CL_DEVICE_TYPE_DEFAULT,
    custom => ffi::CL_DEVICE_TYPE_CUSTOM
);

impl Type {
    pub(super) fn get_bitfield(&self) -> ffi::cl_bitfield {
        self.bitfield
    }
}

/// Special bitfield value for `Type` combining all device types.
pub const ALL: Type = Type { bitfield: ffi::CL_DEVICE_TYPE_ALL };

/// High-level bitfield mapping to `cl_device_fp_config`.
bitfield!(
    FpConfig,
    denorm => ffi::CL_FP_DENORM,
    inf_nan => ffi::CL_FP_INF_NAN,
    round_to_nearest => ffi::CL_FP_ROUND_TO_NEAREST,
    round_to_zero => ffi::CL_FP_ROUND_TO_ZERO,
    round_to_inf => ffi::CL_FP_ROUND_TO_INF,
    fma => ffi::CL_FP_FMA,
    soft_float => ffi::CL_FP_SOFT_FLOAT
);

/// High-level bitfield mapping to `cl_device_affinity_domain`.
bitfield!(
    PartitionAffinityDomain,
    numa => ffi::CL_DEVICE_AFFINITY_DOMAIN_NUMA,
    l4_cache => ffi::CL_DEVICE_AFFINITY_DOMAIN_L4_CACHE,
    l3_cache => ffi::CL_DEVICE_AFFINITY_DOMAIN_L3_CACHE,
    l2_cache => ffi::CL_DEVICE_AFFINITY_DOMAIN_L2_CACHE,
    l1_cache => ffi::CL_DEVICE_AFFINITY_DOMAIN_L1_CACHE,
    next_partitionable => ffi::CL_DEVICE_AFFINITY_DOMAIN_NEXT_PARTITIONABLE
);

/// High-level bitfield mapping to `cl_device_exec_capabilities`.
bitfield!(
    ExecutionCapabilities,
    kernel => ffi::CL_EXEC_KERNEL,
    native_kernel => ffi::CL_EXEC_NATIVE_KERNEL
);

/// High-level bitfield mapping to `cl_command_queue_properties`.
bitfield!(
    QueueProperties,
    out_of_order_exec_mode_enable => ffi::CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE,
    profiling_enable => ffi::CL_QUEUE_PROFILING_ENABLE
);

/// High-level enum mapping to `cl_device_mem_cache_type`.
enumz!(
    GlobalMemCacheType,
    ffi::cl_device_mem_cache_type,
    None => ffi::CL_NONE,
    ReadOnly => ffi::CL_READ_ONLY_CACHE,
    ReadWrite => ffi::CL_READ_WRITE_CACHE
);

/// High-level enum mapping to `cl_device_local_mem_type`.
enumz!(
    LocalMemType,
    ffi::cl_device_local_mem_type,
    None => ffi::CL_NONE,
    Local => ffi::CL_LOCAL,
    Global => ffi::CL_GLOBAL
);

/// High-level enum mapping to `cl_device_partition_property`.
enumz!(
    PartitionProperty,
    ffi::cl_device_partition_property,
    PartitionEqually => ffi::CL_DEVICE_PARTITION_EQUALLY,
    PartitionByCounts => ffi::CL_DEVICE_PARTITION_BY_COUNTS,
    PartitionByAffinityDomain => ffi::CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN
);

/// `Device` is a high-level type which maps to the low-level `cl_device_id` OpenCL type.
/// An object of type `Device` acts as a reference to a physical or logical device. Hence, cloning
/// a device is a shallow copy.
/// The reference counter of a sub device will be incremented on cloning and decrementing on
/// dropping.
#[derive(PartialEq, Eq)]
pub struct Device {
    device_id: ffi::cl_device_id,
}

/// An enum used to represent a parent device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParentDevice {
    /// `None` means that the device queried against was a root device.
    None,

    /// `Device(device)` where `device` is the parent of the sub device queried against.
    Device(Device),
}

impl InformationResult<usize> for ParentDevice {
    type Item = ffi::cl_device_id;

    unsafe fn ask_info<F>(function: F) -> Result<Self>
        where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
    {
        use std::ptr;

        let device_id = InformationResult::ask_info(function)?;

        // The OpenCL specification states that if the device is a root-level one, `device_id`
        // will be a null pointer. Unfortunately, this is not the case on Apple platforms (at
        // least on mine), where `device_id` will simply not be filled. Hence, on these platforms
        // we rely on the default initialization to null in `InformationResult::ask_info` for
        // scalar types. Indeed, since `device_id` will be initialized to null but not filled,
        // it will remain null after the call to `clGetDeviceInfo`.

        if device_id == ptr::null_mut() {
            Ok(ParentDevice::None)
        } else {
            Ok(ParentDevice::Device(Device::from_ffi(device_id)))
        }
    }
}

impl Device {
    fn from_ffi(device_id: ffi::cl_device_id) -> Self {
        Device {
            device_id,
        }
    }

    /// Return `true` if the device is a sub device.
    ///
    /// # Panics
    /// Same as `get_info`.
    pub fn is_subdevice(&self) -> bool {
        match self.get_info::<information::ParentDevice>() {
            ParentDevice::Device(_) => true,
            ParentDevice::None => false,
        }
    }

    /// Query an information to the device. `T` should be a marker type from the `information`
    /// module.
    ///
    /// # Panics
    /// Panic if the host or a device fails to allocate resources.
    pub fn get_info<T: information::private::DeviceInformation>(&self) -> T::Result {
        use std::os::raw::c_void;

        let result = unsafe {
            InformationResult::ask_info(|size, value, ret_size| {
                ffi::clGetDeviceInfo(
                    self.device_id,
                    T::id(),
                    size,
                    value as *mut c_void,
                    ret_size
                )
            })
        };

        expect!(result, ffi::CL_OUT_OF_RESOURCES, ffi::CL_OUT_OF_HOST_MEMORY)
    }
}

map_ffi_impl!(Device, ffi::cl_device_id);

impl Clone for Device {
    fn clone(&self) -> Self {
        if self.is_subdevice() {
            catch_ffi(unsafe { ffi::clRetainDevice(self.device_id) }).unwrap()
        }

        Device {
            device_id: self.device_id,
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if self.is_subdevice() {
            catch_ffi(unsafe { ffi::clReleaseDevice(self.device_id) }).unwrap()
        }
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Device")
         .field("name", &self.get_info::<information::Name>())
         .field("type", &self.get_info::<information::Type>())
         .finish()
    }
}

#[test]
fn test_relation_to_platform() {
    use wrapper::types::platform::{self, Platform};
    
    for p in Platform::list() {
        for d in p.get_devices(ALL) {
            assert!(
                p.get_info::<platform::information::Name>()
                ==
                d.get_info::<information::Platform>()
                 .get_info::<platform::information::Name>()
            )
        }
    }
}
