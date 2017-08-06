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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    fn to_ffi(self) -> Vec<ffi::cl_device_partition_property> {
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
                    affinity.bitfield as _
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

    unsafe fn get_info<F>(function: F) -> Result<Self, RawError>
        where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
    {
        let mut properties: Vec<_> = InformationResult::get_info(function)?;

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
                        bitfield: properties[1] as _,
                    })
                )
            }
            _ => unreachable!(),
        })
    }
}

/// Type indicating which partitions are supported on a device.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    unsafe fn get_info<F>(function: F) -> Result<Self, RawError>
        where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
    {
        let properties: Vec<_> = InformationResult::get_info(function)?;
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
/// The reference counter of a *sub*-device is incremented on cloning and decremented on
/// dropping.
#[derive(PartialEq, Eq)]
pub struct Device {
    device_id: ffi::cl_device_id,
}

unsafe impl Send for Device { }
unsafe impl Sync for Device { }

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

    unsafe fn get_info<F>(function: F) -> Result<Self, RawError>
        where F: Fn(usize, *mut Self::Item, *mut usize) -> ffi::cl_int
    {
        use std::ptr;

        let device_id = InformationResult::get_info(function)?;

        // The OpenCL specification states that if the device is a root-level one, `device_id`
        // will be a null pointer. Unfortunately, this is not the case on Apple platforms (at
        // least on mine), where `device_id` will simply not be filled. Hence, on these platforms
        // we rely on the default initialization to null in `InformationResult::get_info` for
        // scalar types. Indeed, since `device_id` will be initialized to null but not filled,
        // it will remain null after the call to `clGetDeviceInfo`.

        if device_id == ptr::null_mut() {
            Ok(ParentDevice::None)
        } else {
            Ok(ParentDevice::Device(Device::from_ffi(device_id, true)))
        }
    }
}

/// An error returned by `Device::partition`.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum PartitionError {
    /// The device does not support this partition type.
    NotSupported,

    /// The arguments following the partition were invalid.
    InvalidArguments,

    /// The device could not be further partitioned.
    Failed,
}

impl fmt::Display for PartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PartitionError::NotSupported =>
                write!(f, "partition not supported"),
            PartitionError::InvalidArguments =>
                write!(f, "invalid arguments"),
            PartitionError::Failed =>
                write!(f, "partition failed"),
        }
    }
}

impl Device {
    unsafe fn from_ffi(device_id: ffi::cl_device_id, retain: bool) -> Self {
        if retain {
            catch_ffi(ffi::clRetainDevice(device_id)).unwrap();
        }

        Device {
            device_id,
        }
    }

    /// Return a default device if any, namely the first GPU device among all available devices
    /// from all platforms if any, or the first CPU device among all available devices from all
    /// platforms if any, or the first device among all available devices from all platforms if
    /// any.
    ///
    /// # Panics
    /// Same as `Platform::get_devices`.
    pub fn default() -> Option<Device> {
        use wrapper::types::platform::Platform;

        let devices: Vec<_> = Platform::list().iter()
                                              .flat_map(|p| p.get_devices(ALL))
                                              .filter(|d| d.get_info::<information::Available>())
                                              .collect();

        if let Some(device) = devices.iter().find(|d| d.get_info::<information::Type>().gpu()) {
            return Some(device.clone());
        }

        if let Some(device) = devices.iter().find(|d| d.get_info::<information::Type>().cpu()) {
            return Some(device.clone());
        }

        devices.into_iter().next()
    }

    // Unsafe because one should be *VERY* careful with dropping and reference counting.
    pub(super) unsafe fn underlying(&self) -> ffi::cl_device_id {
        self.device_id
    }

    unsafe fn partition_unchecked(&self, partition: &[isize]) -> Result<Vec<Device>, RawError> {
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

        // Do not retain the sub-devices since this is already done by `clCreateSubDevices`.
        Ok(devices.into_iter().map(|d| Device::from_ffi(d, false)).collect())
    }

    /// Partition the device according to `partition`.
    ///
    /// # Examples
    /// ```
    /// # extern crate gprust;
    /// use gprust::{device, Device};
    ///
    /// # fn main_() -> Result<(), &'static str> {
    /// let device = Device::default().ok_or("no default device")?;
    /// if let Ok(sub_devices) = device.partition(device::PartitionType::Equally(8)) {
    ///     // Each sub-device in `sub_devices` has 8 compute units.
    ///     for sub in sub_devices {
    ///         assert_eq!(
    ///             device::ParentDevice::Device(device.clone()),
    ///             sub.get_info::<device::information::ParentDevice>()
    ///         );
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// # fn main() { main_().unwrap(); }
    /// ```
    ///
    /// ```
    /// # extern crate gprust;
    /// use gprust::{device, Device};
    ///
    /// # fn main_() -> Result<(), &'static str> {
    /// let device = Device::default().ok_or("no default device")?;
    /// if let Ok(sub_devices) = device.partition(
    ///     device::PartitionType::ByAffinityDomain(
    ///         device::AffinityDomainBuilder::new().next_partitionable().finish()
    ///     )
    /// ) {
    ///     // The device was split along the outermost cache line.
    ///     for sub in sub_devices {
    ///         assert_eq!(
    ///             device::ParentDevice::Device(device.clone()),
    ///             sub.get_info::<device::information::ParentDevice>()
    ///         );
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// # fn main() { main_().unwrap(); }
    /// ```
    ///
    /// # Errors
    /// * `PartitionError::NotSupported` if `PartitionProperty::PartitionEqually` is not
    /// supported.
    /// * `PartitionError::InvalidValue` if the parameters of the partition type were invalid.
    /// * `PartitionError::Failed` if the partition failed.
    ///
    /// # Panics
    /// Same as `get_info`.
    pub fn partition(&self, partition: PartitionType)
        -> Result<Vec<Device>, PartitionError>
    {
        if !partition.is_supported(self.get_info::<information::PartitionProperties>()) {
            return Err(PartitionError::NotSupported);
        }

        let partition = partition.to_ffi();
        let result = unsafe { self.partition_unchecked(&partition) };

        if let &Err(RawError(ffi::CL_DEVICE_PARTITION_FAILED)) = &result {
            return Err(PartitionError::Failed);
        }

        if let &Err(RawError(ffi::CL_INVALID_VALUE)) = &result {
            return Err(PartitionError::InvalidArguments);
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
    /// use gprust::{device, Device};
    ///
    /// # fn main_() -> Result<(), &'static str> {
    /// let device = Device::default().ok_or("no default device")?;
    /// let name = device.get_info::<device::information::Name>();
    /// # Ok(())
    /// # }
    /// # fn main() { main_().unwrap(); }
    /// ```
    ///
    /// # Panics
    /// Panic if the host or a device fails to allocate resources, or if an invalid information
    /// param is passed (should only happen when a user incorrectly implements
    /// `DeviceInformation` on their own or if the information is not supported on the device
    /// and cargo features have not been set correctly, otherwise it is a bug).
    pub fn get_info<T: information::DeviceInformation>(&self) -> T::Result {
        let result = unsafe {
            InformationResult::get_info(|size, value, ret_size| {
                ffi::clGetDeviceInfo(
                    self.device_id,
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

map_ffi_impl!(Device, ffi::cl_device_id);

impl Clone for Device {
    fn clone(&self) -> Self {
        catch_ffi(unsafe { ffi::clRetainDevice(self.device_id) }).unwrap();

        Device {
            device_id: self.device_id,
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        catch_ffi(unsafe { ffi::clReleaseDevice(self.device_id) }).unwrap();
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
            assert_eq!(
                p.get_info::<platform::information::Name>(),
                d.get_info::<information::Platform>()
                 .get_info::<platform::information::Name>()
            )
        }
    }
}

#[test]
fn test_relation_to_sub_device_partition_type() {
    use wrapper::types::platform::Platform;

    for p in Platform::list() {
        for d in p.get_devices(ALL) {
            if let Ok(sub_devices) = d.partition(PartitionType::Equally(8)) {
                for sub in sub_devices {
                    assert_eq!(
                        sub.get_info::<information::PartitionType>(),
                        Some(PartitionType::Equally(8))
                    )
                }
            }
        }
    }
}
