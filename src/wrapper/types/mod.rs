//! A module for defining high-level types mapping to low-level OpenCL types.

/// Macro for high-level implementation of OpenCL bitfields boilerplate.
macro_rules! bitfield {
    ($name: ident, $type: expr, $([$fun: ident, $fun_name: expr] => $ffi: expr),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[doc="High-level bitfield mapping to `"] #[doc=$type] #[doc="`."]
        pub struct $name {
            bitfield: ::wrapper::ffi::cl_bitfield,
        }

        impl $name {
            $(
            #[doc="Return `true` if `"] #[doc=$fun_name] #[doc="` bit is set."]
            pub fn $fun(&self) -> bool {
                self.bitfield & $ffi == $ffi
            }
            )*
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_struct(stringify!($name))
                $(
                 .field(stringify!($fun), &self.$fun())
                )*
                 .finish()
            }
        }

        impl ::wrapper::information::InformationResult<usize> for $name {
            type Item = ::wrapper::ffi::cl_bitfield;

            unsafe fn get_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                InformationResult::get_info(function).map(|bitfield| $name { bitfield })
            }
        }
    };
}

/// Macro used in combination with `bitfield!` for defining a builder struct for a bitfield.
macro_rules! bitfield_builder {
    ([$name: ident, $builder: ident, $name_expr: expr], $type: expr, $([$fun: ident, $fun_name: expr] => $ffi: expr),*) => {
        bitfield!($name, $type, $([$fun, $fun_name] => $ffi),*);

        #[doc="Builder pattern struct for `"] #[doc=$name_expr] #[doc="`."]
        pub struct $builder {
            bitfield: ::wrapper::ffi::cl_bitfield,
        }

        impl $name {
            /// Return a zeroed bitfield.
            pub fn new() -> Self {
                $name {
                    bitfield: 0,
                }
            }
        }

        impl $builder {
            /// Initialize the builder with a zeroed bitfield.
            pub fn new() -> Self {
                $builder {
                    bitfield: 0,
                }
            }

            $(
            #[doc="Set `"] #[doc=$fun_name] #[doc="` bit."]
            pub fn $fun(&mut self) -> &mut Self {
                self.bitfield |= $ffi;
                self
            }
            )*

            #[doc="Output a `"] #[doc=$name_expr] #[doc="` bitfield."]
            pub fn finish(&self) -> $name {
                $name {
                    bitfield: self.bitfield,
                }
            }
        }
    };
}

/// Macro for high-level implementation of OpenCL enums boilerplate.
macro_rules! enumz {
    ($name: ident, $type: ty, $type_expr: expr,  $($field: ident => [$ffi: pat, $ffi_name: expr]),*) => {
        #[doc="High-level enum mapping to `"] #[doc=$type_expr] #[doc="`."]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
            #[doc="High-level variant for `"] #[doc=$ffi_name] #[doc="`."]
            $field
            ),*
        }

        impl $name {
            fn from_ffi(value: $type) -> Self {
                match value {
                    $(
                    $ffi => $name::$field,
                    )*
                    other => panic!("unexpected enum value: {}", other),
                }
            }
        }

        impl ::wrapper::information::InformationResult<usize> for $name {
            type Item = $type;

            unsafe fn get_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                InformationResult::get_info(function).map($name::from_ffi)
            }
        }
    };
}

/// Macro for implementing `InformationResult` for types which map to a low-level OpenCL type
/// through a `from_ffi` function.
macro_rules! map_ffi_impl {
    ($name: ident, $type: ty) => {
        impl ::wrapper::information::InformationResult<usize> for $name {
            type Item = $type;

            unsafe fn get_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;

                // Always retain when using `InformationResult`.
                InformationResult::get_info(function).map(|val| $name::from_ffi(val, true))
            }
        }

        impl ::wrapper::information::InformationResult<usize> for Vec<$name> {
            type Item = $type;

            unsafe fn get_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                let vec: Result<Vec<_>> = InformationResult::get_info(function);
                Ok(vec?.into_iter().map(|val| $name::from_ffi(val, true)).collect())
            }
        }

        impl ::wrapper::information::InformationResult<::wrapper::ffi::cl_uint> for Vec<$name> {
            type Item = $type;

            unsafe fn get_info<F>(function: F) -> Result<Self>
                where F: Fn(
                    ::wrapper::ffi::cl_uint,
                    *mut Self::Item,
                    *mut ::wrapper::ffi::cl_uint
                ) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                let vec: Result<Vec<_>> = InformationResult::get_info(function);
                Ok(vec?.into_iter().map(|val| $name::from_ffi(val, true)).collect())
            }
        }
    };
}

macro_rules! general_info_impl {
    ($trait: ident, $id_type: ty, $type: ident, $result: ty, $id: expr, $id_name: expr) => {
        #[doc="Marker type mapping to `"] #[doc=$id_name] #[doc="`."]
        pub struct $type;

        impl Information<$id_type> for $type {
            type Result = $result;

            fn id() -> $id_type {
                $id
            }
        }

        impl $trait for $type { }
    };
}

pub mod platform;
pub mod device;
pub mod context;
pub mod command_queue;
pub mod mem;
pub mod program;
