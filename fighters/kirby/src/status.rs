use super::*;

unsafe extern "C" fn kirby_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    0.into()
}

unsafe extern "C" fn kirby_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn kirby_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    0.into()
}

unsafe extern "C" fn kirby_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_2_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_2_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let speed = smash::phx::Vector3f{ x: stick_x/2.0, y: 2.5, z: 0.0 };
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_HIT_NUM);
    fun_71001a6740(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi2") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi2") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
    fun_710022ad50(fighter);
    KineticModule::add_speed(fighter.module_accessor, &speed);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_hi_2_main_loop as *const () as _))
}

unsafe extern "C" fn fun_710022ad50(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let finalcutter_ground_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
    let finalcutter_air_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
    if situation_kind != *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(finalcutter_air_mot), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(finalcutter_air_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(finalcutter_ground_mot), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(finalcutter_ground_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    }
}

unsafe extern "C" fn kirby_special_hi_2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let frame = MotionModule::frame(fighter.module_accessor);
    fighter.sub_off_passive_opponent(L2CValue::I32(*FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_TARGET_ID), L2CValue::I32(*FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_HIT_NUM), false.into());
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_CANCEL) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if frame > 1.0 {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5.into(), false.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_2_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5].contains(&status_kind) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(0));
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE_OFF, 5);
        sv_module_access::effect(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_2_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5].contains(&status_kind) {
        EffectModule::kill_all(fighter.module_accessor, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP as u32, false, true);
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_3_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_3_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_3_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_HIT_NUM);
    fun_71001a6740(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi3") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi3") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
    fun_710022ad50(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_hi_3_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_hi_3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    fighter.sub_off_passive_opponent(L2CValue::I32(*FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_TARGET_ID), L2CValue::I32(*FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_HIT_NUM), false.into());
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_CANCEL) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_3_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_hi_3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5].contains(&status_kind) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(0));
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE_OFF, 5);
        sv_module_access::effect(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_3_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5].contains(&status_kind) {
        EffectModule::kill_all(fighter.module_accessor, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP as u32, false, true);
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_4_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_4_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_hi_4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_71001a6740(fighter);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi4") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi4") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
    fun_710022ad50(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_hi_4_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_hi_4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_4_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_hi_4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5 {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(0));
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE_OFF, 5);
        sv_module_access::effect(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_4_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5 {
        EffectModule::kill_all(fighter.module_accessor, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP as u32, false, true);
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_71001a6740(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi5") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi5") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
    fun_710022ad50(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_hi_5_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_hi_5_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_hi_land_weak_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_land_weak_frame"));
    WorkModule::set_float(fighter.module_accessor, special_hi_land_weak_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(0));
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE_OFF, 5);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn fun_71001a6740(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
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

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, kirby_special_s_exit_status)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, kirby_special_hi_2_pre_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, kirby_special_hi_2_main_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, kirby_special_hi_2_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, kirby_special_hi_2_exit_status)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, kirby_special_hi_3_pre_status)
    .status(Init, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, kirby_special_hi_3_init_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, kirby_special_hi_3_main_status)
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, kirby_special_hi_3_exec_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, kirby_special_hi_3_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, kirby_special_hi_3_exit_status)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, kirby_special_hi_4_pre_status)
    .status(Init, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, kirby_special_hi_4_init_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, kirby_special_hi_4_main_status)
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, kirby_special_hi_4_exec_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, kirby_special_hi_4_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, kirby_special_hi_4_exit_status)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_pre_status)
    .status(Init, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_init_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_main_status)
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_exec_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_exit_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, kirby_special_lw_main_status)
    .install()
    ;
}