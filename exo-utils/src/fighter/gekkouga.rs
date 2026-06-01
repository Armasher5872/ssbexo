use super::*;

pub unsafe extern "C" fn gekkouga_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_VERTICAL);
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_HI);
}

pub unsafe extern "C" fn mat_removal(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    let pos = *PostureModule::pos(boma);
    ReflectorModule::set_status_all(boma, ShieldStatus(*SHIELD_STATUS_NONE), 0);
    EffectModule::req(boma, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}