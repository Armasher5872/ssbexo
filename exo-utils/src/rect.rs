#[repr(simd)]
#[derive(Debug)]
pub struct Rect {
    vec: [f32;4]
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        (self.vec[0] <= x && x <= self.vec[1]) && (self.vec[3] <= y && y <= self.vec[2])
    }
    pub fn grow(&mut self, x: f32, y: f32) {
        self.vec[0] -= x;
        self.vec[1] += x;
        self.vec[2] += y;
        self.vec[3] -= y;
    }
}