use super::*;

const REFLET_GIGAFIRE_VTABLE_ON_ATTACK_OFFSET: usize = 0x34d2160;

//Robin Arcfire On Attack Offset
#[skyline::hook(offset = REFLET_GIGAFIRE_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn reflet_gigafire_on_attack(vtable: u64, weapon: *mut smash::app::Weapon, log: u64) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let lua_module_fighter = get_fighter_common_from_accessor(&mut *boma);
    if WorkModule::is_flag(boma, WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_BOLGANONE) {
        WorkModule::off_flag(boma, WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_BOLGANONE);
        macros::EFFECT(lua_module_fighter, Hash40::new("finreflet_magic_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        StatusModule::change_status_request(boma, *WEAPON_REFLET_GIGAFIRE_STATUS_KIND_VANISH, false);
    }
    if WorkModule::is_flag(boma, WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        StatusModule::change_status_request(boma, *WEAPON_REFLET_GIGAFIRE_STATUS_KIND_VANISH, false);
    }
    original!()(vtable, weapon, log)
}

pub fn install() {
    skyline::install_hook!(reflet_gigafire_on_attack);
    //Removes the vanila effect call
    skyline::patching::Patch::in_text(0x34d2098).nop();
    //Removes the set int for the effect
    skyline::patching::Patch::in_text(0x34d20b4).nop();
}