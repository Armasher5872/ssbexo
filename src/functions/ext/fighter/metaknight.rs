use super::*;

//Metaknight Galaxia Beam Functions

pub unsafe extern "C" fn should_remove_galaxia_on_hit(weapon: &mut L2CWeaponCommon) -> bool {
    if AttackModule::is_infliction_status(weapon.module_accessor, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT)
    || StopModule::is_stop(weapon.module_accessor)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK) {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn galaxia_beam_removal(weapon: &mut L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &NONE_VECTOR, 1.0, 0, -1, false, 0);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}

pub unsafe extern "C" fn galaxia_beam_hit_removal(weapon: &mut L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(weapon.module_accessor, Hash40::new("miiswordsman_hensoku_hit"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &NONE_VECTOR, 1.0, 0, -1, false, 0);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}