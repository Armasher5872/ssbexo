use super::*;

unsafe extern "C" fn lucina_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CHARGE_MAX);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_start"), L2CValue::Hash40s("special_air_n_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if !StatusModule::is_changing(fighter.module_accessor) && StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX.into(), true.into());
    }
    0.into()
}

unsafe extern "C" fn lucina_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn lucina_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        macros::SET_SPEED_EX(fighter, 2.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    0.into()
}

unsafe extern "C" fn lucina_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND 
        && situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR
        && situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_OFF) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("gamemodel"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("back"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("front"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("side"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_mouth_capture"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("handfalchion"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_blink_talk"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_eye"), false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
        AreaModule::set_whole(fighter.module_accessor, false);
        JostleModule::set_status(fighter.module_accessor, false);
        CameraModule::set_status(fighter.module_accessor, CameraStatus{ _address: *CAMERA_STATUS_SLEEP as u8 }, 0);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_ON) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("gamemodel"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("back"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("front"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("side"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_mouth_capture"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("handfalchion"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_blink_talk"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_eye"), true);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AreaModule::set_whole(fighter.module_accessor, true);
        JostleModule::set_status(fighter.module_accessor, true);
        CameraModule::set_status(fighter.module_accessor, CameraStatus{ _address: *CAMERA_STATUS_NORMAL as u8 }, 0);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_OFF);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_ON);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_GRAVITY_ON) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_GRAVITY_ON);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucina_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("gamemodel"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_mouth_capture"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("handfalchion"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_blink_talk"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_eye"), true);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    AreaModule::set_whole(fighter.module_accessor, true);
    JostleModule::set_status(fighter.module_accessor, true);
    CameraModule::set_status(fighter.module_accessor, CameraStatus{ _address: *CAMERA_STATUS_NORMAL as u8 }, 0);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_OFF);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_ON);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_GRAVITY_ON);
    0.into()
}

unsafe extern "C" fn lucina_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("gamemodel"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_mouth_capture"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("handfalchion"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_blink_talk"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("lucina_eye"), true);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    AreaModule::set_whole(fighter.module_accessor, true);
    JostleModule::set_status(fighter.module_accessor, true);
    CameraModule::set_status(fighter.module_accessor, CameraStatus{ _address: *CAMERA_STATUS_NORMAL as u8 }, 0);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_OFF);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_ON);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_GRAVITY_ON);
    0.into()
}

unsafe extern "C" fn lucina_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("special_hi"), hash40("landing_frame"));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        fun_710001b600(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710001b600 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn fun_710001b600(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.set_situation(SITUATION_KIND_AIR.into());
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE_SET_ANGLE);
        }
    }
    0.into()
}

unsafe extern "C" fn lucina_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT_ON) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_LUCINA_STATUS_KIND_SPECIAL_HI2.into(), false.into());
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE) {
                let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if get_sum_speed_y < -0.001 {
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                    return 0.into();
                }
            }
        }
    }
    0.into()
}

unsafe extern "C" fn lucina_special_hi2_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn lucina_special_hi2_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT_ON);
    0.into()
}

unsafe extern "C" fn lucina_special_hi2_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_2"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_hi2_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_special_hi2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn lucina_special_hi2_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_hi2_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_hi2_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn lucina_special_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn lucina_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_start"), L2CValue::Hash40s("special_air_lw_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND && situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_LOOP.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucina_special_lw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_lw_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_lw_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_lw_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn lucina_special_lw_loop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn lucina_special_lw_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw"), L2CValue::Hash40s("special_air_lw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_lw_loop_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_special_lw_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND && situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
        }
    }
    WorkModule::set_float(fighter.module_accessor, 1.0+(0.01364327485*current_frame), FIGHTER_LUCINA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_POWER);
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn lucina_special_lw_loop_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_lw_loop_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_lw_loop_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_lw_hit_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn lucina_special_lw_hit_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let damage_multiplier = WorkModule::get_float(fighter.module_accessor, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_POWER);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    AttackModule::set_power_up(fighter.module_accessor, damage_multiplier);
    AttackModule::set_reaction_mul(fighter.module_accessor, damage_multiplier/1.75);
    0.into()
}

unsafe extern "C" fn lucina_special_lw_hit_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_attack"), L2CValue::Hash40s("special_air_lw_attack"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_lw_hit_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_special_lw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND && situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_attack"), -1.0, 1.0, 0.0, false, false);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_attack"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if StopModule::is_stop(fighter.module_accessor) {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_genesis_beam"), true, true);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucina_special_lw_hit_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucina_special_lw_hit_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    AttackModule::set_power_up(fighter.module_accessor, 1.0);
    AttackModule::set_reaction_mul(fighter.module_accessor, 1.0);
    WorkModule::set_float(fighter.module_accessor, 1.0, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_POWER);
    0.into()
}

unsafe extern "C" fn lucina_special_lw_hit_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    AttackModule::set_power_up(fighter.module_accessor, 1.0);
    AttackModule::set_reaction_mul(fighter.module_accessor, 1.0);
    WorkModule::set_float(fighter.module_accessor, 1.0, FIGHTER_LUCINA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_POWER);
    0.into()
}

pub fn install() {
    Agent::new("lucina")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, lucina_special_n_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, lucina_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, lucina_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, lucina_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, lucina_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, lucina_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, lucina_special_s_exit_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, lucina_special_hi_main_status)
    .status(Pre, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_HI2, lucina_special_hi2_pre_status)
    .status(Init, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_HI2, lucina_special_hi2_init_status)
    .status(Main, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_HI2, lucina_special_hi2_main_status)
    .status(Exec, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_HI2, lucina_special_hi2_exec_status)
    .status(End, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_HI2, lucina_special_hi2_end_status)
    .status(Exit, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_HI2, lucina_special_hi2_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucina_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucina_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucina_special_lw_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucina_special_lw_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucina_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucina_special_lw_exit_status)
    .status(Pre, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_LOOP, lucina_special_lw_loop_pre_status)
    .status(Init, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_LOOP, lucina_special_lw_loop_init_status)
    .status(Main, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_LOOP, lucina_special_lw_loop_main_status)
    .status(Exec, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_LOOP, lucina_special_lw_loop_exec_status)
    .status(End, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_LOOP, lucina_special_lw_loop_end_status)
    .status(Exit, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_LOOP, lucina_special_lw_loop_exit_status)
    .status(Pre, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, lucina_special_lw_hit_pre_status)
    .status(Init, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, lucina_special_lw_hit_init_status)
    .status(Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, lucina_special_lw_hit_main_status)
    .status(Exec, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, lucina_special_lw_hit_exec_status)
    .status(End, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, lucina_special_lw_hit_end_status)
    .status(Exit, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, lucina_special_lw_hit_exit_status)
    .install()
    ;
}