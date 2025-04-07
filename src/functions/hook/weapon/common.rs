const COMMON_WEAPON_ATTACK_CALLBACK: usize = 0x33bdc30;

#[skyline::from_offset(COMMON_WEAPON_ATTACK_CALLBACK)]
pub unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) -> u64;