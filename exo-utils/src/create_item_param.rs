use super::*;

#[repr(C)]
pub struct CreateItemParam {
    pub founder_pos: Vector4f,
    pub item_pos: Vector4f,
    pub item_kind: smash::app::ItemKind,
    pub another_battle_object_id: u32,
    pub variation_kind: i32,
    pub lr_dir: f32,
    pub owner_id: u32,
    pub unk_20: u32,
    pub pokeball_or_assist_kind: i32,
    pub unk_0: u64,
    pub weird_flag: u64,
    pub unk_1_weird: u64,
    pub unk_approx_0: f32,
    pub unk_02: f32
}