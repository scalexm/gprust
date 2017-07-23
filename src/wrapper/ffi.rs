//! OpenCL 1.2 bindings.

pub const CL_SUCCESS: cl_int = 0;
pub const CL_DEVICE_NOT_FOUND: cl_int = -1;
pub const CL_DEVICE_NOT_AVAILABLE: cl_int = -2;
pub const CL_COMPILER_NOT_AVAILABLE: cl_int = -3;
pub const CL_MEM_OBJECT_ALLOCATION_FAILURE: cl_int = -4;
pub const CL_OUT_OF_RESOURCES: cl_int = -5;
pub const CL_OUT_OF_HOST_MEMORY: cl_int = -6;
pub const CL_PROFILING_INFO_NOT_AVAILABLE: cl_int = -7;
pub const CL_MEM_COPY_OVERLAP: cl_int = -8;
pub const CL_IMAGE_FORMAT_MISMATCH: cl_int = -9;
pub const CL_IMAGE_FORMAT_NOT_SUPPORTED: cl_int = -10;
pub const CL_BUILD_PROGRAM_FAILURE: cl_int = -11;
pub const CL_MAP_FAILURE: cl_int = -12;
pub const CL_MISALIGNED_SUB_BUFFER_OFFSET: cl_int = -13;
pub const CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST: cl_int = -14;
pub const CL_COMPILE_PROGRAM_FAILURE: cl_int = -15;
pub const CL_LINKER_NOT_AVAILABLE: cl_int = -16;
pub const CL_LINK_PROGRAM_FAILURE: cl_int = -17;
pub const CL_DEVICE_PARTITION_FAILED: cl_int = -18;
pub const CL_KERNEL_ARG_INFO_NOT_AVAILABLE: cl_int = -19;
pub const CL_INVALID_VALUE: cl_int = -30;
pub const CL_INVALID_DEVICE_TYPE: cl_int = -31;
pub const CL_INVALID_PLATFORM: cl_int = -32;
pub const CL_INVALID_DEVICE: cl_int = -33;
pub const CL_INVALID_CONTEXT: cl_int = -34;
pub const CL_INVALID_QUEUE_PROPERTIES: cl_int = -35;
pub const CL_INVALID_COMMAND_QUEUE: cl_int = -36;
pub const CL_INVALID_HOST_PTR: cl_int = -37;
pub const CL_INVALID_MEM_OBJECT: cl_int = -38;
pub const CL_INVALID_IMAGE_FORMAT_DESCRIPTOR: cl_int = -39;
pub const CL_INVALID_IMAGE_SIZE: cl_int = -40;
pub const CL_INVALID_SAMPLER: cl_int = -41;
pub const CL_INVALID_BINARY: cl_int = -42;
pub const CL_INVALID_BUILD_OPTIONS: cl_int = -43;
pub const CL_INVALID_PROGRAM: cl_int = -44;
pub const CL_INVALID_PROGRAM_EXECUTABLE: cl_int = -45;
pub const CL_INVALID_KERNEL_NAME: cl_int = -46;
pub const CL_INVALID_KERNEL_DEFINITION: cl_int = -47;
pub const CL_INVALID_KERNEL: cl_int = -48;
pub const CL_INVALID_ARG_INDEX: cl_int = -49;
pub const CL_INVALID_ARG_VALUE: cl_int = -50;
pub const CL_INVALID_ARG_SIZE: cl_int = -51;
pub const CL_INVALID_KERNEL_ARGS: cl_int = -52;
pub const CL_INVALID_WORK_DIMENSION: cl_int = -53;
pub const CL_INVALID_WORK_GROUP_SIZE: cl_int = -54;
pub const CL_INVALID_WORK_ITEM_SIZE: cl_int = -55;
pub const CL_INVALID_GLOBAL_OFFSET: cl_int = -56;
pub const CL_INVALID_EVENT_WAIT_LIST: cl_int = -57;
pub const CL_INVALID_EVENT: cl_int = -58;
pub const CL_INVALID_OPERATION: cl_int = -59;
pub const CL_INVALID_GL_OBJECT: cl_int = -60;
pub const CL_INVALID_BUFFER_SIZE: cl_int = -61;
pub const CL_INVALID_MIP_LEVEL: cl_int = -62;
pub const CL_INVALID_GLOBAL_WORK_SIZE: cl_int = -63;
pub const CL_INVALID_PROPERTY: cl_int = -64;
pub const CL_INVALID_IMAGE_DESCRIPTOR: cl_int = -65;
pub const CL_INVALID_COMPILER_OPTIONS: cl_int = -66;
pub const CL_INVALID_LINKER_OPTIONS: cl_int = -67;
pub const CL_INVALID_DEVICE_PARTITION_COUNT: cl_int = -68;

pub const CL_VERSION_1_0: cl_int = 1;
pub const CL_VERSION_1_1: cl_int = 1;
pub const CL_VERSION_1_2: cl_int = 1;

pub const CL_FALSE: cl_bool = 0;
pub const CL_TRUE: cl_bool = 1;

pub const CL_BLOCKING: cl_int = 1;
pub const CL_NON_BLOCKING: cl_int = 0;

pub const CL_PLATFORM_PROFILE: cl_platform_info = 2304;
pub const CL_PLATFORM_VERSION: cl_platform_info = 2305;
pub const CL_PLATFORM_NAME: cl_platform_info = 2306;
pub const CL_PLATFORM_VENDOR: cl_platform_info = 2307;
pub const CL_PLATFORM_EXTENSIONS: cl_platform_info = 2308;

pub const CL_DEVICE_TYPE_DEFAULT: cl_device_type = 1;
pub const CL_DEVICE_TYPE_CPU: cl_device_type = 2;
pub const CL_DEVICE_TYPE_GPU: cl_device_type = 4;
pub const CL_DEVICE_TYPE_ACCELERATOR: cl_device_type = 8;
pub const CL_DEVICE_TYPE_CUSTOM: cl_device_type = 16;
pub const CL_DEVICE_TYPE_ALL: cl_device_type = 4294967295;

pub const CL_DEVICE_TYPE: cl_device_info = 4096;
pub const CL_DEVICE_VENDOR_ID: cl_device_info = 4097;
pub const CL_DEVICE_MAX_COMPUTE_UNITS: cl_device_info = 4098;
pub const CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS: cl_device_info = 4099;
pub const CL_DEVICE_MAX_WORK_GROUP_SIZE: cl_device_info = 4100;
pub const CL_DEVICE_MAX_WORK_ITEM_SIZES: cl_device_info = 4101;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR: cl_device_info = 4102;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT: cl_device_info = 4103;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT: cl_device_info = 4104;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG: cl_device_info = 4105;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT: cl_device_info = 4106;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE: cl_device_info = 4107;
pub const CL_DEVICE_MAX_CLOCK_FREQUENCY: cl_device_info = 4108;
pub const CL_DEVICE_ADDRESS_BITS: cl_device_info = 4109;
pub const CL_DEVICE_MAX_READ_IMAGE_ARGS: cl_device_info = 4110;
pub const CL_DEVICE_MAX_WRITE_IMAGE_ARGS: cl_device_info = 4111;
pub const CL_DEVICE_MAX_MEM_ALLOC_SIZE: cl_device_info = 4112;
pub const CL_DEVICE_IMAGE2D_MAX_WIDTH: cl_device_info = 4113;
pub const CL_DEVICE_IMAGE2D_MAX_HEIGHT: cl_device_info = 4114;
pub const CL_DEVICE_IMAGE3D_MAX_WIDTH: cl_device_info = 4115;
pub const CL_DEVICE_IMAGE3D_MAX_HEIGHT: cl_device_info = 4116;
pub const CL_DEVICE_IMAGE3D_MAX_DEPTH: cl_device_info = 4117;
pub const CL_DEVICE_IMAGE_SUPPORT: cl_device_info = 4118;
pub const CL_DEVICE_MAX_PARAMETER_SIZE: cl_device_info = 4119;
pub const CL_DEVICE_MAX_SAMPLERS: cl_device_info = 4120;
pub const CL_DEVICE_MEM_BASE_ADDR_ALIGN: cl_device_info = 4121;
pub const CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE: cl_device_info = 4122;
pub const CL_DEVICE_SINGLE_FP_CONFIG: cl_device_info = 4123;
pub const CL_DEVICE_GLOBAL_MEM_CACHE_TYPE: cl_device_info = 4124;
pub const CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE: cl_device_info = 4125;
pub const CL_DEVICE_GLOBAL_MEM_CACHE_SIZE: cl_device_info = 4126;
pub const CL_DEVICE_GLOBAL_MEM_SIZE: cl_device_info = 4127;
pub const CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE: cl_device_info = 4128;
pub const CL_DEVICE_MAX_CONSTANT_ARGS: cl_device_info = 4129;
pub const CL_DEVICE_LOCAL_MEM_TYPE: cl_device_info = 4130;
pub const CL_DEVICE_LOCAL_MEM_SIZE: cl_device_info = 4131;
pub const CL_DEVICE_ERROR_CORRECTION_SUPPORT: cl_device_info = 4132;
pub const CL_DEVICE_PROFILING_TIMER_RESOLUTION: cl_device_info = 4133;
pub const CL_DEVICE_ENDIAN_LITTLE: cl_device_info = 4134;
pub const CL_DEVICE_AVAILABLE: cl_device_info = 4135;
pub const CL_DEVICE_COMPILER_AVAILABLE: cl_device_info = 4136;
pub const CL_DEVICE_EXECUTION_CAPABILITIES: cl_device_info = 4137;
pub const CL_DEVICE_QUEUE_PROPERTIES: cl_device_info = 4138;
pub const CL_DEVICE_NAME: cl_device_info = 4139;
pub const CL_DEVICE_VENDOR: cl_device_info = 4140;
pub const CL_DRIVER_VERSION: cl_device_info = 4141;
pub const CL_DEVICE_PROFILE: cl_device_info = 4142;
pub const CL_DEVICE_VERSION: cl_device_info = 4143;
pub const CL_DEVICE_EXTENSIONS: cl_device_info = 4144;
pub const CL_DEVICE_PLATFORM: cl_device_info = 4145;
pub const CL_DEVICE_DOUBLE_FP_CONFIG: cl_device_info = 4146;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF: cl_device_info = 4148;
pub const CL_DEVICE_HOST_UNIFIED_MEMORY: cl_device_info = 4149;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR: cl_device_info = 4150;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT: cl_device_info = 4151;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_INT: cl_device_info = 4152;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG: cl_device_info = 4153;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT: cl_device_info = 4154;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE: cl_device_info = 4155;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF: cl_device_info = 4156;
pub const CL_DEVICE_OPENCL_C_VERSION: cl_device_info = 4157;
pub const CL_DEVICE_LINKER_AVAILABLE: cl_device_info = 4158;
pub const CL_DEVICE_BUILT_IN_KERNELS: cl_device_info = 4159;
pub const CL_DEVICE_IMAGE_MAX_BUFFER_SIZE: cl_device_info = 4160;
pub const CL_DEVICE_IMAGE_MAX_ARRAY_SIZE: cl_device_info = 4161;
pub const CL_DEVICE_PARENT_DEVICE: cl_device_info = 4162;
pub const CL_DEVICE_PARTITION_MAX_SUB_DEVICES: cl_device_info = 4163;
pub const CL_DEVICE_PARTITION_PROPERTIES: cl_device_info = 4164;
pub const CL_DEVICE_PARTITION_AFFINITY_DOMAIN: cl_device_info = 4165;
pub const CL_DEVICE_PARTITION_TYPE: cl_device_info = 4166;
pub const CL_DEVICE_REFERENCE_COUNT: cl_device_info = 4167;
pub const CL_DEVICE_PREFERRED_INTEROP_USER_SYNC: cl_device_info = 4168;
pub const CL_DEVICE_PRINTF_BUFFER_SIZE: cl_device_info = 4169;
pub const CL_DEVICE_IMAGE_PITCH_ALIGNMENT: cl_device_info = 4170;
pub const CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT: cl_device_info = 4171;

pub const CL_FP_DENORM: cl_device_fp_config = 1;
pub const CL_FP_INF_NAN: cl_device_fp_config = 2;
pub const CL_FP_ROUND_TO_NEAREST: cl_device_fp_config = 4;
pub const CL_FP_ROUND_TO_ZERO: cl_device_fp_config = 8;
pub const CL_FP_ROUND_TO_INF: cl_device_fp_config = 16;
pub const CL_FP_FMA: cl_device_fp_config = 32;
pub const CL_FP_SOFT_FLOAT: cl_device_fp_config = 64;
pub const CL_FP_CORRECTLY_ROUNDED_DIVIDE_SQRT: cl_device_fp_config = 128;

pub const CL_NONE: cl_device_mem_cache_type = 0;
pub const CL_READ_ONLY_CACHE: cl_device_mem_cache_type = 1;
pub const CL_READ_WRITE_CACHE: cl_device_mem_cache_type = 2;

pub const CL_LOCAL: cl_device_local_mem_type = 1;
pub const CL_GLOBAL: cl_device_local_mem_type = 2;

pub const CL_EXEC_KERNEL: cl_device_exec_capabilities = 1;
pub const CL_EXEC_NATIVE_KERNEL: cl_device_exec_capabilities = 2;

pub const CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE: cl_command_queue_properties = 1;
pub const CL_QUEUE_PROFILING_ENABLE: cl_command_queue_properties = 2;

pub const CL_CONTEXT_REFERENCE_COUNT: cl_context_info = 4224;
pub const CL_CONTEXT_DEVICES: cl_context_info = 4225;
pub const CL_CONTEXT_PROPERTIES: cl_context_info = 4226;
pub const CL_CONTEXT_NUM_DEVICES: cl_context_info = 4227;
pub const CL_CONTEXT_PLATFORM: cl_context_properties = 4228;
pub const CL_CONTEXT_INTEROP_USER_SYNC: cl_context_properties = 4229;

pub const CL_DEVICE_PARTITION_EQUALLY: cl_device_partition_property = 4230;
pub const CL_DEVICE_PARTITION_BY_COUNTS: cl_device_partition_property = 4231;
pub const CL_DEVICE_PARTITION_BY_COUNTS_LIST_END: cl_device_partition_property = 0;
pub const CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN: cl_device_partition_property = 4232;

pub const CL_DEVICE_AFFINITY_DOMAIN_NUMA: cl_device_affinity_domain = 1;
pub const CL_DEVICE_AFFINITY_DOMAIN_L4_CACHE: cl_device_affinity_domain = 2;
pub const CL_DEVICE_AFFINITY_DOMAIN_L3_CACHE: cl_device_affinity_domain = 4;
pub const CL_DEVICE_AFFINITY_DOMAIN_L2_CACHE: cl_device_affinity_domain = 8;
pub const CL_DEVICE_AFFINITY_DOMAIN_L1_CACHE: cl_device_affinity_domain = 16;
pub const CL_DEVICE_AFFINITY_DOMAIN_NEXT_PARTITIONABLE: cl_device_affinity_domain = 32;

pub const CL_QUEUE_CONTEXT: cl_int = 4240;
pub const CL_QUEUE_DEVICE: cl_int = 4241;
pub const CL_QUEUE_REFERENCE_COUNT: cl_int = 4242;
pub const CL_QUEUE_PROPERTIES: cl_int = 4243;
pub const CL_MEM_READ_WRITE: cl_int = 1;
pub const CL_MEM_WRITE_ONLY: cl_int = 2;
pub const CL_MEM_READ_ONLY: cl_int = 4;
pub const CL_MEM_USE_HOST_PTR: cl_int = 8;
pub const CL_MEM_ALLOC_HOST_PTR: cl_int = 16;
pub const CL_MEM_COPY_HOST_PTR: cl_int = 32;
pub const CL_MEM_HOST_WRITE_ONLY: cl_int = 128;
pub const CL_MEM_HOST_READ_ONLY: cl_int = 256;
pub const CL_MEM_HOST_NO_ACCESS: cl_int = 512;
pub const CL_MIGRATE_MEM_OBJECT_HOST: cl_int = 1;
pub const CL_MIGRATE_MEM_OBJECT_CONTENT_UNDEFINED: cl_int = 2;
pub const CL_R: cl_int = 4272;
pub const CL_A: cl_int = 4273;
pub const CL_RG: cl_int = 4274;
pub const CL_RA: cl_int = 4275;
pub const CL_RGB: cl_int = 4276;
pub const CL_RGBA: cl_int = 4277;
pub const CL_BGRA: cl_int = 4278;
pub const CL_ARGB: cl_int = 4279;
pub const CL_INTENSITY: cl_int = 4280;
pub const CL_LUMINANCE: cl_int = 4281;
pub const CL_Rx: cl_int = 4282;
pub const CL_RGx: cl_int = 4283;
pub const CL_RGBx: cl_int = 4284;
pub const CL_DEPTH: cl_int = 4285;
pub const CL_DEPTH_STENCIL: cl_int = 4286;
pub const CL_SNORM_INT8: cl_int = 4304;
pub const CL_SNORM_INT16: cl_int = 4305;
pub const CL_UNORM_INT8: cl_int = 4306;
pub const CL_UNORM_INT16: cl_int = 4307;
pub const CL_UNORM_SHORT_565: cl_int = 4308;
pub const CL_UNORM_SHORT_555: cl_int = 4309;
pub const CL_UNORM_INT_101010: cl_int = 4310;
pub const CL_SIGNED_INT8: cl_int = 4311;
pub const CL_SIGNED_INT16: cl_int = 4312;
pub const CL_SIGNED_INT32: cl_int = 4313;
pub const CL_UNSIGNED_INT8: cl_int = 4314;
pub const CL_UNSIGNED_INT16: cl_int = 4315;
pub const CL_UNSIGNED_INT32: cl_int = 4316;
pub const CL_HALF_FLOAT: cl_int = 4317;
pub const CL_FLOAT: cl_int = 4318;
pub const CL_UNORM_INT24: cl_int = 4319;
pub const CL_MEM_OBJECT_BUFFER: cl_int = 4336;
pub const CL_MEM_OBJECT_IMAGE2D: cl_int = 4337;
pub const CL_MEM_OBJECT_IMAGE3D: cl_int = 4338;
pub const CL_MEM_OBJECT_IMAGE2D_ARRAY: cl_int = 4339;
pub const CL_MEM_OBJECT_IMAGE1D: cl_int = 4340;
pub const CL_MEM_OBJECT_IMAGE1D_ARRAY: cl_int = 4341;
pub const CL_MEM_OBJECT_IMAGE1D_BUFFER: cl_int = 4342;
pub const CL_MEM_TYPE: cl_int = 4352;
pub const CL_MEM_FLAGS: cl_int = 4353;
pub const CL_MEM_SIZE: cl_int = 4354;
pub const CL_MEM_HOST_PTR: cl_int = 4355;
pub const CL_MEM_MAP_COUNT: cl_int = 4356;
pub const CL_MEM_REFERENCE_COUNT: cl_int = 4357;
pub const CL_MEM_CONTEXT: cl_int = 4358;
pub const CL_MEM_ASSOCIATED_MEMOBJECT: cl_int = 4359;
pub const CL_MEM_OFFSET: cl_int = 4360;
pub const CL_IMAGE_FORMAT: cl_int = 4368;
pub const CL_IMAGE_ELEMENT_SIZE: cl_int = 4369;
pub const CL_IMAGE_ROW_PITCH: cl_int = 4370;
pub const CL_IMAGE_SLICE_PITCH: cl_int = 4371;
pub const CL_IMAGE_WIDTH: cl_int = 4372;
pub const CL_IMAGE_HEIGHT: cl_int = 4373;
pub const CL_IMAGE_DEPTH: cl_int = 4374;
pub const CL_IMAGE_ARRAY_SIZE: cl_int = 4375;
pub const CL_IMAGE_BUFFER: cl_int = 4376;
pub const CL_IMAGE_NUM_MIP_LEVELS: cl_int = 4377;
pub const CL_IMAGE_NUM_SAMPLES: cl_int = 4378;
pub const CL_ADDRESS_NONE: cl_int = 4400;
pub const CL_ADDRESS_CLAMP_TO_EDGE: cl_int = 4401;
pub const CL_ADDRESS_CLAMP: cl_int = 4402;
pub const CL_ADDRESS_REPEAT: cl_int = 4403;
pub const CL_ADDRESS_MIRRORED_REPEAT: cl_int = 4404;
pub const CL_FILTER_NEAREST: cl_int = 4416;
pub const CL_FILTER_LINEAR: cl_int = 4417;
pub const CL_SAMPLER_REFERENCE_COUNT: cl_int = 4432;
pub const CL_SAMPLER_CONTEXT: cl_int = 4433;
pub const CL_SAMPLER_NORMALIZED_COORDS: cl_int = 4434;
pub const CL_SAMPLER_ADDRESSING_MODE: cl_int = 4435;
pub const CL_SAMPLER_FILTER_MODE: cl_int = 4436;
pub const CL_MAP_READ: cl_int = 1;
pub const CL_MAP_WRITE: cl_int = 2;
pub const CL_MAP_WRITE_INVALIDATE_REGION: cl_int = 4;
pub const CL_PROGRAM_REFERENCE_COUNT: cl_int = 4448;
pub const CL_PROGRAM_CONTEXT: cl_int = 4449;
pub const CL_PROGRAM_NUM_DEVICES: cl_int = 4450;
pub const CL_PROGRAM_DEVICES: cl_int = 4451;
pub const CL_PROGRAM_SOURCE: cl_int = 4452;
pub const CL_PROGRAM_BINARY_SIZES: cl_int = 4453;
pub const CL_PROGRAM_BINARIES: cl_int = 4454;
pub const CL_PROGRAM_NUM_KERNELS: cl_int = 4455;
pub const CL_PROGRAM_KERNEL_NAMES: cl_int = 4456;
pub const CL_PROGRAM_BUILD_STATUS: cl_int = 4481;
pub const CL_PROGRAM_BUILD_OPTIONS: cl_int = 4482;
pub const CL_PROGRAM_BUILD_LOG: cl_int = 4483;
pub const CL_PROGRAM_BINARY_TYPE: cl_int = 4484;
pub const CL_PROGRAM_BINARY_TYPE_NONE: cl_int = 0;
pub const CL_PROGRAM_BINARY_TYPE_COMPILED_OBJECT: cl_int = 1;
pub const CL_PROGRAM_BINARY_TYPE_LIBRARY: cl_int = 2;
pub const CL_PROGRAM_BINARY_TYPE_EXECUTABLE: cl_int = 4;
pub const CL_BUILD_SUCCESS: cl_int = 0;
pub const CL_BUILD_NONE: cl_int = -1;
pub const CL_BUILD_ERROR: cl_int = -2;
pub const CL_BUILD_IN_PROGRESS: cl_int = -3;
pub const CL_KERNEL_FUNCTION_NAME: cl_int = 4496;
pub const CL_KERNEL_NUM_ARGS: cl_int = 4497;
pub const CL_KERNEL_REFERENCE_COUNT: cl_int = 4498;
pub const CL_KERNEL_CONTEXT: cl_int = 4499;
pub const CL_KERNEL_PROGRAM: cl_int = 4500;
pub const CL_KERNEL_ATTRIBUTES: cl_int = 4501;
pub const CL_KERNEL_ARG_ADDRESS_QUALIFIER: cl_int = 4502;
pub const CL_KERNEL_ARG_ACCESS_QUALIFIER: cl_int = 4503;
pub const CL_KERNEL_ARG_TYPE_NAME: cl_int = 4504;
pub const CL_KERNEL_ARG_TYPE_QUALIFIER: cl_int = 4505;
pub const CL_KERNEL_ARG_NAME: cl_int = 4506;
pub const CL_KERNEL_ARG_ADDRESS_GLOBAL: cl_int = 4507;
pub const CL_KERNEL_ARG_ADDRESS_LOCAL: cl_int = 4508;
pub const CL_KERNEL_ARG_ADDRESS_CONSTANT: cl_int = 4509;
pub const CL_KERNEL_ARG_ADDRESS_PRIVATE: cl_int = 4510;
pub const CL_KERNEL_ARG_ACCESS_READ_ONLY: cl_int = 4512;
pub const CL_KERNEL_ARG_ACCESS_WRITE_ONLY: cl_int = 4513;
pub const CL_KERNEL_ARG_ACCESS_READ_WRITE: cl_int = 4514;
pub const CL_KERNEL_ARG_ACCESS_NONE: cl_int = 4515;
pub const CL_KERNEL_ARG_TYPE_NONE: cl_int = 0;
pub const CL_KERNEL_ARG_TYPE_CONST: cl_int = 1;
pub const CL_KERNEL_ARG_TYPE_RESTRICT: cl_int = 2;
pub const CL_KERNEL_ARG_TYPE_VOLATILE: cl_int = 4;
pub const CL_KERNEL_WORK_GROUP_SIZE: cl_int = 4528;
pub const CL_KERNEL_COMPILE_WORK_GROUP_SIZE: cl_int = 4529;
pub const CL_KERNEL_LOCAL_MEM_SIZE: cl_int = 4530;
pub const CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE: cl_int = 4531;
pub const CL_KERNEL_PRIVATE_MEM_SIZE: cl_int = 4532;
pub const CL_KERNEL_GLOBAL_WORK_SIZE: cl_int = 4533;
pub const CL_EVENT_COMMAND_QUEUE: cl_int = 4560;
pub const CL_EVENT_COMMAND_TYPE: cl_int = 4561;
pub const CL_EVENT_REFERENCE_COUNT: cl_int = 4562;
pub const CL_EVENT_COMMAND_EXECUTION_STATUS: cl_int = 4563;
pub const CL_EVENT_CONTEXT: cl_int = 4564;
pub const CL_COMMAND_NDRANGE_KERNEL: cl_int = 4592;
pub const CL_COMMAND_TASK: cl_int = 4593;
pub const CL_COMMAND_NATIVE_KERNEL: cl_int = 4594;
pub const CL_COMMAND_READ_BUFFER: cl_int = 4595;
pub const CL_COMMAND_WRITE_BUFFER: cl_int = 4596;
pub const CL_COMMAND_COPY_BUFFER: cl_int = 4597;
pub const CL_COMMAND_READ_IMAGE: cl_int = 4598;
pub const CL_COMMAND_WRITE_IMAGE: cl_int = 4599;
pub const CL_COMMAND_COPY_IMAGE: cl_int = 4600;
pub const CL_COMMAND_COPY_IMAGE_TO_BUFFER: cl_int = 4601;
pub const CL_COMMAND_COPY_BUFFER_TO_IMAGE: cl_int = 4602;
pub const CL_COMMAND_MAP_BUFFER: cl_int = 4603;
pub const CL_COMMAND_MAP_IMAGE: cl_int = 4604;
pub const CL_COMMAND_UNMAP_MEM_OBJECT: cl_int = 4605;
pub const CL_COMMAND_MARKER: cl_int = 4606;
pub const CL_COMMAND_ACQUIRE_GL_OBJECTS: cl_int = 4607;
pub const CL_COMMAND_RELEASE_GL_OBJECTS: cl_int = 4608;
pub const CL_COMMAND_READ_BUFFER_RECT: cl_int = 4609;
pub const CL_COMMAND_WRITE_BUFFER_RECT: cl_int = 4610;
pub const CL_COMMAND_COPY_BUFFER_RECT: cl_int = 4611;
pub const CL_COMMAND_USER: cl_int = 4612;
pub const CL_COMMAND_BARRIER: cl_int = 4613;
pub const CL_COMMAND_MIGRATE_MEM_OBJECTS: cl_int = 4614;
pub const CL_COMMAND_FILL_BUFFER: cl_int = 4615;
pub const CL_COMMAND_FILL_IMAGE: cl_int = 4616;
pub const CL_COMPLETE: cl_int = 0;
pub const CL_RUNNING: cl_int = 1;
pub const CL_SUBMITTED: cl_int = 2;
pub const CL_QUEUED: cl_int = 3;
pub const CL_BUFFER_CREATE_TYPE_REGION: cl_int = 4640;
pub const CL_PROFILING_COMMAND_QUEUED: cl_int = 4736;
pub const CL_PROFILING_COMMAND_SUBMIT: cl_int = 4737;
pub const CL_PROFILING_COMMAND_START: cl_int = 4738;
pub const CL_PROFILING_COMMAND_END: cl_int = 4739;

pub type cl_char = i8;
pub type cl_uchar = u8;
pub type cl_short = i16;
pub type cl_ushort = u16;
pub type cl_int = i32;
pub type cl_uint = u32;
pub type cl_long = i64;
pub type cl_ulong = u64;
pub type cl_half = u16;
pub type cl_float = f32;
pub type cl_double = f64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_platform_id {
    _unused: [u8; 0],
}
/******************************************************************************/
pub type cl_platform_id = *mut _cl_platform_id;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_device_id {
    _unused: [u8; 0],
}
pub type cl_device_id = *mut _cl_device_id;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_context {
    _unused: [u8; 0],
}
pub type cl_context = *mut _cl_context;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_command_queue {
    _unused: [u8; 0],
}
pub type cl_command_queue = *mut _cl_command_queue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_mem {
    _unused: [u8; 0],
}
pub type cl_mem = *mut _cl_mem;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_program {
    _unused: [u8; 0],
}
pub type cl_program = *mut _cl_program;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_kernel {
    _unused: [u8; 0],
}
pub type cl_kernel = *mut _cl_kernel;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_event {
    _unused: [u8; 0],
}
pub type cl_event = *mut _cl_event;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_sampler {
    _unused: [u8; 0],
}
pub type cl_sampler = *mut _cl_sampler;
pub type cl_bool = cl_uint;
pub type cl_bitfield = cl_ulong;
pub type cl_device_type = cl_bitfield;
pub type cl_platform_info = cl_uint;
pub type cl_device_info = cl_uint;
pub type cl_device_fp_config = cl_bitfield;
pub type cl_device_mem_cache_type = cl_uint;
pub type cl_device_local_mem_type = cl_uint;
pub type cl_device_exec_capabilities = cl_bitfield;
pub type cl_command_queue_properties = cl_bitfield;
pub type cl_device_partition_property = isize;
pub type cl_device_affinity_domain = cl_bitfield;
pub type cl_context_properties = isize;
pub type cl_context_info = cl_uint;
pub type cl_command_queue_info = cl_uint;
pub type cl_channel_order = cl_uint;
pub type cl_channel_type = cl_uint;
pub type cl_mem_flags = cl_bitfield;
pub type cl_mem_object_type = cl_uint;
pub type cl_mem_info = cl_uint;
pub type cl_mem_migration_flags = cl_bitfield;
pub type cl_image_info = cl_uint;
pub type cl_buffer_create_type = cl_uint;
pub type cl_addressing_mode = cl_uint;
pub type cl_filter_mode = cl_uint;
pub type cl_sampler_info = cl_uint;
pub type cl_map_flags = cl_bitfield;
pub type cl_program_info = cl_uint;
pub type cl_program_build_info = cl_uint;
pub type cl_program_binary_type = cl_uint;
pub type cl_build_status = cl_int;
pub type cl_kernel_info = cl_uint;
pub type cl_kernel_arg_info = cl_uint;
pub type cl_kernel_arg_address_qualifier = cl_uint;
pub type cl_kernel_arg_access_qualifier = cl_uint;
pub type cl_kernel_arg_type_qualifier = cl_bitfield;
pub type cl_kernel_work_group_info = cl_uint;
pub type cl_event_info = cl_uint;
pub type cl_command_type = cl_uint;
pub type cl_profiling_info = cl_uint;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _cl_image_format {
    pub image_channel_order: cl_channel_order,
    pub image_channel_data_type: cl_channel_type,
}
impl Clone for _cl_image_format {
    fn clone(&self) -> Self { *self }
}
pub type cl_image_format = _cl_image_format;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _cl_image_desc {
    pub image_type: cl_mem_object_type,
    pub image_width: usize,
    pub image_height: usize,
    pub image_depth: usize,
    pub image_array_size: usize,
    pub image_row_pitch: usize,
    pub image_slice_pitch: usize,
    pub num_mip_levels: cl_uint,
    pub num_samples: cl_uint,
    pub buffer: cl_mem,
}
impl Clone for _cl_image_desc {
    fn clone(&self) -> Self { *self }
}
pub type cl_image_desc = _cl_image_desc;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _cl_buffer_region {
    pub origin: usize,
    pub size: usize,
}
impl Clone for _cl_buffer_region {
    fn clone(&self) -> Self { *self }
}
pub type cl_buffer_region = _cl_buffer_region;

#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg_attr(target_os = "windows", link(name = "OpenCL"))]
#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
extern "system" {
    /********************************************************************************************************/
    pub fn clGetPlatformIDs(arg1: cl_uint, arg2: *mut cl_platform_id,
                            arg3: *mut cl_uint) -> cl_int;

    pub fn clGetPlatformInfo(arg1: cl_platform_id, arg2: cl_platform_info,
                             arg3: usize, arg4: *mut ::std::os::raw::c_void,
                             arg5: *mut usize) -> cl_int;

    pub fn clGetDeviceIDs(arg1: cl_platform_id, arg2: cl_device_type,
                          arg3: cl_uint, arg4: *mut cl_device_id,
                          arg5: *mut cl_uint) -> cl_int;

    pub fn clGetDeviceInfo(arg1: cl_device_id, arg2: cl_device_info,
                           arg3: usize, arg4: *mut ::std::os::raw::c_void,
                           arg5: *mut usize) -> cl_int;

    pub fn clCreateSubDevices(arg1: cl_device_id,
                              arg2: *const cl_device_partition_property,
                              arg3: cl_uint, arg4: *mut cl_device_id,
                              arg5: *mut cl_uint) -> cl_int;

    pub fn clRetainDevice(arg1: cl_device_id) -> cl_int;

    pub fn clReleaseDevice(arg1: cl_device_id) -> cl_int;

    pub fn clCreateContext(arg1: *const cl_context_properties, arg2: cl_uint,
                           arg3: *const cl_device_id,
                           arg4:
                               ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                              *const ::std::os::raw::c_char,
                                                                          arg2:
                                                                              *const ::std::os::raw::c_void,
                                                                          arg3:
                                                                              usize,
                                                                          arg4:
                                                                              *mut ::std::os::raw::c_void)>,
                           arg5: *mut ::std::os::raw::c_void,
                           arg6: *mut cl_int) -> cl_context;

    pub fn clCreateContextFromType(arg1: *const cl_context_properties,
                                   arg2: cl_device_type,
                                   arg3:
                                       ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                      *const ::std::os::raw::c_char,
                                                                                  arg2:
                                                                                      *const ::std::os::raw::c_void,
                                                                                  arg3:
                                                                                      usize,
                                                                                  arg4:
                                                                                      *mut ::std::os::raw::c_void)>,
                                   arg4: *mut ::std::os::raw::c_void,
                                   arg5: *mut cl_int) -> cl_context;

    pub fn clRetainContext(arg1: cl_context) -> cl_int;

    pub fn clReleaseContext(arg1: cl_context) -> cl_int;

    pub fn clGetContextInfo(arg1: cl_context, arg2: cl_context_info,
                            arg3: usize, arg4: *mut ::std::os::raw::c_void,
                            arg5: *mut usize) -> cl_int;

    pub fn clCreateCommandQueue(arg1: cl_context, arg2: cl_device_id,
                                arg3: cl_command_queue_properties,
                                arg4: *mut cl_int) -> cl_command_queue;

    pub fn clRetainCommandQueue(arg1: cl_command_queue) -> cl_int;

    pub fn clReleaseCommandQueue(arg1: cl_command_queue) -> cl_int;

    pub fn clGetCommandQueueInfo(arg1: cl_command_queue,
                                 arg2: cl_command_queue_info, arg3: usize,
                                 arg4: *mut ::std::os::raw::c_void,
                                 arg5: *mut usize) -> cl_int;

    pub fn clCreateBuffer(arg1: cl_context, arg2: cl_mem_flags, arg3: usize,
                          arg4: *mut ::std::os::raw::c_void,
                          arg5: *mut cl_int) -> cl_mem;

    pub fn clCreateSubBuffer(arg1: cl_mem, arg2: cl_mem_flags,
                             arg3: cl_buffer_create_type,
                             arg4: *const ::std::os::raw::c_void,
                             arg5: *mut cl_int) -> cl_mem;

    pub fn clCreateImage(arg1: cl_context, arg2: cl_mem_flags,
                         arg3: *const cl_image_format,
                         arg4: *const cl_image_desc,
                         arg5: *mut ::std::os::raw::c_void, arg6: *mut cl_int)
     -> cl_mem;

    pub fn clRetainMemObject(arg1: cl_mem) -> cl_int;

    pub fn clReleaseMemObject(arg1: cl_mem) -> cl_int;

    pub fn clGetSupportedImageFormats(arg1: cl_context, arg2: cl_mem_flags,
                                      arg3: cl_mem_object_type, arg4: cl_uint,
                                      arg5: *mut cl_image_format,
                                      arg6: *mut cl_uint) -> cl_int;

    pub fn clGetMemObjectInfo(arg1: cl_mem, arg2: cl_mem_info, arg3: usize,
                              arg4: *mut ::std::os::raw::c_void,
                              arg5: *mut usize) -> cl_int;

    pub fn clGetImageInfo(arg1: cl_mem, arg2: cl_image_info, arg3: usize,
                          arg4: *mut ::std::os::raw::c_void, arg5: *mut usize)
     -> cl_int;

    pub fn clSetMemObjectDestructorCallback(arg1: cl_mem,
                                            arg2:
                                                ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                               cl_mem,
                                                                                           arg2:
                                                                                               *mut ::std::os::raw::c_void)>,
                                            arg3: *mut ::std::os::raw::c_void)
     -> cl_int;

    pub fn clCreateSampler(arg1: cl_context, arg2: cl_bool,
                           arg3: cl_addressing_mode, arg4: cl_filter_mode,
                           arg5: *mut cl_int) -> cl_sampler;

    pub fn clRetainSampler(arg1: cl_sampler) -> cl_int;

    pub fn clReleaseSampler(arg1: cl_sampler) -> cl_int;

    pub fn clGetSamplerInfo(arg1: cl_sampler, arg2: cl_sampler_info,
                            arg3: usize, arg4: *mut ::std::os::raw::c_void,
                            arg5: *mut usize) -> cl_int;

    pub fn clCreateProgramWithSource(arg1: cl_context, arg2: cl_uint,
                                     arg3: *mut *const ::std::os::raw::c_char,
                                     arg4: *const usize, arg5: *mut cl_int)
     -> cl_program;

    pub fn clCreateProgramWithBinary(arg1: cl_context, arg2: cl_uint,
                                     arg3: *const cl_device_id,
                                     arg4: *const usize,
                                     arg5:
                                         *mut *const ::std::os::raw::c_uchar,
                                     arg6: *mut cl_int, arg7: *mut cl_int)
     -> cl_program;

    pub fn clCreateProgramWithBuiltInKernels(arg1: cl_context, arg2: cl_uint,
                                             arg3: *const cl_device_id,
                                             arg4:
                                                 *const ::std::os::raw::c_char,
                                             arg5: *mut cl_int) -> cl_program;

    pub fn clRetainProgram(arg1: cl_program) -> cl_int;

    pub fn clReleaseProgram(arg1: cl_program) -> cl_int;

    pub fn clBuildProgram(arg1: cl_program, arg2: cl_uint,
                          arg3: *const cl_device_id,
                          arg4: *const ::std::os::raw::c_char,
                          arg5:
                              ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                             cl_program,
                                                                         arg2:
                                                                             *mut ::std::os::raw::c_void)>,
                          arg6: *mut ::std::os::raw::c_void) -> cl_int;

    pub fn clCompileProgram(arg1: cl_program, arg2: cl_uint,
                            arg3: *const cl_device_id,
                            arg4: *const ::std::os::raw::c_char,
                            arg5: cl_uint, arg6: *const cl_program,
                            arg7: *mut *const ::std::os::raw::c_char,
                            arg8:
                                ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                               cl_program,
                                                                           arg2:
                                                                               *mut ::std::os::raw::c_void)>,
                            arg9: *mut ::std::os::raw::c_void) -> cl_int;

    pub fn clLinkProgram(arg1: cl_context, arg2: cl_uint,
                         arg3: *const cl_device_id,
                         arg4: *const ::std::os::raw::c_char, arg5: cl_uint,
                         arg6: *const cl_program,
                         arg7:
                             ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                            cl_program,
                                                                        arg2:
                                                                            *mut ::std::os::raw::c_void)>,
                         arg8: *mut ::std::os::raw::c_void, arg9: *mut cl_int)
     -> cl_program;

    pub fn clUnloadPlatformCompiler(arg1: cl_platform_id) -> cl_int;

    pub fn clGetProgramInfo(arg1: cl_program, arg2: cl_program_info,
                            arg3: usize, arg4: *mut ::std::os::raw::c_void,
                            arg5: *mut usize) -> cl_int;

    pub fn clGetProgramBuildInfo(arg1: cl_program, arg2: cl_device_id,
                                 arg3: cl_program_build_info, arg4: usize,
                                 arg5: *mut ::std::os::raw::c_void,
                                 arg6: *mut usize) -> cl_int;

    pub fn clCreateKernel(arg1: cl_program,
                          arg2: *const ::std::os::raw::c_char,
                          arg3: *mut cl_int) -> cl_kernel;

    pub fn clCreateKernelsInProgram(arg1: cl_program, arg2: cl_uint,
                                    arg3: *mut cl_kernel, arg4: *mut cl_uint)
     -> cl_int;

    pub fn clRetainKernel(arg1: cl_kernel) -> cl_int;

    pub fn clReleaseKernel(arg1: cl_kernel) -> cl_int;

    pub fn clSetKernelArg(arg1: cl_kernel, arg2: cl_uint, arg3: usize,
                          arg4: *const ::std::os::raw::c_void) -> cl_int;

    pub fn clGetKernelInfo(arg1: cl_kernel, arg2: cl_kernel_info, arg3: usize,
                           arg4: *mut ::std::os::raw::c_void,
                           arg5: *mut usize) -> cl_int;

    pub fn clGetKernelArgInfo(arg1: cl_kernel, arg2: cl_uint,
                              arg3: cl_kernel_arg_info, arg4: usize,
                              arg5: *mut ::std::os::raw::c_void,
                              arg6: *mut usize) -> cl_int;

    pub fn clGetKernelWorkGroupInfo(arg1: cl_kernel, arg2: cl_device_id,
                                    arg3: cl_kernel_work_group_info,
                                    arg4: usize,
                                    arg5: *mut ::std::os::raw::c_void,
                                    arg6: *mut usize) -> cl_int;

    pub fn clWaitForEvents(arg1: cl_uint, arg2: *const cl_event) -> cl_int;

    pub fn clGetEventInfo(arg1: cl_event, arg2: cl_event_info, arg3: usize,
                          arg4: *mut ::std::os::raw::c_void, arg5: *mut usize)
     -> cl_int;

    pub fn clCreateUserEvent(arg1: cl_context, arg2: *mut cl_int) -> cl_event;

    pub fn clRetainEvent(arg1: cl_event) -> cl_int;

    pub fn clReleaseEvent(arg1: cl_event) -> cl_int;

    pub fn clSetUserEventStatus(arg1: cl_event, arg2: cl_int) -> cl_int;

    pub fn clSetEventCallback(arg1: cl_event, arg2: cl_int,
                              arg3:
                                  ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                 cl_event,
                                                                             arg2:
                                                                                 cl_int,
                                                                             arg3:
                                                                                 *mut ::std::os::raw::c_void)>,
                              arg4: *mut ::std::os::raw::c_void) -> cl_int;

    pub fn clGetEventProfilingInfo(arg1: cl_event, arg2: cl_profiling_info,
                                   arg3: usize,
                                   arg4: *mut ::std::os::raw::c_void,
                                   arg5: *mut usize) -> cl_int;

    pub fn clFlush(arg1: cl_command_queue) -> cl_int;

    pub fn clFinish(arg1: cl_command_queue) -> cl_int;

    pub fn clEnqueueReadBuffer(arg1: cl_command_queue, arg2: cl_mem,
                               arg3: cl_bool, arg4: usize, arg5: usize,
                               arg6: *mut ::std::os::raw::c_void,
                               arg7: cl_uint, arg8: *const cl_event,
                               arg9: *mut cl_event) -> cl_int;

    pub fn clEnqueueReadBufferRect(arg1: cl_command_queue, arg2: cl_mem,
                                   arg3: cl_bool, arg4: *const usize,
                                   arg5: *const usize, arg6: *const usize,
                                   arg7: usize, arg8: usize, arg9: usize,
                                   arg10: usize,
                                   arg11: *mut ::std::os::raw::c_void,
                                   arg12: cl_uint, arg13: *const cl_event,
                                   arg14: *mut cl_event) -> cl_int;

    pub fn clEnqueueWriteBuffer(arg1: cl_command_queue, arg2: cl_mem,
                                arg3: cl_bool, arg4: usize, arg5: usize,
                                arg6: *const ::std::os::raw::c_void,
                                arg7: cl_uint, arg8: *const cl_event,
                                arg9: *mut cl_event) -> cl_int;

    pub fn clEnqueueWriteBufferRect(arg1: cl_command_queue, arg2: cl_mem,
                                    arg3: cl_bool, arg4: *const usize,
                                    arg5: *const usize, arg6: *const usize,
                                    arg7: usize, arg8: usize, arg9: usize,
                                    arg10: usize,
                                    arg11: *const ::std::os::raw::c_void,
                                    arg12: cl_uint, arg13: *const cl_event,
                                    arg14: *mut cl_event) -> cl_int;

    pub fn clEnqueueFillBuffer(arg1: cl_command_queue, arg2: cl_mem,
                               arg3: *const ::std::os::raw::c_void,
                               arg4: usize, arg5: usize, arg6: usize,
                               arg7: cl_uint, arg8: *const cl_event,
                               arg9: *mut cl_event) -> cl_int;

    pub fn clEnqueueCopyBuffer(arg1: cl_command_queue, arg2: cl_mem,
                               arg3: cl_mem, arg4: usize, arg5: usize,
                               arg6: usize, arg7: cl_uint,
                               arg8: *const cl_event, arg9: *mut cl_event)
     -> cl_int;

    pub fn clEnqueueCopyBufferRect(arg1: cl_command_queue, arg2: cl_mem,
                                   arg3: cl_mem, arg4: *const usize,
                                   arg5: *const usize, arg6: *const usize,
                                   arg7: usize, arg8: usize, arg9: usize,
                                   arg10: usize, arg11: cl_uint,
                                   arg12: *const cl_event,
                                   arg13: *mut cl_event) -> cl_int;

    pub fn clEnqueueReadImage(arg1: cl_command_queue, arg2: cl_mem,
                              arg3: cl_bool, arg4: *const usize,
                              arg5: *const usize, arg6: usize, arg7: usize,
                              arg8: *mut ::std::os::raw::c_void,
                              arg9: cl_uint, arg10: *const cl_event,
                              arg11: *mut cl_event) -> cl_int;

    pub fn clEnqueueWriteImage(arg1: cl_command_queue, arg2: cl_mem,
                               arg3: cl_bool, arg4: *const usize,
                               arg5: *const usize, arg6: usize, arg7: usize,
                               arg8: *const ::std::os::raw::c_void,
                               arg9: cl_uint, arg10: *const cl_event,
                               arg11: *mut cl_event) -> cl_int;

    pub fn clEnqueueFillImage(arg1: cl_command_queue, arg2: cl_mem,
                              arg3: *const ::std::os::raw::c_void,
                              arg4: *const usize, arg5: *const usize,
                              arg6: cl_uint, arg7: *const cl_event,
                              arg8: *mut cl_event) -> cl_int;

    pub fn clEnqueueCopyImage(arg1: cl_command_queue, arg2: cl_mem,
                              arg3: cl_mem, arg4: *const usize,
                              arg5: *const usize, arg6: *const usize,
                              arg7: cl_uint, arg8: *const cl_event,
                              arg9: *mut cl_event) -> cl_int;

    pub fn clEnqueueCopyImageToBuffer(arg1: cl_command_queue, arg2: cl_mem,
                                      arg3: cl_mem, arg4: *const usize,
                                      arg5: *const usize, arg6: usize,
                                      arg7: cl_uint, arg8: *const cl_event,
                                      arg9: *mut cl_event) -> cl_int;

    pub fn clEnqueueCopyBufferToImage(arg1: cl_command_queue, arg2: cl_mem,
                                      arg3: cl_mem, arg4: usize,
                                      arg5: *const usize, arg6: *const usize,
                                      arg7: cl_uint, arg8: *const cl_event,
                                      arg9: *mut cl_event) -> cl_int;

    pub fn clEnqueueMapBuffer(arg1: cl_command_queue, arg2: cl_mem,
                              arg3: cl_bool, arg4: cl_map_flags, arg5: usize,
                              arg6: usize, arg7: cl_uint,
                              arg8: *const cl_event, arg9: *mut cl_event,
                              arg10: *mut cl_int)
     -> *mut ::std::os::raw::c_void;

    pub fn clEnqueueMapImage(arg1: cl_command_queue, arg2: cl_mem,
                             arg3: cl_bool, arg4: cl_map_flags,
                             arg5: *const usize, arg6: *const usize,
                             arg7: *mut usize, arg8: *mut usize,
                             arg9: cl_uint, arg10: *const cl_event,
                             arg11: *mut cl_event, arg12: *mut cl_int)
     -> *mut ::std::os::raw::c_void;

    pub fn clEnqueueUnmapMemObject(arg1: cl_command_queue, arg2: cl_mem,
                                   arg3: *mut ::std::os::raw::c_void,
                                   arg4: cl_uint, arg5: *const cl_event,
                                   arg6: *mut cl_event) -> cl_int;

    pub fn clEnqueueMigrateMemObjects(arg1: cl_command_queue, arg2: cl_uint,
                                      arg3: *const cl_mem,
                                      arg4: cl_mem_migration_flags,
                                      arg5: cl_uint, arg6: *const cl_event,
                                      arg7: *mut cl_event) -> cl_int;

    pub fn clEnqueueNDRangeKernel(arg1: cl_command_queue, arg2: cl_kernel,
                                  arg3: cl_uint, arg4: *const usize,
                                  arg5: *const usize, arg6: *const usize,
                                  arg7: cl_uint, arg8: *const cl_event,
                                  arg9: *mut cl_event) -> cl_int;

    pub fn clEnqueueTask(arg1: cl_command_queue, arg2: cl_kernel,
                         arg3: cl_uint, arg4: *const cl_event,
                         arg5: *mut cl_event) -> cl_int;

    pub fn clEnqueueNativeKernel(arg1: cl_command_queue,
                                 arg2:
                                     ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                    *mut ::std::os::raw::c_void)>,
                                 arg3: *mut ::std::os::raw::c_void,
                                 arg4: usize, arg5: cl_uint,
                                 arg6: *const cl_mem,
                                 arg7: *mut *const ::std::os::raw::c_void,
                                 arg8: cl_uint, arg9: *const cl_event,
                                 arg10: *mut cl_event) -> cl_int;

    pub fn clEnqueueMarkerWithWaitList(arg1: cl_command_queue, arg2: cl_uint,
                                       arg3: *const cl_event,
                                       arg4: *mut cl_event) -> cl_int;

    pub fn clEnqueueBarrierWithWaitList(arg1: cl_command_queue, arg2: cl_uint,
                                        arg3: *const cl_event,
                                        arg4: *mut cl_event) -> cl_int;

    pub fn clGetExtensionFunctionAddressForPlatform(arg1: cl_platform_id,
                                                    arg2:
                                                        *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_void;

    pub fn clCreateImage2D(arg1: cl_context, arg2: cl_mem_flags,
                           arg3: *const cl_image_format, arg4: usize,
                           arg5: usize, arg6: usize,
                           arg7: *mut ::std::os::raw::c_void,
                           arg8: *mut cl_int) -> cl_mem;

    pub fn clCreateImage3D(arg1: cl_context, arg2: cl_mem_flags,
                           arg3: *const cl_image_format, arg4: usize,
                           arg5: usize, arg6: usize, arg7: usize, arg8: usize,
                           arg9: *mut ::std::os::raw::c_void,
                           arg10: *mut cl_int) -> cl_mem;

    pub fn clEnqueueMarker(arg1: cl_command_queue, arg2: *mut cl_event)
     -> cl_int;

    pub fn clEnqueueWaitForEvents(arg1: cl_command_queue, arg2: cl_uint,
                                  arg3: *const cl_event) -> cl_int;

    pub fn clEnqueueBarrier(arg1: cl_command_queue) -> cl_int;

    pub fn clUnloadCompiler() -> cl_int;

    pub fn clGetExtensionFunctionAddress(arg1: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_void;
}
