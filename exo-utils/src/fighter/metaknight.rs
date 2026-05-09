use super::*;

pub unsafe extern "C" fn metaknight_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SHIELD_HIT);
    WorkModule::off_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_POWER);
    WorkModule::set_float(boma, 0.0, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_DAMAGE);
    WorkModule::set_int(boma, 0, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_HANDLE);
}

pub unsafe extern "C" fn beam_removal(weapon: &mut L2CWeaponCommon) {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("metaknight_beam"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}