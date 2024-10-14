use super::*;

const PALUTENA_REFLECTIONBOARD_VTABLE_ON_ATTACK_OFFSET: usize = 0x348ce60;

//Palutena Reflection Board On Attack Offset
#[skyline::hook(offset = PALUTENA_REFLECTIONBOARD_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn palutena_reflectionboard_on_attack(vtable: u64, weapon: *mut smash::app::Weapon, log: u64) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let hit_count = WorkModule::get_int(boma, WEAPON_PALUTENA_REFLECTIONBOARD_INSTANCE_WORK_ID_INT_HIT_COUNT);
    WorkModule::inc_int(boma, WEAPON_PALUTENA_REFLECTIONBOARD_INSTANCE_WORK_ID_INT_HIT_COUNT);
    if hit_count > 4 {
        StatusModule::change_status_request(boma, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_BREAK, false);
    }
    original!()(vtable, weapon, log)
}

pub fn install() {
    skyline::install_hook!(palutena_reflectionboard_on_attack);
}