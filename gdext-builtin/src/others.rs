// Stub for various other built-in classes, which are currently incomplete, but whose types
// are required for codegen
use crate::GodotString;
use gdext_sys as sys;
use sys::{ffi_methods, GodotFfi};

macro_rules! impl_builtin_stub {
    ($Class:ident, $OpaqueTy:ident) => {
        #[repr(C)]
        pub struct $Class {
            opaque: sys::types::$OpaqueTy,
        }

        impl $Class {
            fn from_opaque(opaque: sys::types::$OpaqueTy) -> Self {
                Self { opaque }
            }
        }

        impl GodotFfi for $Class {
            ffi_methods! { type sys::GDNativeTypePtr = *mut Opaque; .. }
        }
    };
}

impl_builtin_stub!(Array, OpaqueArray);
impl_builtin_stub!(Dictionary, OpaqueDictionary);
impl_builtin_stub!(Transform2D, OpaqueTransform2D);
impl_builtin_stub!(Transform3D, OpaqueTransform3D);
impl_builtin_stub!(NodePath, OpaqueNodePath);

#[repr(C)]
pub struct StringName {
    opaque: sys::types::OpaqueStringName,
}
impl StringName {
    fn from_opaque(opaque: sys::types::OpaqueStringName) -> Self {
        Self { opaque }
    }

    ffi_methods! {
        type sys::GDNativeStringNamePtr = *mut Opaque;

        fn from_string_sys = from_sys;
        fn from_string_sys_init = from_sys_init;
        fn string_sys = sys;
        fn write_string_sys = write_sys;
    }

    #[doc(hidden)]
    pub fn leak_string_sys(self) -> sys::GDNativeStringNamePtr {
        let ptr = self.string_sys();
        std::mem::forget(self);
        ptr
    }
}
impl GodotFfi for StringName {
    ffi_methods! { type sys::GDNativeTypePtr = *mut Opaque; .. }
}
impl Default for StringName {
    fn default() -> Self {
        unsafe {
            Self::from_sys_init(|self_ptr| {
                let ctor = sys::method_table().string_name_construct_default;
                ctor(self_ptr, std::ptr::null_mut());
            })
        }
    }
}
impl From<&GodotString> for StringName {
    fn from(s: &GodotString) -> Self {
        unsafe {
            Self::from_sys_init(|self_ptr| {
                let ctor = sys::method_table().string_name_from_string;
                let args = [s.sys()];
                ctor(self_ptr, args.as_ptr());
            })
        }
    }
}
impl From<&StringName> for GodotString {
    fn from(s: &StringName) -> Self {
        unsafe {
            Self::from_sys_init(|self_ptr| {
                let ctor = sys::method_table().string_from_string_name;
                let args = [s.sys()];
                ctor(self_ptr, args.as_ptr());
            })
        }
    }
}
