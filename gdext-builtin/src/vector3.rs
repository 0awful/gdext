use gdext_sys as sys;
use sys::{ffi_methods, real, GodotFfi};

//#[cfg(not(feature = "real_is_double"))]
type Inner = glam::f32::Vec3;
// #[cfg(feature = "real_is_double")]
// type Inner = glam::f64::DVec3;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vector3 {
    inner: Inner,
}

impl Vector3 {
    pub fn new(x: real, y: real, z: real) -> Self {
        Self {
            inner: Inner::new(x, y, z),
        }
    }
}

impl GodotFfi for Vector3 {
    ffi_methods! { type sys::GDNativeTypePtr = *mut Self; .. }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let Inner {x, y, z} = self.inner;
        //write!(f, "({x}, {y}, {z})")
        self.inner.fmt(f)
    }
}
