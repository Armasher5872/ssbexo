use super::*;

unsafe extern "C" fn luigi_obakyumu_win_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    0.into()
}

unsafe extern "C" fn luigi_obakyumu_win_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("win"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(luigi_obakyumu_win_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_obakyumu_win_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    if MotionModule::is_end(boma) {
        MotionModule::change_motion(boma, Hash40::new("win_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn luigi_obakyumu_win_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("luigi_obakyumu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_WIN, luigi_obakyumu_win_pre_status)
    .status(Main, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_WIN, luigi_obakyumu_win_main_status)
    .status(End, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_WIN, luigi_obakyumu_win_end_status)
    .install()
    ;
}