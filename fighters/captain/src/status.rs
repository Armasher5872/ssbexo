use super::*;

//Jab Pre Status
unsafe extern "C" fn captain_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_1 as u32, 0);
    0.into()
}

//Forward Tilt Pre Status
unsafe extern "C" fn captain_attack_s3_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32, 0);
    0.into()
}

//Up Tilt Pre Status
unsafe extern "C" fn captain_attack_hi3_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32, 0);
    0.into()
}

//Down Tilt Pre Status
unsafe extern "C" fn captain_attack_lw3_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32, 0);
    0.into()
}

//Forward Smash Start Pre Status
unsafe extern "C" fn captain_attack_s4_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_START_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_START_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_START_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, 0, 0, 0);
    0.into()
}

//Grab Pre Status
unsafe extern "C" fn captain_catch_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64, 0, 0, 0);
    0.into()
}

//Dash Grab Pre Status
unsafe extern "C" fn captain_catch_dash_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64, 0, 0, 0);
    0.into()
}

//Grab Pull Pre Status
unsafe extern "C" fn captain_catch_pull_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

//Dash Grab Pull Pre Status
unsafe extern "C" fn captain_catch_dash_pull_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

//Pummel Pre Status
unsafe extern "C" fn captain_catch_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, true, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_CATCH_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_CATCH as u32, 0);
    0.into()
}

//Down Special Pre Status
unsafe extern "C" fn captain_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn captain_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let hit_reduct_count = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("hit_reduct_count"));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_SP_BRAKE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_CLIFF_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_FALL_ONOFF);
    WorkModule::set_int(fighter.module_accessor, hit_reduct_count, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_REDUCTION_LEFT);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLOAT_FALCON_KICK_SPEED_COEFFICIENT);
    if situation_kind != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(captain_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn captain_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let kick_start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
    let lr = PostureModule::lr(fighter.module_accessor);
    let touch = if lr <= 0.0 {*GROUND_TOUCH_FLAG_LEFT} else {*GROUND_TOUCH_FLAG_RIGHT};
    if kick_start_situation != *SITUATION_KIND_GROUND {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK) {
                fighter.change_status(FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
            }
        }
    }
    else {
        if GroundModule::is_touch(fighter.module_accessor, touch as u32) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK) {
                fighter.change_status(FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END.into(), false.into());
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        if touch == *GROUND_TOUCH_FLAG_LEFT && GroundModule::is_touch(fighter.module_accessor, touch as u32) {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT != 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_WALL_JUMP.into(), false.into());
            }
        }
        if touch == *GROUND_TOUCH_FLAG_RIGHT && GroundModule::is_touch(fighter.module_accessor, touch as u32) {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT != 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_WALL_JUMP.into(), false.into());
            }
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
    }
    0.into()
}

//Down Special Wall Bounce
unsafe extern "C" fn captain_special_lw_wall_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_JUMP) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_throw"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(captain_special_lw_wall_end_main_loop as *const () as _))
}

unsafe extern "C" fn captain_special_lw_wall_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if !fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if get_sum_speed_y <= 0.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), false.into());
        }
    }
    if situation_kind != prev_situation_kind {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn captain_special_lw_wall_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_JUMP);
    0.into()
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK, captain_attack_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S3, captain_attack_s3_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_HI3, captain_attack_hi3_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_LW3, captain_attack_lw3_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S4_START, captain_attack_s4_start_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH, captain_catch_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_DASH, captain_catch_dash_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_PULL, captain_catch_pull_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, captain_catch_dash_pull_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_ATTACK, captain_catch_attack_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, captain_special_lw_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, captain_special_lw_main_status)
    .status(Main, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END, captain_special_lw_wall_end_main_status)
    .status(End, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END, captain_special_lw_wall_end_end_status)
    .install()
    ;
}