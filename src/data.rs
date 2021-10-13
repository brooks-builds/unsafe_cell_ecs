use std::cell::UnsafeCell;

#[derive(Debug)]
pub enum Data {
    F32(UnsafeCell<f32>),
}

impl Data {
    #[allow(clippy::mut_from_ref)]
    pub fn cast_f32(&self) -> &mut f32 {
        unsafe {
            match self {
                Self::F32(data) => &mut *(data.get()),
            }
        }
    }
}
