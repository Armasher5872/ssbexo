use super::*;

unsafe extern "C" fn luigi_obakyumu_special_lw_loop_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    0.into()
}

unsafe extern "C" fn luigi_obakyumu_special_lw_loop_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    if owner_lr == -1.0 {
        MotionModule::change_motion(boma, Hash40::new("special_lw_loop_l"), 0.0, 1.0, false, 0.0, false, false);
        MotionModule::set_flip(boma, false, true, true);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("special_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
    }
    weapon.fastshift(L2CValue::Ptr(luigi_obakyumu_special_lw_loop_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_obakyumu_special_lw_loop_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life > 1 {
        WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn luigi_obakyumu_special_lw_loop_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("luigi_obakyumu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_LOOP, luigi_obakyumu_special_lw_loop_pre_status)
    .status(Main, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_LOOP, luigi_obakyumu_special_lw_loop_main_status)
    .status(End, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_LOOP, luigi_obakyumu_special_lw_loop_end_status)
    .install()
    ;
}