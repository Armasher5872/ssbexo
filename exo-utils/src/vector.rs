use super::*;

//Credited to HDR, used for adding 0 values of Vector2f and Vector3f
pub trait Vec2Ext {
    fn new(x: f32, y: f32) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
}

pub trait Vec3Ext {
    fn new(x: f32, y: f32, z: f32) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
}

impl Vec2Ext for smash::phx::Vector2f {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Vec3Ext for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct PaddedVec2 {
    pub x: f32,
    pub y: f32,
    pub padding: u64
}

impl PaddedVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x, y, padding: 0
        }
    }
    pub fn zeros() -> Self {
        Self {
            x: 0.0, y: 0.0, padding: 0
        }
    }
    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}