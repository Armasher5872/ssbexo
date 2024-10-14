use super::*;

//Mega-Man Leafshield On Attack Offset
unsafe extern "C" fn rockman_leafshield_on_attack(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("leafshield1"), 0, 0, 0, 0, 0, 0, 0.5, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("leafshield2"), 0, 0, 0, 0, 0, 0, 0.5, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("leafshield3"), 0, 0, 0, 0, 0, 0, 0.5, true);
    macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("leafshield4"), 0, 0, 0, 0, 0, 0, 0.5, true);
    *(weapon as *mut bool).add(0x90) = false;
    normal_weapon_hit_handler(vtable, weapon, log)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x5211270).data(rockman_leafshield_on_attack as u64);
}