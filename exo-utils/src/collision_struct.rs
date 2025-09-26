use super::*;

//Collision Log
#[repr(C)]
pub struct CollisionLog {
    pub _next: *mut CollisionLog,
    pub _end: *mut CollisionLog,
    pub location: Vector3f,
    pub _padding_0: u32,
    pub _padding_1: u32,
    pub opponent_battle_object_id: u32,
    pub _padding_2: [u8;7],
    pub _collision_kind: u8,
    pub _receiver_part_id: u8,
    pub _collider_part_id: u8,
    pub _receiver_id: u8,
    pub _collider_id: u8,
    pub _padding_3: [u8;10]
}

#[repr(C)]
pub struct CollisionLogScuffed {
    pub x00: *const u64,
    pub x08: *const u64,
    pub location: smash2::cpp::simd::Vector3,
    pub x20: u8,
    pub x21: u8,
    pub x22: u8,
    pub x23: u8,
    pub opponent_object_id: u32,
    pub opponent_object_category: u8,
    pub x29: u8,
    pub x2a: u8,
    pub x2b: u8,
    pub x2c: u8,
    pub x2d: u8,
    pub x2e: u8,
    pub collision_kind: u8,
    pub receiver_part_id: u8,
    pub collider_part_id: u8,
    pub receiver_id: u8,
    pub collider_id: u8,
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

/*
impl ShieldAttackCollisionEvent {
    pub fn new(
        vtable: u64,
        shield_id: u32,
        unk: u8,
        unk1: u8,
        unk2: u8,
        unk3: u8,
        attack_module: u64,
        raw_power: f32,
        real_power: f32,
        collision_log: *const CollisionLogScuffed,
        group_index: i32,
        pos_x: f32,
        lr: f32,
        unk4: u8,
        unk5: u8,
        unk6: u8,
        unk7: u8
    ) -> Self {
        ShieldAttackCollisionEvent {
            vtable: vtable,
            shield_id: shield_id,
            unk: unk,
            unk1: unk1,
            unk2: unk2,
            unk3: unk3,
            attack_module: attack_module,
            raw_power: raw_power,
            real_power: real_power,
            collision_log: collision_log,
            group_index: group_index,
            pos_x: pos_x,
            lr: lr,
            unk4: unk4,
            unk5: unk5,
            unk6: unk6,
            unk7: unk7
        }
    }
}
*/