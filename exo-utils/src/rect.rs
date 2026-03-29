#[repr(simd)]
#[derive(Clone, Copy)]
pub struct Rect {
    _vec: [f32;4]
}

impl Rect {
    pub fn x(self) -> f32 {
        unsafe { 
            core::intrinsics::simd::simd_extract(self, 0) 
        }
    }
    pub fn set_x(&mut self, value: f32) {
        unsafe {
            *self = core::intrinsics::simd::simd_insert(*self, 0, value);
        }
    }
    pub fn y(self) -> f32 {
        unsafe { 
            core::intrinsics::simd::simd_extract(self, 1) 
        }
    }
    pub fn set_y(&mut self, value: f32) {
        unsafe {
            *self = core::intrinsics::simd::simd_insert(*self, 1, value);
        }
    }
    pub fn z(self) -> f32 {
        unsafe { 
            core::intrinsics::simd::simd_extract(self, 2)
        }
    }
    pub fn set_z(&mut self, value: f32) {
        unsafe {
            *self = core::intrinsics::simd::simd_insert(*self, 2, value);
        }
    }
    pub fn w(self) -> f32 {
        unsafe { 
            core::intrinsics::simd::simd_extract(self, 3) 
        }
    }
    pub fn set_w(&mut self, value: f32) {
        unsafe {
            *self = core::intrinsics::simd::simd_insert(*self, 3, value);
        }
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        (self.x() <= x && x <= self.y()) && (self.w() <= y && y <= self.z())
    }
    pub fn grow(&mut self, x: f32, y: f32) {
        self.set_x(self.x() - x);
        self.set_y(self.y() + x);
        self.set_z(self.z() + y);
        self.set_w(self.w() - y);
    }
}