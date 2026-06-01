use super::*;

pub unsafe extern "C" fn sludge_removal(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    let pos = *PostureModule::pos(boma);
    EffectModule::req(boma, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
    EffectModule::kill_kind(boma, Hash40::new("packun_poison_gas"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}