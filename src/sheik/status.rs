use super::*;

unsafe extern "C" fn sheik_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn sheik_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lr_stick_x"));
    let attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_frame"));
    PostureModule::set_stick_lr(fighter.module_accessor, lr_stick_x);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SHEIK_SPECIAL_LW_JUMP);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    WorkModule::set_int(fighter.module_accessor, attack_frame, *FIGHTER_SHEIK_STATUS_SPECIAL_LW_WORK_INT_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        sheik_special_s_substatus(fighter, true);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sheik_special_s_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(sheik_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn sheik_special_s_substatus(fighter: &mut L2CFighterCommon, dec_int: bool) -> L2CValue {
    if dec_int {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_SHEIK_STATUS_SPECIAL_LW_WORK_INT_FRAME);
    }
    0.into()
}

unsafe extern "C" fn sheik_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let special_lw_work_int_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SHEIK_STATUS_SPECIAL_LW_WORK_INT_FRAME);
    let jump_wall_check_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("jump_wall_check_start_frame"));
    let jump_wall_check_end_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("jump_wall_check_end_frame"));
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if StatusModule::is_changing(fighter.module_accessor) {
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into());
        }
        else {
            if special_lw_work_int_frame < 0 {
                if fighter.global_table[PAD_FLAG].get_i32() & (*FIGHTER_PAD_FLAG_ATTACK_TRIGGER | *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) != 0 {
                    fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
                }
            }
            if sheik_special_s_walljump_substatus(fighter, lr, jump_wall_check_start_frame, jump_wall_check_end_frame).get_bool() {
                return 1.into();
            }
            if !MotionModule::is_end(fighter.module_accessor) {
                return 0.into();
            }
            fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn sheik_special_s_walljump_substatus(fighter: &mut L2CFighterCommon, lr: f32, start_frame: f32, end_frame: f32) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    if (start_frame <= frame) && (frame <= end_frame) {
        if sheik_special_s_walljump_check_substatus(fighter, lr).get_bool() {
            let jump_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SHEIK_STATUS_SPECIAL_LW_WORK_INT_WALL_JUMP_NUM);
            let wall_jump_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("wall_jump_max"));
            if jump_num < wall_jump_max {
                WorkModule::inc_int(fighter.module_accessor, *FIGHTER_SHEIK_STATUS_SPECIAL_LW_WORK_INT_WALL_JUMP_NUM);
                if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN {
                    PostureModule::reverse_lr(fighter.module_accessor);
                    PostureModule::update_rot_y_lr(fighter.module_accessor);
                }
                fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN.into(), false.into());
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn sheik_special_s_walljump_check_substatus(fighter: &mut L2CFighterCommon, lr: f32) -> L2CValue {
    let mut ground_touch_flag: u32 = 0;
    if 0.0 > lr {
        ground_touch_flag = *GROUND_TOUCH_FLAG_LEFT as u32;
    }
    else {
        ground_touch_flag = *GROUND_TOUCH_FLAG_RIGHT as u32;
    }
    if GroundModule::is_wall_touch_line(fighter.module_accessor, ground_touch_flag) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SHEIK_STATUS_SPECIAL_LW_FLAG_TOUCH_WALL);
        return 1.into();
    }
    else {
        0.into()
    }
}

unsafe extern "C" fn sheik_special_s_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn sheik_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn sheik_special_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    0.into()
}

unsafe extern "C" fn sheik_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER) > 0 {
        fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_CANCEL.into(), false.into());
        0.into()
    }
    else {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw"), L2CValue::Hash40s("special_air_lw"), false.into());
        fighter.sub_shift_status_main(L2CValue::Ptr(sheik_special_lw_main_loop as *const () as _))
    }
}

unsafe extern "C" fn sheik_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn sheik_special_lw_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn sheik_special_lw_vanish_cancel_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn sheik_special_lw_vanish_cancel_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    AreaModule::set_whole(fighter.module_accessor, true);
    CameraModule::set_status(fighter.module_accessor, CameraStatus{ _address: *CAMERA_STATUS_NORMAL as u8 }, 0);
    JostleModule::set_status(fighter.module_accessor, true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("gamemodel"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("hair"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_eye"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_facen"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_openblink"), true);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HAS_VANISHED);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    0.into()
}

unsafe extern "C" fn sheik_special_lw_vanish_cancel_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_vanish_cancel"), L2CValue::Hash40s("special_air_lw_vanish_cancel"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(sheik_special_lw_vanish_cancel_main_loop as *const () as _))
}

unsafe extern "C" fn sheik_special_lw_vanish_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_vanish_cancel"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_vanish_cancel"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn sheik_special_lw_vanish_cancel_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S
    ];
    let transition_group_check = [
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK, 
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL
    ];
    for x in 0..transition_terms.len() {
        WorkModule::enable_transition_term(fighter.module_accessor, transition_terms[x]);
        WorkModule::unable_transition_term_forbid(fighter.module_accessor, transition_terms[x]);
    }
    for x in 0..transition_group_check.len() {
        WorkModule::enable_transition_term_group(fighter.module_accessor, transition_group_check[x]);
    }
    AreaModule::set_whole(fighter.module_accessor, true);
    CameraModule::set_status(fighter.module_accessor, CameraStatus{ _address: *CAMERA_STATUS_NORMAL as u8 }, 0);
    JostleModule::set_status(fighter.module_accessor, true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("gamemodel"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("hair"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_eye"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_facen"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_openblink"), true);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HAS_VANISHED);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER);
    0.into()
}

unsafe extern "C" fn sheik_special_lw_vanish_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn sheik_special_lw_vanish_attack_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    AreaModule::set_whole(fighter.module_accessor, true);
    CameraModule::set_status(fighter.module_accessor, CameraStatus{ _address: *CAMERA_STATUS_NORMAL as u8 }, 0);
    JostleModule::set_status(fighter.module_accessor, true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("gamemodel"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("hair"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_eye"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_facen"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_openblink"), true);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK);
    0.into()
}

unsafe extern "C" fn sheik_special_lw_vanish_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_vanish_attack"), L2CValue::Hash40s("special_air_lw_vanish_attack"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(sheik_special_lw_vanish_attack_main_loop as *const () as _))
}

unsafe extern "C" fn sheik_special_lw_vanish_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_vanish_attack"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_vanish_attack"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn sheik_special_lw_vanish_attack_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            let attacker_lr = PostureModule::lr(fighter.module_accessor);
            let defender_lr = PostureModule::lr(opponent_boma);
            if attacker_lr == defender_lr {
                WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BACK_HIT);
            }
            else {
                WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BACK_HIT);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn sheik_special_lw_vanish_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S
    ];
    let transition_group_check = [
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK, 
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL
    ];
    for x in 0..transition_terms.len() {
        WorkModule::enable_transition_term(fighter.module_accessor, transition_terms[x]);
        WorkModule::unable_transition_term_forbid(fighter.module_accessor, transition_terms[x]);
    }
    for x in 0..transition_group_check.len() {
        WorkModule::enable_transition_term_group(fighter.module_accessor, transition_group_check[x]);
    }
    AreaModule::set_whole(fighter.module_accessor, true);
    CameraModule::set_status(fighter.module_accessor, CameraStatus{ _address: *CAMERA_STATUS_NORMAL as u8 }, 0);
    JostleModule::set_status(fighter.module_accessor, true);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK);
    0.into()
}

pub fn install() {
    Agent::new("sheik")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, sheik_special_s_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, sheik_special_s_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, sheik_special_s_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, sheik_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, sheik_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, sheik_special_lw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, sheik_special_lw_end_status)
    .status(Pre, FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_CANCEL, sheik_special_lw_vanish_cancel_pre_status)
    .status(Init, FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_CANCEL, sheik_special_lw_vanish_cancel_init_status)
    .status(Main, FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_CANCEL, sheik_special_lw_vanish_cancel_main_status)
    .status(End, FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_CANCEL, sheik_special_lw_vanish_cancel_end_status)
    .status(Pre, FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_ATTACK, sheik_special_lw_vanish_attack_pre_status)
    .status(Init, FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_ATTACK, sheik_special_lw_vanish_attack_init_status)
    .status(Main, FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_ATTACK, sheik_special_lw_vanish_attack_main_status)
    .status(CheckAttack, FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_ATTACK, sheik_special_lw_vanish_attack_check_attack_status)
    .status(End, FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_ATTACK, sheik_special_lw_vanish_attack_end_status)
    .install()
    ;
}