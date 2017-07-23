//! A module containing the information marker types for `Device`.

use wrapper::ffi;
use wrapper::information::*;

/// A trait implemented by marker types for retrieving information through `clGetDeviceInfo`.
pub trait DeviceInformation: Information<ffi::cl_device_info> { }

macro_rules! info_impl {
    ($type: ident, $result: ty, $id: expr, $id_name: expr, $test_fun: ident) => {
        generic_info_impl!(DeviceInformation, ffi::cl_device_info, $type, $result, $id, $id_name);

        #[test]
        fn $test_fun() {
            use wrapper::types::platform;

            for p in platform::Platform::list() {
                for d in p.get_devices(super::ALL) {
                    let _ = d.get_info::<$type>();
                }
            }
        }
    };
}

info_impl!(AddressBits, ffi::cl_uint, ffi::CL_DEVICE_ADDRESS_BITS, "CL_DEVICE_ADDRESS_BITS", test_address_bits);
info_impl!(Available, bool, ffi::CL_DEVICE_AVAILABLE, "CL_DEVICE_AVAILABLE", test_available);
info_impl!(BuiltinKernels, String, ffi::CL_DEVICE_BUILT_IN_KERNELS, "CL_DEVICE_BUILT_IN_KERNELS", test_builtin_kernels);
info_impl!(CompilerAvailable, bool, ffi::CL_DEVICE_COMPILER_AVAILABLE, "CL_DEVICE_COMPILER_AVAILABLE", test_compiler_available);
info_impl!(DoubleFpConfig, super::FpConfig, ffi::CL_DEVICE_DOUBLE_FP_CONFIG, "CL_DEVICE_DOUBLE_FP_CONFIG", test_double_fp_config);
info_impl!(EndianLittle, bool, ffi::CL_DEVICE_ENDIAN_LITTLE, "CL_DEVICE_ENDIAN_LITTLE", test_endian_little);
info_impl!(ErrorCorrectionSupport, bool, ffi::CL_DEVICE_ERROR_CORRECTION_SUPPORT, "CL_DEVICE_ERROR_CORRECTION_SUPPORT", test_error_correction_support);
info_impl!(ExecutionCapabilities, super::ExecutionCapabilities, ffi::CL_DEVICE_EXECUTION_CAPABILITIES, "CL_DEVICE_EXECUTION_CAPABILITIES", test_execution_capabilities);
info_impl!(Extensions, String, ffi::CL_DEVICE_EXTENSIONS, "CL_DEVICE_EXTENSIONS", test_extensions);
info_impl!(GlobalMemCacheSize, ffi::cl_ulong, ffi::CL_DEVICE_GLOBAL_MEM_CACHE_SIZE, "CL_DEVICE_GLOBAL_MEM_CACHE_SIZE", test_global_mem_cache_size);
info_impl!(GlobalMemCacheType, super::GlobalMemCacheType, ffi::CL_DEVICE_GLOBAL_MEM_CACHE_TYPE, "CL_DEVICE_GLOBAL_MEM_CACHE_TYPE", test_global_mem_cache_type);
info_impl!(GlobalMemCacheLineSize, ffi::cl_uint, ffi::CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE, "CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE", test_global_mem_cache_line_size);
info_impl!(GlobalMemSize, ffi::cl_ulong, ffi::CL_DEVICE_GLOBAL_MEM_SIZE, "CL_DEVICE_GLOBAL_MEM_SIZE", test_global_mem_size);
info_impl!(HostUnifiedMemory, bool, ffi::CL_DEVICE_HOST_UNIFIED_MEMORY, "CL_DEVICE_HOST_UNIFIED_MEMORY", test_host_unified_memory);
info_impl!(ImageSupport, bool, ffi::CL_DEVICE_IMAGE_SUPPORT, "CL_DEVICE_IMAGE_SUPPORT", test_image_support);
info_impl!(Image2dMaxHeight, usize, ffi::CL_DEVICE_IMAGE2D_MAX_HEIGHT, "CL_DEVICE_IMAGE2D_MAX_HEIGHT", test_imaged2d_max_height);
info_impl!(Image2dMaxWidth, usize, ffi::CL_DEVICE_IMAGE2D_MAX_WIDTH, "CL_DEVICE_IMAGE2D_MAX_WIDTH", test_image2d_max_width);
info_impl!(Image3dMaxDepth, usize, ffi::CL_DEVICE_IMAGE3D_MAX_DEPTH, "CL_DEVICE_IMAGE3D_MAX_DEPTH", test_image3d_max_depth);
info_impl!(Image3dMaxHeight, usize, ffi::CL_DEVICE_IMAGE3D_MAX_HEIGHT, "CL_DEVICE_IMAGE3D_MAX_HEIGHT", test_image3d_max_height);
info_impl!(Image3dMaxWidth, usize, ffi::CL_DEVICE_IMAGE3D_MAX_WIDTH, "CL_DEVICE_IMAGE3D_MAX_WIDTH", test_image3d_max_width);
info_impl!(ImageMaxBufferSize, usize, ffi::CL_DEVICE_IMAGE_MAX_BUFFER_SIZE, "CL_DEVICE_IMAGE_MAX_BUFFER_SIZE", test_image_max_buffer_size);
info_impl!(ImageMaxArraySize, usize, ffi::CL_DEVICE_IMAGE_MAX_ARRAY_SIZE, "CL_DEVICE_IMAGE_MAX_ARRAY_SIZE", test_image_max_array_size);
info_impl!(LinkerAvailable, bool, ffi::CL_DEVICE_LINKER_AVAILABLE, "CL_DEVICE_LINKER_AVAILABLE", test_linker_available);
info_impl!(LocalMemSize, ffi::cl_ulong, ffi::CL_DEVICE_LOCAL_MEM_SIZE, "CL_DEVICE_LOCAL_MEM_SIZE", test_local_mem_size);
info_impl!(LocalMemType, super::LocalMemType, ffi::CL_DEVICE_LOCAL_MEM_TYPE, "CL_DEVICE_LOCAL_MEM_TYPE", test_local_mem_type);
info_impl!(MaxClockFrequency, ffi::cl_uint, ffi::CL_DEVICE_MAX_CLOCK_FREQUENCY, "CL_DEVICE_MAX_CLOCK_FREQUENCY", test_max_clock_frequency);
info_impl!(MaxComputeUnits, ffi::cl_uint, ffi::CL_DEVICE_MAX_COMPUTE_UNITS, "CL_DEVICE_MAX_COMPUTE_UNITS", test_max_compute_units);
info_impl!(MaxConstantArgs, ffi::cl_uint, ffi::CL_DEVICE_MAX_CONSTANT_ARGS, "CL_DEVICE_MAX_CONSTANT_ARGS", test_max_constant_args);
info_impl!(MaxConstantBufferSize, ffi::cl_ulong, ffi::CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE, "CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE", test_max_constant_buffer_size);
info_impl!(MaxMemAllocSize, ffi::cl_ulong, ffi::CL_DEVICE_MAX_MEM_ALLOC_SIZE, "CL_DEVICE_MAX_MEM_ALLOC_SIZE", test_max_mem_alloc_size);
info_impl!(MaxParameterSize, usize, ffi::CL_DEVICE_MAX_PARAMETER_SIZE, "CL_DEVICE_MAX_PARAMETER_SIZE", test_max_parameter_size);
info_impl!(MaxReadImageArgs, ffi::cl_uint, ffi::CL_DEVICE_MAX_READ_IMAGE_ARGS, "CL_DEVICE_MAX_READ_IMAGE_ARGS", test_max_read_image_args);
info_impl!(MaxSamplers, ffi::cl_uint, ffi::CL_DEVICE_MAX_SAMPLERS, "CL_DEVICE_MAX_SAMPLERS", test_max_samplers);
info_impl!(MaxWorkGroupSize, usize, ffi::CL_DEVICE_MAX_WORK_GROUP_SIZE, "CL_DEVICE_MAX_WORK_GROUP_SIZE", test_max_work_group_size);
info_impl!(MaxWorkItemDimensions, ffi::cl_uint, ffi::CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS, "CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS", test_max_work_item_dimensions);
info_impl!(MaxWorkItemSizes, Vec<usize>, ffi::CL_DEVICE_MAX_WORK_ITEM_SIZES, "CL_DEVICE_MAX_WORK_ITEM_SIZES", test_max_work_item_sizes);
info_impl!(MaxWriteImageArgs, ffi::cl_uint, ffi::CL_DEVICE_MAX_WRITE_IMAGE_ARGS, "CL_DEVICE_MAX_WRITE_IMAGE_ARGS", test_max_write_image_args);
info_impl!(MemBaseAddrAlign, ffi::cl_uint, ffi::CL_DEVICE_MEM_BASE_ADDR_ALIGN, "CL_DEVICE_MEM_BASE_ADDR_ALIGN", test_mem_base_addr_align);
info_impl!(MinDataTypeAlignSize, ffi::cl_uint, ffi::CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE, "CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE", test_min_data_type_align_size);
info_impl!(Name, String, ffi::CL_DEVICE_NAME, "CL_DEVICE_NAME", test_name);
info_impl!(NativeVectorWidthChar, ffi::cl_uint, ffi::CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR, "CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR", test_native_vector_width_char);
info_impl!(NativeVectorWidthShort, ffi::cl_uint, ffi::CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT, "CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT", test_native_vector_width_short);
info_impl!(NativeVectorWidthInt, ffi::cl_uint, ffi::CL_DEVICE_NATIVE_VECTOR_WIDTH_INT, "CL_DEVICE_NATIVE_VECTOR_WIDTH_INT", test_native_vector_width_int);
info_impl!(NativeVectorWidthLong, ffi::cl_uint, ffi::CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG, "CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG", test_native_vector_width_long);
info_impl!(NativeVectorWidthFloat, ffi::cl_uint, ffi::CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT, "CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT", test_native_vector_width_float);
info_impl!(NativeVectorWidthDouble, ffi::cl_uint, ffi::CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE, "CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE", test_native_vector_width_double);
info_impl!(NativeVectorWidthHalf, ffi::cl_uint, ffi::CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF, "CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF", test_native_vector_width_half);
info_impl!(OpenClCVersion, String, ffi::CL_DEVICE_OPENCL_C_VERSION, "CL_DEVICE_OPENCL_C_VERSION", test_opencl_c_version);
info_impl!(ParentDevice, super::ParentDevice, ffi::CL_DEVICE_PARENT_DEVICE, "CL_DEVICE_PARENT_DEVICE", test_parent_device);
info_impl!(PartitionMaxSubDevices, ffi::cl_uint, ffi::CL_DEVICE_PARTITION_MAX_SUB_DEVICES, "CL_DEVICE_PARTITION_MAX_SUB_DEVICES", test_partition_max_sub_devices);
info_impl!(PartitionProperties, super::PartitionProperties, ffi::CL_DEVICE_PARTITION_PROPERTIES, "CL_DEVICE_PARTITION_PROPERTIES", test_partition_properties);
info_impl!(PartitionAffinityDomain, super::AffinityDomain, ffi::CL_DEVICE_PARTITION_AFFINITY_DOMAIN, "CL_DEVICE_PARTITION_AFFINITY_DOMAIN", test_partition_affinity_domain);
info_impl!(PartitionType, Option<super::PartitionType>, ffi::CL_DEVICE_PARTITION_TYPE, "CL_DEVICE_PARTITION_TYPE", test_partition_type);
info_impl!(Platform, ::wrapper::types::platform::Platform, ffi::CL_DEVICE_PLATFORM, "CL_DEVICE_PLATFORM", test_platform);
info_impl!(PreferredVectorWidthChar, ffi::cl_uint, ffi::CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR, "CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR", test_preferred_vector_width_char);
info_impl!(PreferredVectorWidthShort, ffi::cl_uint, ffi::CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT, "CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT", test_preferred_vector_width_short);
info_impl!(PreferredVectorWidthInt, ffi::cl_uint, ffi::CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT, "CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT", test_preferred_vector_width_int);
info_impl!(PreferredVectorWidthLong, ffi::cl_uint, ffi::CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG, "CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG", test_preferred_vector_width_long);
info_impl!(PreferredVectorWidthFloat, ffi::cl_uint, ffi::CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT, "CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT", test_preferred_vector_width_float);
info_impl!(PreferredVectorWidthDouble, ffi::cl_uint, ffi::CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE, "CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE", test_preferred_vector_width_double);
info_impl!(PreferredVectorWidthHalf, ffi::cl_uint, ffi::CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF, "CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF", test_preferred_vector_width_half);
info_impl!(PrintfBufferSize, usize, ffi::CL_DEVICE_PRINTF_BUFFER_SIZE, "CL_DEVICE_PRINTF_BUFFER_SIZE", test_printf_buffer_size);
info_impl!(PreferredInteropUserSync, bool, ffi::CL_DEVICE_PREFERRED_INTEROP_USER_SYNC, "CL_DEVICE_PREFERRED_INTEROP_USER_SYNC", test_preferred_interop_user_sync);
info_impl!(Profile, String, ffi::CL_DEVICE_PROFILE, "CL_DEVICE_PROFILE", test_profile);
info_impl!(ProfilingTimerResolution, usize, ffi::CL_DEVICE_PROFILING_TIMER_RESOLUTION, "CL_DEVICE_PROFILING_TIMER_RESOLUTION", test_profiling_timer_resolution);
info_impl!(QueueProperties, super::QueueProperties, ffi::CL_DEVICE_QUEUE_PROPERTIES, "CL_DEVICE_QUEUE_PROPERTIES", test_queue_properties);
info_impl!(ReferenceCount, ffi::cl_uint, ffi::CL_DEVICE_REFERENCE_COUNT, "CL_DEVICE_REFERENCE_COUNT", test_reference_count);
info_impl!(SingleFpConfig, super::FpConfig, ffi::CL_DEVICE_SINGLE_FP_CONFIG, "CL_DEVICE_SINGLE_FP_CONFIG", test_single_fp_config);
info_impl!(Type, super::Type, ffi::CL_DEVICE_TYPE, "CL_DEVICE_TYPE", test_type);
info_impl!(Vendor, String, ffi::CL_DEVICE_VENDOR, "CL_DEVICE_VENDOR", test_vendor);
info_impl!(VendorId, ffi::cl_uint, ffi::CL_DEVICE_VENDOR_ID, "CL_DEVICE_VENDOR_ID", test_vendor_id);
info_impl!(Version, String, ffi::CL_DEVICE_VERSION, "CL_DEVICE_VERSION", test_version);
info_impl!(DriverVersion, String, ffi::CL_DRIVER_VERSION, "CL_DRIVER_VERSION", test_driver_version);
