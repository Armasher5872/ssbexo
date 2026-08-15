use super::*;

//Neutral Special Catch Jump Pre Status
unsafe extern "C" fn wario_special_n_catch_jump_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

//Neutral Special Catch Jump Init Status
unsafe extern "C" fn wario_special_n_catch_jump_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special Catch Jump Main Status
unsafe extern "C" fn wario_special_n_catch_jump_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    grabbed_anim_selector(fighter, "barrel_screw", 0.0, 0.0);
    MotionModule::change_motion(boma, Hash40::new("special_n_catch_jump"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_n_catch_jump_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_n_catch_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    handle_mash(fighter);
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Neutral Special Catch Jump Exec Status
unsafe extern "C" fn wario_special_n_catch_jump_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special Catch Jump End Status
unsafe extern "C" fn wario_special_n_catch_jump_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    wario_special_n_end(fighter, false);
    0.into()
}

//Neutral Special Catch Jump Exit Status
unsafe extern "C" fn wario_special_n_catch_jump_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    wario_special_n_exit(fighter, false);
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, wario_special_n_catch_jump_exit_status)
    .install()
    ;
}