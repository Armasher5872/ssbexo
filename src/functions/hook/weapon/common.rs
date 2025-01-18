use super::*;

const COMMON_WEAPON_ATTACK_CALLBACK: usize = 0x33bdc30;
const COMMON_WEAPON_SHIELD_ATTACK_CALLBACK: usize = 0x346c8d0; //Used for when generic weapons hit something elses shield.

#[skyline::from_offset(COMMON_WEAPON_ATTACK_CALLBACK)]
pub unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) -> u64;

#[skyline::hook(offset = COMMON_WEAPON_SHIELD_ATTACK_CALLBACK)]
unsafe extern "C" fn common_weapon_shield_attack_callback(vtable: u64, weapon: *mut smash::app::Weapon, arg: u32) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_object_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_object_id as u32);
    let status_kind = StatusModule::status_kind(boma);
    let weapon_kind = (*weapon).battle_object.kind;
    if weapon_kind == *WEAPON_KIND_NESS_PK_FIRE as u32
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)
    && status_kind != *WEAPON_NESS_PK_FIRE_STATUS_KIND_PILLAR {
        if WorkModule::is_flag(owner_boma, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
            *(weapon as *mut bool).add(0x90) = true;
            (*boma).change_status_req(WEAPON_NESS_PK_FIRE_STATUS_KIND_PILLAR.into(), false.into());
        }
        else {
            *(weapon as *mut bool).add(0x90) = false;
        }
    }
    call_original!(vtable, weapon, arg)
}

pub fn install() {
    skyline::install_hook!(common_weapon_shield_attack_callback);
}