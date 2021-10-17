use std::mem;

#[repr(C)]
pub struct FFIVec<T> {
    pub ptr: *mut T,
    pub len: u64,
    pub cap: u64,
}
impl<T> From<Vec<T>> for FFIVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::new(vec)
    }
}
impl<T> FFIVec<T> {
    pub fn new(mut x: Vec<T>) -> Self {
        let s = Self {
            ptr: x.as_mut_ptr(),
            len: x.len() as u64,
            cap: x.capacity() as u64,
        };
        mem::forget(x);
        s
    }
}
