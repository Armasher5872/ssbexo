use super::*;

pub unsafe extern "C" fn mat_removal(weapon: &mut L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    ReflectorModule::set_status_all(weapon.module_accessor, ShieldStatus(*SHIELD_STATUS_NONE), 0);
    EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}