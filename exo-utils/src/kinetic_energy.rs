#![allow(dead_code, improper_ctypes_definitions)]
use super::*;

//Credited to WuBoyTH and BluJay, the following functions handle kinetics, ultimately related to Momentum Transfer
#[repr(C)]
pub struct KineticEnergyVTable {
    pub destructor: extern "C" fn(&mut KineticEnergy),
    pub deleter: extern "C" fn(*mut KineticEnergy),
    pub unk: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub update: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub get_speed: extern "C" fn(&mut KineticEnergy) -> *mut PaddedVec2,
    pub initialize: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub get_some_flag: extern "C" fn(&mut KineticEnergy) -> bool,
    pub set_some_flag: extern "C" fn(&mut KineticEnergy, bool),
    pub setup_energy: extern "C" fn(&mut KineticEnergy, u32, &Vector3f, u64, &mut BattleObjectModuleAccessor),
    pub clear_energy: extern "C" fn(&mut KineticEnergy),
    pub unk2: extern "C" fn(&mut KineticEnergy),
    pub set_speed: extern "C" fn (&mut KineticEnergy, &Vector2f),
    pub mul_accel: extern "C" fn(&mut KineticEnergy, &Vector2f),
    // ...

}

#[repr(C)]
pub struct KineticEnergy {
    pub vtable: &'static KineticEnergyVTable,
    pub _x8: u64, // probably padding
    pub speed: PaddedVec2,
    pub rot_speed: PaddedVec2,
    pub enable: bool,
    pub unk2: [u8; 0xF], // probably padding
    pub accel: PaddedVec2,
    pub speed_max: PaddedVec2,
    pub speed_brake: PaddedVec2,
    pub speed_limit: PaddedVec2,
    pub _x80: u8,
    pub consider_ground_friction: bool,
    pub active_flag: bool, // no clue?
    pub _x83: u8,
    pub energy_reset_type: u32,
}

impl KineticEnergy {
    pub fn adjust_speed_for_ground_normal(speed: &PaddedVec2, boma: &mut BattleObjectModuleAccessor) -> PaddedVec2 {
        #[skyline::from_offset(0x47b4f0)]
        extern "C" fn adjust_speed_for_ground_normal_internal(speed: smash2::cpp::simd::Vector2, boma: &mut BattleObjectModuleAccessor) -> smash2::cpp::simd::Vector2;

        unsafe {
            let result = adjust_speed_for_ground_normal_internal(smash2::cpp::simd::Vector2{vec: [speed.x, speed.y]}, boma);
            PaddedVec2::new(result.vec[0], result.vec[1])
        }
    }
    pub fn process(&mut self, boma: &mut BattleObjectModuleAccessor) {
        unsafe {
            #[skyline::from_offset(0x47bf90)]
            extern "C" fn process_energy(energy: &mut KineticEnergy, boma: &mut BattleObjectModuleAccessor);

            process_energy(self, boma)
        }
    }
    pub fn update(&mut self, boma: &mut BattleObjectModuleAccessor) {
        (self.vtable.update)(self, boma)
    }
    pub fn get_speed<'a>(&'a mut self) -> &'a mut PaddedVec2 {
        unsafe {
            std::mem::transmute((self.vtable.get_speed)(self))
        }
    }
    pub fn initialize(&mut self, boma: &mut BattleObjectModuleAccessor) {
        (self.vtable.initialize)(self, boma)
    }
    pub fn get_some_flag(&mut self) -> bool {
        (self.vtable.get_some_flag)(self)
    }
    pub fn set_some_flag(&mut self, flag: bool) {
        (self.vtable.set_some_flag)(self, flag)
    }
    pub fn setup_energy(&mut self, reset_type: u32, incoming_speed: &Vector3f, some: u64, boma: &mut BattleObjectModuleAccessor) {
        (self.vtable.setup_energy)(self, reset_type, incoming_speed, some, boma)
    }
    pub fn clear_energy(&mut self) {
        (self.vtable.clear_energy)(self)
    }
    pub fn unk2(&mut self) {
        (self.vtable.unk2)(self)
    }
    pub fn set_speed(&mut self, speed: &Vector2f) {
        (self.vtable.set_speed)(self, speed)
    }
    pub fn mul_accel(&mut self, mul: &Vector2f) {
        (self.vtable.mul_accel)(self, mul)
    }
}

#[repr(C)]
pub struct FighterKineticEnergyControl {
    pub parent: KineticEnergy,
    pub lr: f32,
    pub accel_mul_x: f32,
    pub accel_add_x: f32,
    pub accel_mul_y: f32,
    pub accel_add_y: f32,
    pub _x9c: f32,
    pub _xa0: f32,
    pub unk: [u8; 4]
}

impl Deref for FighterKineticEnergyControl {
    type Target = KineticEnergy;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterKineticEnergyControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}