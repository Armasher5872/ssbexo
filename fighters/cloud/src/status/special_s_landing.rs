use super::*;

//Special S Landing Pre Status
unsafe extern "C" fn cloud_special_s_landing_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, 0, 0);
    0.into()
}

//Special S Landing Init Status
unsafe extern "C" fn cloud_special_s_landing_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Special S Landing Main Status
unsafe extern "C" fn cloud_special_s_landing_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_heavy"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_special_s_landing_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_special_s_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        if frame > 8.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    else {
        if frame > 6.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Special S Landing Exec Status
unsafe extern "C" fn cloud_special_s_landing_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Special S Landing End Status
unsafe extern "C" fn cloud_special_s_landing_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Landing();
    0.into()
}

//Special S Landing Exit Status
unsafe extern "C" fn cloud_special_s_landing_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_landing_uniq_process_exit();
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LANDING, cloud_special_s_landing_pre_status)
    .status(Init, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LANDING, cloud_special_s_landing_init_status)
    .status(Main, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LANDING, cloud_special_s_landing_main_status)
    .status(Exec, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LANDING, cloud_special_s_landing_exec_status)
    .status(End, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LANDING, cloud_special_s_landing_end_status)
    .status(Exit, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LANDING, cloud_special_s_landing_exit_status)
    .install()
    ;
}