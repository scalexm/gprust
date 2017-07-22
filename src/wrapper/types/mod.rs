//! A module for defining high-level types mapping to low-level OpenCL types.

/// Macro for high-level implementation of OpenCL bitfields boilerplate.
macro_rules! bitfield {
    ($name: ident, $($fun: ident => $ffi: expr),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name {
            bitfield: ::wrapper::ffi::cl_bitfield,
        }

        impl $name {
            $(
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

            unsafe fn ask_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                InformationResult::ask_info(function).map(|bitfield| $name { bitfield })
            }
        }
    };
}

/// Macro used in combination with `bitfield!` for defining a builder struct for a bitfield.
macro_rules! bitfield_builder {
    ([$name: ident, $builder: ident], $($fun: ident => $ffi: expr),*) => {
        bitfield!($name, $($fun => $ffi),*);

        pub struct $builder {
            bitfield: ::wrapper::ffi::cl_bitfield,
        }

        impl $builder {
            pub fn new() -> Self {
                $builder {
                    bitfield: 0,
                }
            }

            $(
            pub fn $fun(&mut self) -> &mut Self {
                self.bitfield |= $ffi;
                self
            }
            )*

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
    ($name: ident, $type: ty, $($field: ident => $ffi: pat),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
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

            unsafe fn ask_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                InformationResult::ask_info(function).map($name::from_ffi)
            }
        }

        impl ::wrapper::information::InformationResult<usize> for Vec<$name> {
            type Item = $type;

            unsafe fn ask_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                let vec: Result<Vec<_>> = InformationResult::ask_info(function);
                Ok(vec?.into_iter().filter(|val| *val != 0).map($name::from_ffi).collect())
            }
        }

        impl ::wrapper::information::InformationResult<::wrapper::ffi::cl_uint> for Vec<$name> {
            type Item = $type;

            unsafe fn ask_info<F>(function: F) -> Result<Self>
                where F: Fn(
                    ::wrapper::ffi::cl_uint,
                    *mut Self::Item,
                    *mut ::wrapper::ffi::cl_uint
                ) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                let vec: Result<Vec<_>> = InformationResult::ask_info(function);
                Ok(vec?.into_iter().filter(|val| *val != 0).map($name::from_ffi).collect())
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

            unsafe fn ask_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                InformationResult::ask_info(function).map($name::from_ffi)
            }
        }

        impl ::wrapper::information::InformationResult<usize> for Vec<$name> {
            type Item = $type;

            unsafe fn ask_info<F>(function: F) -> Result<Self>
                where F: Fn(usize, *mut Self::Item, *mut usize) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                let vec: Result<Vec<_>> = InformationResult::ask_info(function);
                Ok(vec?.into_iter().map($name::from_ffi).collect())
            }
        }

        impl ::wrapper::information::InformationResult<::wrapper::ffi::cl_uint> for Vec<$name> {
            type Item = $type;

            unsafe fn ask_info<F>(function: F) -> Result<Self>
                where F: Fn(
                    ::wrapper::ffi::cl_uint,
                    *mut Self::Item,
                    *mut ::wrapper::ffi::cl_uint
                ) -> ::wrapper::ffi::cl_int
            {
                use wrapper::information::InformationResult;
                let vec: Result<Vec<_>> = InformationResult::ask_info(function);
                Ok(vec?.into_iter().map($name::from_ffi).collect())
            }
        }
    };
}

pub mod platform;
pub mod device;
