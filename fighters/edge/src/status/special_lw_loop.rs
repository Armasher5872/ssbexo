use super::*;

//Retaliation Stance Loop Pre Status
unsafe extern "C" fn edge_special_lw_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

//Retaliation Stance Loop Init Status
unsafe extern "C" fn edge_special_lw_loop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    0.into()
}

//Retaliation Stance Loop Main Status
unsafe extern "C" fn edge_special_lw_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    edge_enable_cancel_terms(boma);
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw_loop").into(), Hash40::new("special_air_lw_loop").into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_lw_loop_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_lw_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if !StatusModule::is_changing(boma) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw_loop").into(), Hash40::new("special_air_lw_loop").into(), true.into());
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    edge_try_cancel(fighter);
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_LOOP.into(), false.into());
        return 0.into();
    }
    0.into()
}

//Retaliation Stance Loop Exec Status
unsafe extern "C" fn edge_special_lw_loop_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Retaliation Stance Loop End Status
unsafe extern "C" fn edge_special_lw_loop_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Retaliation Stance Loop Exit Status
unsafe extern "C" fn edge_special_lw_loop_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_LOOP, edge_special_lw_loop_pre_status)
    .status(Init, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_LOOP, edge_special_lw_loop_init_status)
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_LOOP, edge_special_lw_loop_main_status)
    .status(Exec, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_LOOP, edge_special_lw_loop_exec_status)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_LOOP, edge_special_lw_loop_end_status)
    .status(Exit, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_LOOP, edge_special_lw_loop_exit_status)
    .install()
    ;
}