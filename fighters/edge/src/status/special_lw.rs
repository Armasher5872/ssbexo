use super::*;

//Retaliation Stance Start Pre Status
unsafe extern "C" fn edge_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

//Retaliation Stance Start Init Status
unsafe extern "C" fn edge_special_lw_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Retaliation Stance Start Main Status
unsafe extern "C" fn edge_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw").into(), Hash40::new("special_air_lw").into(), false.into());
    edge_special_kinetic_handler(fighter, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if !StatusModule::is_changing(boma) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw").into(), Hash40::new("special_air_lw").into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_lw").into());
        edge_special_kinetic_handler(fighter, false);
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_LOOP.into(), false.into());
        return 0.into();
    }
    0.into()
}

//Retaliation Stance Start Exec Status
unsafe extern "C" fn edge_special_lw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Retaliation Stance Start End Status
unsafe extern "C" fn edge_special_lw_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Retaliation Stance Start Exit Status
unsafe extern "C" fn edge_special_lw_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, edge_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, edge_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, edge_special_lw_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, edge_special_lw_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, edge_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, edge_special_lw_exit_status)
    .install()
    ;
}