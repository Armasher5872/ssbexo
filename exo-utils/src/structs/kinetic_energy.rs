#![allow(improper_ctypes_definitions)] //Addresses Building warning: `extern` fn uses type `Vector2`, which is not FFI-safe
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
            PaddedVec2::new(result.x(), result.y())
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

#[repr(C)]
pub struct FighterKineticEnergyMotion {
    parent: KineticEnergy,
    pub lr: f32,
    pub angle: f32,
    pub angle_whole: f32,
    pub angle_intp_end: f32,
    pub angle_intp_frames_remaining: i32,
    pub speed_mul: f32,
    pub prev_speed: PaddedVec2,
    pub speed_mul_2nd: PaddedVec2,
    pub update_flag: bool,
    // ...
}

impl Deref for FighterKineticEnergyMotion {
    type Target = KineticEnergy;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterKineticEnergyMotion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl FighterKineticEnergyMotion {
    //Calls a MotionModule vtable function to update the trans move speed (2nd)
    pub fn update_trans_move_speed_2nd(boma: &mut BattleObjectModuleAccessor) {
        unsafe {
            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88/0x8);
            let motion_module_vtable = *(motion_module as *const *const u64);
            let function: extern "C" fn(u64) = std::mem::transmute(*motion_module_vtable.add(0x220/0x8));
            function(motion_module);
        }
    }
    //Checks if the motion (2nd) is updating the kinetic energy
    pub fn is_motion_2nd_updating_energy(boma: &mut BattleObjectModuleAccessor) -> bool {
        unsafe {
            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88/0x8);
            let motion_module_vtable = *(motion_module as *const *const u64);
            let function: extern "C" fn(u64) -> bool = std::mem::transmute(*motion_module_vtable.add(0x1f0/0x8));
            function(motion_module)
        }
    }
    //Checks if the motion is updating the kinetic energy
    pub fn is_main_motion_updating_energy(boma: &mut BattleObjectModuleAccessor) -> bool {
        unsafe {
            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88/0x8);
            let motion_module_vtable = *(motion_module as *const *const u64);
            let function: extern "C" fn(u64) -> bool = std::mem::transmute(*motion_module_vtable.add(0x1e8/0x8));
            function(motion_module)
        }
    }
    pub fn trans_move_speed_correct(boma: &mut BattleObjectModuleAccessor) -> Vector3f {
        unsafe {
            let func: extern "C" fn(&mut BattleObjectModuleAccessor) -> smash2::cpp::simd::Vector3 = std::mem::transmute(MotionModule::trans_move_speed as *const ());
            let vec = func(boma);
            Vector3f{x: vec.x(), y: vec.y(), z: vec.z()}
        }
    }
    pub fn trans_move_speed_2nd_correct(boma: &mut BattleObjectModuleAccessor) -> Vector3f {
        unsafe {
            let func: extern "C" fn(&mut BattleObjectModuleAccessor) -> smash2::cpp::simd::Vector3 = std::mem::transmute(MotionModule::trans_move_speed_2nd as *const ());
            let vec = func(boma);
            Vector3f{x: vec.x(), y: vec.y(), z: vec.z()}
        }
    }
    /// Sets some of the main behavioral values of the KineticEnergy and performs processing on it
    /// # Arguments
    /// * `accel` - The acceleration of the energy
    /// * `max_speed` - The maximum speed of the energy
    /// * `speed` - The speed that we are attempting to accelerate to
    pub fn set_values_and_process(&mut self, accel: PaddedVec2, max_speed: PaddedVec2, speed: PaddedVec2, boma: &mut BattleObjectModuleAccessor) {
        self.accel = accel;
        self.speed_max = max_speed;
        self.process(boma);
        self.active_flag = true;
        self.prev_speed = speed;
    }
    /// Gets the translation based on the specified energy reset type
    /// # Arguments
    /// * `boma` - The BattleObjectModuleAccessor
    /// * `reset_type` - The reset type of the current energy
    /// # Returns
    /// The translation as a Vec2
    pub fn get_translation_by_reset_type(boma: &mut BattleObjectModuleAccessor, reset_type: EnergyMotionResetType) -> PaddedVec2 {
        let translation = unsafe {
            if reset_type.is_2nd() {
                Self::update_trans_move_speed_2nd(boma);
                Self::trans_move_speed_2nd_correct(boma)
            } 
            else {
                MotionModule::update_trans_move_speed(boma);
                Self::trans_move_speed_correct(boma)
            }
        };
        PaddedVec2::new(translation.z, translation.y)
    }
    /// Checks if the animation is updating the kinetic energy, depending on the EnergyMotionResetType
    /// # Arguments
    /// * `boma` - The BattleObjectModuleAccessor
    /// * `reset_type` - The reset type of the current energy
    pub fn is_motion_updating_energy(boma: &mut BattleObjectModuleAccessor, reset_type: EnergyMotionResetType) -> bool {
        if reset_type.is_2nd() {
            Self::is_motion_2nd_updating_energy(boma)
        } 
        else {
            Self::is_main_motion_updating_energy(boma)
        }
    }
}