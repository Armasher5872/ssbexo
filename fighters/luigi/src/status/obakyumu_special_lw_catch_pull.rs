use super::*;

unsafe extern "C" fn luigi_obakyumu_special_lw_catch_pull_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    0.into()
}

unsafe extern "C" fn luigi_obakyumu_special_lw_catch_pull_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    if WorkModule::is_flag(owner_boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_lw_catch_pull_plunger"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_lw_catch_pull"), 0.0, 1.0, false, 0.0, false, false);
    }
    if owner_lr == -1.0 {
        MotionModule::set_flip(weapon.module_accessor, false, true, true);
    }
    weapon.fastshift(L2CValue::Ptr(luigi_obakyumu_special_lw_catch_pull_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_obakyumu_special_lw_catch_pull_main_loop(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_obakyumu_special_lw_catch_pull_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("luigi_obakyumu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_PULL, luigi_obakyumu_special_lw_catch_pull_pre_status)
    .status(Main, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_PULL, luigi_obakyumu_special_lw_catch_pull_main_status)
    .status(End, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_PULL, luigi_obakyumu_special_lw_catch_pull_end_status)
    .install()
    ;
}