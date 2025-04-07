use super::*;

const NESS_PK_FIRE_ON_ATTACK: usize = 0x346c8d0;

#[skyline::hook(offset = NESS_PK_FIRE_ON_ATTACK)]
unsafe extern "C" fn ness_pk_fire_on_attack(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) {
    original!()(vtable, weapon, log);
    let boma = (*weapon).battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id as u32);
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && status_kind == *WEAPON_NESS_PK_FIRE_STATUS_KIND_SHOOT {
        if WorkModule::is_flag(owner_boma, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
            *(weapon as *mut bool).add(0x90) = true;
            (*boma).change_status_req(WEAPON_NESS_PK_FIRE_STATUS_KIND_PILLAR.into(), false.into());
        }
        else {
            *(weapon as *mut bool).add(0x90) = false;
        }
    }
}

pub fn install() {
    skyline::install_hook!(ness_pk_fire_on_attack);
}