//! A module defining the `cl_device_id` related types, such as the high-level `Device` type.

pub mod information;

use wrapper::ffi;
use wrapper::information::*;
use errors::*;
use std::fmt;

bitfield_builder!(
    [Type, TypeBuilder, "Type"],
    "cl_device_type",
    [gpu, "gpu"] => ffi::CL_DEVICE_TYPE_GPU,
    [cpu, "cpu"] => ffi::CL_DEVICE_TYPE_CPU,
    [accelerator, "accelerator"] => ffi::CL_DEVICE_TYPE_ACCELERATOR,
    [default, "default"] => ffi::CL_DEVICE_TYPE_DEFAULT,
    [custom, "custom"] => ffi::CL_DEVICE_TYPE_CUSTOM
);

impl Type {
    pub(super) fn get_bitfield(&self) -> ffi::cl_bitfield {
        self.bitfield
    }
}

/// Special bitfield value for `Type` combining all device types.
pub const ALL: Type = Type { bitfield: ffi::CL_DEVICE_TYPE_ALL };

bitfield!(
    FpConfig,
    "cl_device_fp_config",
    [denorm, "denorm"] => ffi::CL_FP_DENORM,
    [inf_nan, "inf_nan"] => ffi::CL_FP_INF_NAN,
    [round_to_nearest, "round_to_nearest"] => ffi::CL_FP_ROUND_TO_NEAREST,
    [round_to_zero, "round_to_zero"] => ffi::CL_FP_ROUND_TO_ZERO,
    [round_to_inf, "round_to_inf"] => ffi::CL_FP_ROUND_TO_INF,
    [fma, "fma"] => ffi::CL_FP_FMA,
    [soft_float, "soft_float"] => ffi::CL_FP_SOFT_FLOAT
);

bitfield!(
    PartitionAffinityDomain,
    "cl_device_affinity_domain",
    [numa, "numa"] => ffi::CL_DEVICE_AFFINITY_DOMAIN_NUMA,
    [l4_cache, "l4_cache"] => ffi::CL_DEVICE_AFFINITY_DOMAIN_L4_CACHE,
    [l3_cache, "l3_cache"] => ffi::CL_DEVICE_AFFINITY_DOMAIN_L3_CACHE,
    [l2_cache, "l2_cache"] => ffi::CL_DEVICE_AFFINITY_DOMAIN_L2_CACHE,
    [l1_cache, "l1_cache"] => ffi::CL_DEVICE_AFFINITY_DOMAIN_L1_CACHE,
    [next_partitionable, "next_partitionable"] => ffi::CL_DEVICE_AFFINITY_DOMAIN_NEXT_PARTITIONABLE
);

bitfield!(
    ExecutionCapabilities,
    "cl_device_exec_capabilities",
    [kernel, "kernel"] => ffi::CL_EXEC_KERNEL,
    [native_kernel, "native_kernel"] => ffi::CL_EXEC_NATIVE_KERNEL
);

bitfield!(
    QueueProperties,
    "cl_command_queue_properties",
    [out_of_order_exec_mode_enable, "out_of_order_exec_mode_enable"] => ffi::CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE,
    [profiling_enable, "profiling_enable"] => ffi::CL_QUEUE_PROFILING_ENABLE
);

enumz!(
    GlobalMemCacheType,
    ffi::cl_device_mem_cache_type,
    "cl_device_mem_cache_type",
    None => [ffi::CL_NONE, "CL_NONE"],
    ReadOnly => [ffi::CL_READ_ONLY_CACHE, "CL_READ_ONLY_CACHE"],
    ReadWrite => [ffi::CL_READ_WRITE_CACHE, "CL_READ_WRITE_CACHE"]
);

enumz!(
    LocalMemType,
    ffi::cl_device_local_mem_type,
    "cl_device_local_mem_type",
    None => [ffi::CL_NONE, "CL_NONE"],
    Local => [ffi::CL_LOCAL, "CL_LOCAL"],
    Global => [ffi::CL_GLOBAL, "CL_GLOBAL"]
);

enumz!(
    PartitionProperty,
    ffi::cl_device_partition_property,
    "cl_device_partition_property",
    PartitionEqually => [ffi::CL_DEVICE_PARTITION_EQUALLY, "CL_DEVICE_PARTITION_EQUALLY"],
    PartitionByCounts => [ffi::CL_DEVICE_PARTITION_BY_COUNTS, "CL_DEVICE_PARTITION_BY_COUNTS"],
    PartitionByAffinityDomain => [ffi::CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN, "CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN"]
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
    /// Same as `get_info`, but `CL_INVALID_VALUE` should never be triggered if the device
    /// correctly support OpenCL 1.2.
    pub fn is_subdevice(&self) -> bool {
        match self.get_info::<information::ParentDevice>() {
            ParentDevice::Device(_) => true,
            ParentDevice::None => false,
        }
    }

    /// Query an information to the device. `T` should be a marker type from the `information`
    /// module.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// # use gprust::{Platform, device};
    ///
    /// # fn main() {
    /// # let device = Platform::list().pop().unwrap().get_devices(device::ALL).pop().unwrap();
    /// // `device` is an object of type `Device`.
    /// let name = device.get_info::<device::information::Name>();
    /// # }
    /// ```
    ///
    /// # Panics
    /// Panic if the host or a device fails to allocate resources, or if an invalid information
    /// param is passed (should only happen when a user incorrectly implements
    /// `PlatformInformation` on their own or if the information is not supported on the device
    /// and cargo features have not been set correctly, otherwise it is a bug).
    pub fn get_info<T: information::DeviceInformation>(&self) -> T::Result {
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
