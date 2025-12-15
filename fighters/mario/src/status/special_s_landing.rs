use super::*;

//Side Special Landing Pre Status
unsafe extern "C" fn mario_special_s_landing_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

//Side Special Landing Init Status
unsafe extern "C" fn mario_special_s_landing_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.2, 0.0);
    0.into()
}

//Side Special Landing Main Status
unsafe extern "C" fn mario_special_s_landing_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_landing"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_s_landing_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_s_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Side Special Landing Exec Status
unsafe extern "C" fn mario_special_s_landing_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Side Special Landing End Status
unsafe extern "C" fn mario_special_s_landing_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Side Special Landing Exit Status
unsafe extern "C" fn mario_special_s_landing_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LANDING, mario_special_s_landing_pre_status)
    .status(Init, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LANDING, mario_special_s_landing_init_status)
    .status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LANDING, mario_special_s_landing_main_status)
    .status(Exec, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LANDING, mario_special_s_landing_exec_status)
    .status(End, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LANDING, mario_special_s_landing_end_status)
    .status(Exit, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LANDING, mario_special_s_landing_exit_status)
    .install()
    ;
}