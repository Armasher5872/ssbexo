#[repr(simd)]
#[derive(Debug)]
pub struct Rect {
    direction: [f32;4]
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        (x >= self.direction[0] && x <= self.direction[1]) && (y <= self.direction[2] && y >= self.direction[3])
    }
}