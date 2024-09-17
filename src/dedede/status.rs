use super::*;

unsafe extern "C" fn dedede_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    FighterUtil::cancel_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL));
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn dedede_special_n_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    GroundModule::set_no_cliff_stop_energy(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ID_NONE);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE) {
        FighterUtil::set_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL), Hash40::new("special_lw_max_face"));
    }
    0.into()
}

unsafe extern "C" fn dedede_special_n_swallow_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE) {
        FighterUtil::set_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL), Hash40::new("special_lw_max_face"));
    }
    0.into()
}

unsafe extern "C" fn dedede_special_n_shot_object_hit_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    end_special_n_shot_object_hit(global_fighter);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE) {
        FighterUtil::set_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL), Hash40::new("special_lw_max_face"));
    }
    0.into()    
}

//Handles eat_jump_1, eat_jump_2, eat_fall, eat_landing, eat_pass, eat_turn, eat_turn_air, eat_wait, eat_wait_fall, eat_wait_jump, eat_wait_pass, and eat_walk
unsafe extern "C" fn dedede_special_n_eat_common_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x322aadfea5), true);
    fun_710002dd40(fighter);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE) {
        FighterUtil::set_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL), Hash40::new("special_lw_max_face"));
    }
    0.into()
}

unsafe extern "C" fn fun_710002dd40(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHECK_DEAD_OFFSET_X);
    0.into()
}

unsafe extern "C" fn dedede_special_n_spit_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE) {
        FighterUtil::set_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL), Hash40::new("special_lw_max_face"));
    }
    0.into()
}

unsafe extern "C" fn dedede_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_SOUND);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn dedede_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn dedede_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT1);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fun_7100025330(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(dedede_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100025330(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT1) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT1);
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT1) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT1);
        }
    }
    0.into()
}

unsafe extern "C" fn dedede_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if !fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        fun_7100025330(fighter);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE) {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
        fighter.change_status(FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn dedede_special_lw_wait_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn dedede_special_lw_wait_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX_FLUSHING);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT1);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT2);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    if !StopModule::is_stop(fighter.module_accessor) {
        fun_71000249a0(fighter);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(fun_71000249a0 as *const () as _));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    fighter.sub_shift_status_main(L2CValue::Ptr(dedede_special_lw_wait_main_loop as *const () as _))
}

unsafe extern "C" fn fun_71000249a0(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_7100022210(fighter);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT1) {
            return 0.into();
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hold"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT1);
    }
    0.into()
}

unsafe extern "C" fn fun_7100022210(fighter: &mut L2CFighterCommon) -> L2CValue {
    let hold_count = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT);
    let charge_progress = WorkModule::get_int(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
    let hold_max_f = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("hold_max_f") as u64);
    let charge_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("charge_speed") as u64);
    if hold_count < hold_max_f {
        WorkModule::add_float(fighter.module_accessor, charge_speed, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT);
        WorkModule::add_int(fighter.module_accessor, 1, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
    }
    if hold_max_f <= hold_count
    || hold_max_f <= charge_progress as f32 {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX_FLUSHING) {
            EffectModule::req_common(fighter.module_accessor, Hash40::new("charge_max"), 0.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX_FLUSHING);
        }
    }
    0.into()
}

unsafe extern "C" fn dedede_special_lw_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let jet_charge_progress = WorkModule::get_int(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if !fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD) {
        if fighter.sub_check_command_guard().get_bool() {
            WorkModule::set_int(fighter.module_accessor, jet_charge_progress, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), false.into());
        }
    }
    if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE) {
        if fighter.is_cat_flag(Cat2::StickEscape) {
            WorkModule::set_int(fighter.module_accessor, jet_charge_progress, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), false.into());
        }
        if fighter.is_cat_flag(Cat2::StickEscapeF) {
            WorkModule::set_int(fighter.module_accessor, jet_charge_progress, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), false.into());
        }
        if fighter.is_cat_flag(Cat2::StickEscapeB) {
            WorkModule::set_int(fighter.module_accessor, jet_charge_progress, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), false.into());
        }
    }
    if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP) {
        if fighter.is_cat_flag(Cat1::Jump) || fighter.is_cat_flag(Cat1::JumpButton) {
            WorkModule::set_int(fighter.module_accessor, jet_charge_progress, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), false.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT2) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT2);
        }
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE);
        WorkModule::set_int(fighter.module_accessor, 120, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
        fighter.gimmick_flash();
        FighterUtil::set_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL), Hash40::new("special_lw_max_face"));
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn dedede_special_lw_wait_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let jet_charge_progress = WorkModule::get_int(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
    WorkModule::set_int(fighter.module_accessor, jet_charge_progress, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
    0.into()
}

unsafe extern "C" fn dedede_special_lw_fall_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn dedede_special_lw_fall_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor) {
        fun_7100022210(fighter);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(fun_7100022210 as *const () as _));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT1);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_fall"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    fighter.sub_shift_status_main(L2CValue::Ptr(dedede_special_lw_fall_main_loop as *const () as _))
}

unsafe extern "C" fn dedede_special_lw_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let jet_charge_progress = WorkModule::get_int(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if !fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        || fighter.is_cat_flag(Cat1::AirEscape) {
            WorkModule::set_int(fighter.module_accessor, jet_charge_progress, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), false.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT2) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT2);
        }
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE);
        WorkModule::set_int(fighter.module_accessor, 120, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
        fighter.gimmick_flash();
        FighterUtil::set_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL), Hash40::new("special_lw_max_face"));
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn dedede_special_lw_fall_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let jet_charge_progress = WorkModule::get_int(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
    WorkModule::set_int(fighter.module_accessor, jet_charge_progress, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
    0.into()
}

unsafe extern "C" fn dedede_special_lw_landing_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn dedede_special_lw_landing_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor) {
        fun_7100022210(fighter);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(fun_7100022210 as *const () as _));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_landing"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    fighter.sub_shift_status_main(L2CValue::Ptr(dedede_special_lw_landing_main_loop as *const () as _))
}

unsafe extern "C" fn dedede_special_lw_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if !fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT2) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_CONTINUE_MOT2);
        }
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE);
        WorkModule::set_int(fighter.module_accessor, 120, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
        fighter.gimmick_flash();
        FighterUtil::set_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL), Hash40::new("special_lw_max_face"));
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn dedede_special_lw_landing_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let jet_charge_progress = WorkModule::get_int(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
    WorkModule::set_int(fighter.module_accessor, jet_charge_progress, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
    0.into()
}

unsafe extern "C" fn dedede_special_lw_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn dedede_special_lw_attack_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn dedede_special_lw_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_max"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_max"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(dedede_special_lw_attack_main_loop as *const () as _))
}

unsafe extern "C" fn dedede_special_lw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if !fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_max"), 0.0, 1.0, false, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_max"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn dedede_special_lw_attack_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn dedede_special_lw_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    FighterUtil::cancel_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL));
    0.into()
}

pub fn install() {
    Agent::new("dedede")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, dedede_special_n_pre_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END, dedede_special_n_end_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SWALLOW, dedede_special_n_swallow_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT, dedede_special_n_shot_object_hit_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_JUMP1, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_JUMP2, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_FALL, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_LANDING, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_PASS, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_TURN, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_TURN_AIR, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_FALL, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_JUMP, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_PASS, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WALK, dedede_special_n_eat_common_end_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SPIT, dedede_special_n_spit_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, dedede_special_hi_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, dedede_special_lw_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, dedede_special_lw_main_status)
    .status(Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT, dedede_special_lw_wait_pre_status)
    .status(Main, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT, dedede_special_lw_wait_main_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT, dedede_special_lw_wait_end_status)
    .status(Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL, dedede_special_lw_fall_pre_status)
    .status(Main, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL, dedede_special_lw_fall_main_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL, dedede_special_lw_fall_end_status)
    .status(Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_LANDING, dedede_special_lw_landing_pre_status)
    .status(Main, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_LANDING, dedede_special_lw_landing_main_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_LANDING, dedede_special_lw_landing_end_status)
    .status(Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, dedede_special_lw_attack_pre_status)
    .status(Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, dedede_special_lw_attack_pre_status)
    .status(Init, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, dedede_special_lw_attack_init_status)
    .status(Main, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, dedede_special_lw_attack_main_status)
    .status(Exec, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, dedede_special_lw_attack_exec_status)
    .status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, dedede_special_lw_attack_end_status)
    .install()
    ;
}