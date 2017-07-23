//! A module defining the `cl_device_id` related types, such as the high-level `Device` type.

pub mod information;

use wrapper::ffi;
use wrapper::information::*;
use errors::*;
use std::fmt;
use std::iter::IntoIterator;

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
    [soft_float, "soft_float"] => ffi::CL_FP_SOFT_FLOAT,
    [correctly_rounded_divide_sqrt, "correctly_rounded_divide_sqrt"] => ffi::CL_FP_CORRECTLY_ROUNDED_DIVIDE_SQRT
);

bitfield_builder!(
    [AffinityDomain, AffinityDomainBuilder, "AffinityDomain"],
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

/// Enum indicating the partition type for partitioning a device into sub-devices.
pub enum PartitionType {
    /// Partition the device into equal sub-devices with `n` compute units.
    Equally(isize),

    /// `ByCounts(counts)`: for each non-zero count `m` in `counts`, create a sub-device with `m`
    /// compute units.
    ByCounts(Vec<isize>),

    /// `ByAffinityDomain(affinity)`: split the device into smaller aggregate devices containing
    /// one or more compute units that all share part of a cache hierarchy (ref: OpenCL
    /// specification) indicated by the `affinity` bitfield.
    ByAffinityDomain(AffinityDomain),
}

impl PartitionType {
    fn to_ffi(self) -> Vec<isize> {
        let mut partition = vec![];
        match self {
            PartitionType::Equally(n) => partition.extend(&[ffi::CL_DEVICE_PARTITION_EQUALLY, n]),
            PartitionType::ByCounts(counts) => {
                partition.push(ffi::CL_DEVICE_PARTITION_BY_COUNTS);
                partition.extend(counts);
                partition.push(ffi::CL_DEVICE_PARTITION_BY_COUNTS_LIST_END);
            }
            PartitionType::ByAffinityDomain(affinity) => {
                partition.extend(&[
                    ffi::CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN,
                    affinity.bitfield as isize
                ]);
            }
        }
        partition.push(0);
        partition
    }

    fn is_supported(&self, properties: PartitionProperties) -> bool {
        match *self {
            PartitionType::Equally(_) => properties.support_partition_equally(),
            PartitionType::ByCounts(_) => properties.support_partition_by_counts(),
            PartitionType::ByAffinityDomain(_) => properties.support_partition_by_affinity_domain(),
        }
    }
}

impl InformationResult<usize> for Option<PartitionType> {
    type Item = ffi::cl_device_partition_property;

    unsafe fn ask_info<F>(function: F) -> Result<Self>
        where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
    {
        let mut properties: Vec<_> = InformationResult::ask_info(function)?;

        // Empty or single trailing `0`.
        if properties.len() <= 1 {
            return Ok(None);
        }

        assert!(properties.len() >= 3); // Partition type at index 0, params following, `0` at the end.
        let _ = properties.pop().unwrap(); // Remove the trailing `0`.

        Ok(match properties[0] {
            ffi::CL_DEVICE_PARTITION_EQUALLY => {
                Some(PartitionType::Equally(properties[1]))
            }
            ffi::CL_DEVICE_PARTITION_BY_COUNTS => {
                // Remove the trailing `CL_DEVICE_PARTITION_BY_COUNTS_LIST_END`.
                let _ = properties.pop().unwrap();
                Some(PartitionType::ByCounts(properties.split_off(1)))
            }
            ffi::CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN => {
                Some(
                    PartitionType::ByAffinityDomain(AffinityDomain {
                        bitfield: properties[1] as ffi::cl_device_affinity_domain
                    })
                )
            }
            _ => unreachable!(),
        })
    }
}

/// Type indicating which partitions are supported on a device.
pub struct PartitionProperties {
    equally: bool,
    by_counts: bool,
    by_affinity_domain: bool,
}

impl PartitionProperties {
    /// Return `true` if the device supports CL_DEVICE_PARTITION_EQUALLY.
    pub fn support_partition_equally(&self) -> bool {
        self.equally
    }

    /// Return `true` if the device supports CL_DEVICE_PARTITION_BY_COUNTS.
    pub fn support_partition_by_counts(&self) -> bool {
        self.by_counts
    }

    /// Return `true` if the device supports CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN.
    pub fn support_partition_by_affinity_domain(&self) -> bool {
        self.by_affinity_domain
    }
}

impl InformationResult<usize> for PartitionProperties {
    type Item = ffi::cl_device_partition_property;

    unsafe fn ask_info<F>(function: F) -> Result<Self>
        where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
    {
        let properties: Vec<_> = InformationResult::ask_info(function)?;
        Ok(PartitionProperties {
            equally: properties.contains(&ffi::CL_DEVICE_PARTITION_EQUALLY),
            by_counts: properties.contains(&ffi::CL_DEVICE_PARTITION_BY_COUNTS),
            by_affinity_domain: properties.contains(&ffi::CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN),
        })
    }
}

/// `Device` is a high-level type which maps to the low-level `cl_device_id` OpenCL type.
/// An object of type `Device` acts as a reference to a physical or logical device. Hence, cloning
/// a device is a shallow copy.
/// The reference counter of a *sub*-device is incremented on cloning and decrementing on
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

    /// `Device(device)` where `device` is the parent of the sub-device queried against.
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

mod partition_error {
    error_chain! {
        types {
            PartitionError, PartitionErrorKind, ResultExt, Result;
        }

        errors {
            /// The device does not support this partition type.
            NotSupported {
                description("partition not supported")
            }

            /// The arguments following the partition were invalid.
            InvalidArguments {
                description("invalid arguments")
            }

            /// The device could not be further partitioned.
            Failed {
                description("partition failed")
            }
        }
    }
}

pub use self::partition_error::{PartitionError, PartitionErrorKind};

impl Device {
    fn from_ffi(device_id: ffi::cl_device_id) -> Self {
        Device {
            device_id,
        }
    }

    unsafe fn partition_unchecked(&self, partition: &[isize]) -> Result<Vec<Device>> {
        use std::ptr;

        // We retrieve the number of sub-devices that this partition will create.
        let mut num_devices = 0;
        catch_ffi(
            ffi::clCreateSubDevices(
                self.device_id,
                partition.as_ptr(),
                0,
                ptr::null_mut(),
                &mut num_devices
            )
        )?;

        let mut devices = vec![ptr::null_mut(); num_devices as usize];
        catch_ffi(
            ffi::clCreateSubDevices(
                self.device_id,
                partition.as_ptr(),
                num_devices,
                devices.as_mut_ptr(),
                ptr::null_mut()
            )
        )?;

       Ok(devices.into_iter().map(Device::from_ffi).collect())
    }

    /// Partition the device according to `partition`.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// # use gprust::{Platform, device};
    /// # fn main() {
    /// # let device = Platform::list().pop().unwrap().get_devices(device::ALL).pop().unwrap();
    /// // `device` is an object of type `Device`.
    /// if let Ok(sub_devices) = device.partition(device::PartitionType::Equally(2)) {
    ///     // Each sub-device in `sub_devices` has 2 compute units.
    ///     for sub in sub_devices {
    ///         assert!(
    ///             device::ParentDevice::Device(device.clone())
    ///             ==
    ///             sub.get_info::<device::information::ParentDevice>()
    ///         );
    ///     }
    /// }
    /// # }
    /// ```
    ///
    /// ```
    /// # extern crate gprust;
    /// # use gprust::{Platform, device};
    /// # fn main() {
    /// # let device = Platform::list().pop().unwrap().get_devices(device::ALL).pop().unwrap();
    /// // `device` is an object of type `Device`.
    /// if let Ok(sub_devices) = device.partition(
    ///     device::PartitionType::ByAffinityDomain(
    ///         device::AffinityDomainBuilder::new().next_partitionable().finish()
    ///     )
    /// ) {
    ///     // The device was split along the outermost cache line.
    ///     for sub in sub_devices {
    ///         assert!(
    ///             device::ParentDevice::Device(device.clone())
    ///             ==
    ///             sub.get_info::<device::information::ParentDevice>()
    ///         );
    ///     }
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    /// * `PartitionErrorKind::NotSupported` if `PartitionProperty::PartitionEqually` is not
    /// supported.
    /// * `PartitionErrorKind::InvalidValue` if the parameters of the partition type were invalid.
    /// * `PartitionErrorKind::Failed` if the partition failed.
    ///
    /// # Panics
    /// Same as `get_info`.
    pub fn partition(&self, partition: PartitionType)
        -> partition_error::Result<Vec<Device>>
    {
        if !partition.is_supported(self.get_info::<information::PartitionProperties>()) {
            return Err(PartitionErrorKind::NotSupported.into());
        }

        let partition = partition.to_ffi();
        let result = unsafe { self.partition_unchecked(&partition) };

        if let &Err(Error(ErrorKind::RawError(ffi::CL_DEVICE_PARTITION_FAILED), _)) = &result {
            return Err(PartitionErrorKind::Failed.into());
        }

        if let &Err(Error(ErrorKind::RawError(ffi::CL_INVALID_VALUE), _)) = &result {
            return Err(PartitionErrorKind::InvalidArguments.into());
        }

        Ok(expect!(result, ffi::CL_OUT_OF_RESOURCES, ffi::CL_OUT_OF_HOST_MEMORY))
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

    /// Return a list of built-in kernels supported by the device.
    ///
    /// # Panics
    /// Same as `get_info`.
    pub fn builtin_kernels(&self) -> Vec<String> {
        self.get_info::<information::BuiltinKernels>()
            .split(';')
            .map(|s| s.to_owned())
            .collect()
    }

    /// Return a list of extensions supported by the device.
    ///
    /// # Panics
    /// Same as `get_info`.
    pub fn extensions(&self) -> Vec<String> {
        self.get_info::<information::Extensions>()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect()
    }

    /// Query an information to the device. `T` should be a marker type from the `information`
    /// module.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// # use gprust::{Platform, device};
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
    /// `DeviceInformation` on their own or if the information is not supported on the device
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

        expect!(
            result,
            ffi::CL_OUT_OF_RESOURCES,
            ffi::CL_OUT_OF_HOST_MEMORY,
            ffi::CL_INVALID_VALUE
        )
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
