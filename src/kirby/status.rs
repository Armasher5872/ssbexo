use super::*;

/*   WHEEL START STATUS SCRIPTS   */

unsafe extern "C" fn kirby_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_s_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_start"), L2CValue::Hash40s("special_air_s_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_s_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn kirby_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   GROUNDED WHEEL HOLD STATUS SCRIPTS   */

unsafe extern "C" fn kirby_special_s_wait_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT as i32, *FS_SUCCEEDS_KEEP_EFFECT);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_s_wait_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_wait_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_S_FLAG_HOLD_MAX);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT1);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT2);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT3);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_hold"), 1.0, 1.0, false, 0.0, false, false);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_s_wait_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_s_wait_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let wheel_hold_timer = WorkModule::get_int(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
    WorkModule::inc_int(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
    WorkModule::set_float(fighter.module_accessor, 1.0+(0.0042*(wheel_hold_timer as f32)), FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
    WHEEL_SPEED_UP[entry_id] = 2.5+(0.0167*(wheel_hold_timer as f32));
    if wheel_hold_timer >= 120 {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn kirby_special_s_wait_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_wait_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_wait_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   AERIAL WHEEL HOLD STATUS SCRIPTS   */

unsafe extern "C" fn kirby_special_s_fall_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_EFFECT);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_s_fall_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_fall_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_S_FLAG_HOLD_MAX);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT1);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT2);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT3);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_fall"), 1.0, 1.0, false, 0.0, false, false);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), false);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_s_fall_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_s_fall_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let wheel_hold_timer = WorkModule::get_int(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
    WorkModule::inc_int(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
    WorkModule::set_float(fighter.module_accessor, 0.75+(0.0042*(wheel_hold_timer as f32)), FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
    WHEEL_SPEED_UP[entry_id] = 1.0+(0.0167*(wheel_hold_timer as f32));
    if wheel_hold_timer >= 120 {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn kirby_special_s_fall_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_fall_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_fall_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   WHEEL TURN STATUS SCRIPTS   */

unsafe extern "C" fn kirby_special_s_turn_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_s_turn_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_turn_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    WorkModule::inc_int(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_turn"), 1.0, 1.0, false, 0.0, false, false);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s_turn"), false, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_s_turn_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_s_turn_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_AIR {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn kirby_special_s_turn_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_turn_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_turn_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   WHEEL DASH STATUS SCRIPTS   */

unsafe extern "C" fn kirby_special_s_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_s_attack_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        let speed = smash::phx::Vector3f{ x: WHEEL_SPEED_UP[entry_id], y: 0.0, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        let speed = smash::phx::Vector3f{ x: WHEEL_SPEED_UP[entry_id], y: -0.05, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_s_attack_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_s_attack_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
    let jump_count = WorkModule::get_int(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_JUMP_COUNT);
    let turn_count = WorkModule::get_int(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
    }
    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && frame < 30.0 {
        if stick_x < turn_stick_x
        && turn_count < 6 {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN.into(), false.into());
        }
        if fighter.check_jump_cancel(false, false)
        && situation_kind == *SITUATION_KIND_GROUND
        && jump_count < 2 {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP.into(), false.into());
        }
    }
    if frame < 30.0
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
    && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
    && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL);
        if situation_kind == *SITUATION_KIND_GROUND {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 60.0, true, true, false);
            macros::SET_SPEED_EX(fighter, WHEEL_SPEED_UP[entry_id]/2.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 50.0, true, true, false);
            macros::SET_SPEED_EX(fighter, WHEEL_SPEED_UP[entry_id]/2.0, -0.05, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            WorkModule::set_float(fighter.module_accessor, 17.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn kirby_special_s_attack_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let turn_count = WorkModule::get_int(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
    if turn_count > 48 {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
    }
    0.into()
}

unsafe extern "C" fn kirby_special_s_attack_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   WHEEL JUMP STATUS SCRIPTS   */

unsafe extern "C" fn kirby_special_s_jump_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT as i32, *FS_SUCCEEDS_KEEP_EFFECT);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_s_jump_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_jump_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::inc_int(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_JUMP_COUNT);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump"), 0.0, 1.0, false, 0.0, false, false);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s_jump"), false, 0.0);
    let speed = smash::phx::Vector3f{ x: WHEEL_SPEED_UP[entry_id]/2.5, y: 0.0, z: 0.0 };
    KineticModule::add_speed(fighter.module_accessor, &speed);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_s_jump_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_s_jump_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            WorkModule::set_float(fighter.module_accessor, 17.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_AIR {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn kirby_special_s_jump_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_jump_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_jump_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   FINAL CUTTER 1 STATUS SCRIPTS   */

unsafe extern "C" fn kirby_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

/*   FINAL CUTTER 2 STATUS SCRIPTS   */

unsafe extern "C" fn kirby_special_hi_2_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_2_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Vanilla
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_HIT_NUM);
    fun_71001a6740(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi2") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi2") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
    fun_710022ad50(fighter);
    /* START OF NEW ADDITIONS */
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let speed = smash::phx::Vector3f{ x: stick_x/2.0, y: 2.5, z: 0.0 };
    KineticModule::add_speed(fighter.module_accessor, &speed);
    /* END OF NEW ADDITIONS */
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_hi_2_loop as *const () as _))
}

unsafe extern "C" fn fun_71001a6740(fighter: &mut L2CFighterCommon) {
    //Vanilla
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
}

unsafe extern "C" fn fun_710022ad50(fighter: &mut L2CFighterCommon) {
    //Vanilla (to the best of my knowledge)
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        let finalcutter_air_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(finalcutter_air_mot), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(finalcutter_air_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    }
    else {
        let finalcutter_ground_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(finalcutter_ground_mot), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(finalcutter_ground_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    }
}

unsafe extern "C" fn kirby_special_hi_2_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Dictates Up Special Option Select
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    fighter.sub_off_passive_opponent(L2CValue::I32(*FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_TARGET_ID), L2CValue::I32(*FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_HIT_NUM), false.into());
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_CANCEL) {
        /* START OF NEW ADDITIONS */
        if [hash40("special_hi2"), hash40("special_air_hi2")].contains(&motion_kind)
        && (5.0..14.0).contains(&frame) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi3"), 0.0, 1.0, false, 0.0, false, false);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi31"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_hi31")
        && frame >= 11.0 {
            let speed = smash::phx::Vector3f{ x: 0.0, y: -0.5, z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speed);
        }
        if motion_kind == hash40("special_hi32") {
            let speed = smash::phx::Vector3f{ x: 0.0, y: -0.5, z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speed);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            if motion_kind != hash40("special_hi31") {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            }
        }
        /* END OF NEW ADDITIONS */
        if situation_kind == *SITUATION_KIND_GROUND {
            if frame > 1.0 {
                fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4.into(), false.into());
            }
        }
        return 1.into();
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

/*   STONE STATUS SCRIPTS   */

unsafe extern "C" fn kirby_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Makes it so Kirby retains his previous momentum before he activated Stone
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw") as i64, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw") as i64, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_AIR_MOT);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, false, -1);
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, false, ArticleOperationTarget(0));
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        fun_71001a6740(fighter);
    }
    else {
        let speed = smash::phx::Vector3f{ x: KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*PostureModule::lr(fighter.module_accessor), y: 0.0, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_lw_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_lw_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Vanilla (to the best of my knowledge) (╯°□°)╯︵ ┻━┻
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let effect_handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_STONE_EFFECT_HANDLE);
    if !fun_7100229ec0(fighter) {
        if situation_kind != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
        fun_7100229610(fighter);
    }
    if fun_710022a090(fighter) {
        fun_71001a6740(fighter);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_EFFECT_ONOFF) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_PREV_EFFECT_ONOFF);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_PREV_EFFECT_ONOFF);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_PREV_EFFECT_ONOFF) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_EFFECT_ONOFF) {
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_EFFECT_REMOVE, effect_handle);
            sv_module_access::effect(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_STONE_EFFECT_HANDLE);
        }
        else {
            //Normally called as fun_7100191510
            let stone_effect_handle = {fighter.clear_lua_stack(); lua_args!(fighter, Hash40::new("kirby_stone_s"), Hash40::new("rot"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, false, 0, 0, 0); sv_module_access::effect(fighter.lua_state_agent); fighter.pop_lua_stack(1)};
            WorkModule::set_int(fighter.module_accessor, stone_effect_handle.into(), *FIGHTER_KIRBY_STATUS_WORK_ID_INT_STONE_EFFECT_HANDLE);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_STONE_STONE.into(), false.into());
        return true.into();
    }    
    0.into()
}

unsafe extern "C" fn fun_7100229ec0(fighter: &mut L2CFighterCommon) -> bool {
    //Vanilla (to the best of my knowledge)
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND) {
            return true.into();
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END) {
            return false.into();
        }
    }
    else {
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn fun_7100229610(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Vanilla (to the best of my knowledge)
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let ground_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_GROUND_MOT);
    let air_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_AIR_MOT);
    if situation_kind != *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(air_mot), -1.0, 1.0, 0.0, false, false);
            return air_mot.into();
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(air_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
            return ground_mot.into();
        }
        else{
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(ground_mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
        }
    }
    return (*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT).into()
}

unsafe extern "C" fn fun_710022a090(fighter: &mut L2CFighterCommon) -> bool {
    //Vanilla (to the best of my knowledge)
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND) {
            if situation_kind == *SITUATION_KIND_GROUND {
                return true.into();
            }
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END) {
            return false.into();
        }
        else {
            if MotionModule::is_end(fighter.module_accessor) {
                return false.into();
            }
        }
    }
    else {
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn kirby_link_special_n_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bow_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    ArticleModule::change_status_exist(fighter.module_accessor, bow_id, *WN_LINK_BOW_STATUS_KIND_BACK);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        if ArticleModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = ArticleModule::get_int(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma, true);
            if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
            }
        }
    }
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(0));
    0.into()
}

pub fn install() {
    Agent::new("kirby")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_exit_status)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, kirby_special_s_wait_pre_status)
    .status(Init, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, kirby_special_s_wait_init_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, kirby_special_s_wait_main_status)
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, kirby_special_s_wait_exec_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, kirby_special_s_wait_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, kirby_special_s_wait_exit_status)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, kirby_special_s_fall_pre_status)
    .status(Init, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, kirby_special_s_fall_init_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, kirby_special_s_fall_main_status)
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, kirby_special_s_fall_exec_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, kirby_special_s_fall_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, kirby_special_s_fall_exit_status)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, kirby_special_s_turn_pre_status)
    .status(Init, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, kirby_special_s_turn_init_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, kirby_special_s_turn_main_status)
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, kirby_special_s_turn_exec_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, kirby_special_s_turn_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, kirby_special_s_turn_exit_status)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, kirby_special_s_attack_pre_status)
    .status(Init, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, kirby_special_s_attack_init_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, kirby_special_s_attack_main_status)
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, kirby_special_s_attack_exec_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, kirby_special_s_attack_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, kirby_special_s_attack_exit_status)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, kirby_special_s_jump_pre_status)
    .status(Init, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, kirby_special_s_jump_init_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, kirby_special_s_jump_main_status)
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, kirby_special_s_jump_exec_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, kirby_special_s_jump_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, kirby_special_s_jump_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, kirby_special_hi_pre_status)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, kirby_special_hi_2_pre_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, kirby_special_hi_2_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, kirby_special_lw_main_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_LINK_SPECIAL_N, kirby_link_special_n_exit_status)
    .install()
    ;
}