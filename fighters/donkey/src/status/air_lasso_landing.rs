use super::*;

//Zair Landing Pre Status
unsafe extern "C" fn donkey_zair_landing_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, true, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, 0, 0);
    0.into()
}

//Zair Landing Init Status
unsafe extern "C" fn donkey_zair_landing_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Zair Landing Main Status
unsafe extern "C" fn donkey_zair_landing_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("landing_heavy"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_zair_landing_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_zair_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if frame > 6.0 {
        CancelModule::enable_cancel(boma);
    }
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Zair Landing Exec Status
unsafe extern "C" fn donkey_zair_landing_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Zair Landing End Status
unsafe extern "C" fn donkey_zair_landing_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Landing();
    0.into()
}

//Zair Landing Exit Status
unsafe extern "C" fn donkey_zair_landing_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_landing_uniq_process_exit();
    0.into()
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_DONKEY_STATUS_KIND_AIR_LASSO_LANDING, donkey_zair_landing_pre_status)
    .status(Init, *FIGHTER_DONKEY_STATUS_KIND_AIR_LASSO_LANDING, donkey_zair_landing_init_status)
    .status(Main, *FIGHTER_DONKEY_STATUS_KIND_AIR_LASSO_LANDING, donkey_zair_landing_main_status)
    .status(Exec, *FIGHTER_DONKEY_STATUS_KIND_AIR_LASSO_LANDING, donkey_zair_landing_exec_status)
    .status(End, *FIGHTER_DONKEY_STATUS_KIND_AIR_LASSO_LANDING, donkey_zair_landing_end_status)
    .status(Exit, *FIGHTER_DONKEY_STATUS_KIND_AIR_LASSO_LANDING, donkey_zair_landing_exit_status)
    .install()
    ;
}