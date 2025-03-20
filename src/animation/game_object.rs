use std::ffi::c_void;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
#[repr(C)]
pub struct GameObject{
    pub ptr: *const c_void
}

impl From<*const c_void> for GameObject {
    fn from(value: *const c_void) -> Self {
        GameObject { ptr: value }
    }
}

impl From<GameObject> for *const c_void {
    fn from(value: GameObject) -> Self {
        value.ptr
    }
}
