//Collision Log
#[repr(C)]
pub struct CollisionLogScuffed {
    pub x00: *const u64,
    pub x08: *const u64,
    pub location: smash2::cpp::simd::Vector3,
    pub x20: u8,
    pub x21: u8,
    pub x22: u8,
    pub x23: u8,
    pub opponent_object_id: u32, //x24 - x27
    pub opponent_object_category: u8, //x28
    pub x29: u8,
    pub x2a: u8,
    pub x2b: u8,
    pub x2c: u8,
    pub x2d: u8,
    pub x2e: u8,
    pub collision_kind: u8, //x2f
    pub receiver_part_id: u8, //x30
    pub collider_part_id: u8, //x31
    pub receiver_id: u8, //x32
    pub collider_id: u8, //x33
    pub x35: bool
}

#[repr(C)]
pub struct ShieldAttackCollisionEvent {
    pub vtable: u64,
    pub shield_id: u32,
    pub unk: u8,
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub attack_module: u64,
    pub raw_power: f32,
    pub real_power: f32,
    pub collision_log: *const CollisionLogScuffed,
    pub group_index: i32,
    pub pos_x: f32,
    pub lr: f32,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: u8,
}