use super::*;

//Neutral Special Charged Pre Status
unsafe extern "C" fn captain_special_n_charged_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

//Neutral Special Charged Init Status
unsafe extern "C" fn captain_special_n_charged_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

//Neutral Special Charged Main Status
unsafe extern "C" fn captain_special_n_charged_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_charged"), L2CValue::Hash40s("special_air_n_charged"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(captain_special_n_charged_main_loop as *const () as _))
}

//Neutral Special Charged Main Loop Status
unsafe extern "C" fn captain_special_n_charged_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_n_charged"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_n_charged"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(boma) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

//Neutral Special Charged Exec Status
unsafe extern "C" fn captain_special_n_charged_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special Charged End Status
unsafe extern "C" fn captain_special_n_charged_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special Charged Exit Status
unsafe extern "C" fn captain_special_n_charged_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_CHARGED, captain_special_n_charged_pre_status)
    .status(Init, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_CHARGED, captain_special_n_charged_init_status)
    .status(Main, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_CHARGED, captain_special_n_charged_main_status)
    .status(Exec, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_CHARGED, captain_special_n_charged_exec_status)
    .status(End, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_CHARGED, captain_special_n_charged_end_status)
    .status(Exit, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_CHARGED, captain_special_n_charged_exit_status)
    .install()
    ;
}