use super::*;

unsafe extern "C" fn ness_appeal_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if status_kind != *FIGHTER_STATUS_KIND_SMASH_APPEAL {
        if 0 < log_attack_kind {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
        CANCEL_FILL_SCREEN(fighter, 1, 4);
        CANCEL_FILL_SCREEN(fighter, 2, 4);
    }
    0.into()
}

unsafe extern "C" fn ness_attack_s4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    fighter.sub_AttackS4(true.into());
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    fighter.sub_shift_status_main(L2CValue::Ptr(ness_attack_s4_main_loop as *const () as _))
}

unsafe extern "C" fn ness_attack_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let combo = ComboModule::count(fighter.module_accessor) as i32;
    let s4_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s4_combo_max"), 0);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if combo < s4_combo_max
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            fighter.attack_s4_mtrans();
        }
    }
    else {
        fighter.attack_s4_mtrans();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_START) {
        shield!(fighter, *MA_MSC_SHIELD_SET_STATUS, *COLLISION_KIND_REFLECTOR, *FIGHTER_NESS_REFLECTOR_KIND_BAT, *SHIELD_STATUS_NORMAL, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_END) {
        shield!(fighter, *MA_MSC_SHIELD_SET_STATUS, *COLLISION_KIND_REFLECTOR, *FIGHTER_NESS_REFLECTOR_KIND_BAT, *SHIELD_STATUS_NONE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT)
    && fighter.global_table[CURRENT_FRAME].get_f32() < 21.0 {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 21.0, true, false, false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn ness_attack_s4_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
                EFFECT(fighter, Hash40::new("starman_smash"), Hash40::new("top"), 0, 16, -10, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                SoundModule::play_se(fighter.module_accessor, Hash40::new("se_ness_smash"), false, false, false, false, enSEType(0));
            }
        }
    }
    0.into()
}

unsafe extern "C" fn ness_special_guard_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn ness_special_guard_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ness_special_guard_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_guard"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(ness_special_guard_main_loop as *const () as _))
}

unsafe extern "C" fn ness_special_guard_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
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
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if frame == 120.0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP);
        WorkModule::set_int(fighter.module_accessor, 600, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
        //fighter.gimmick_flash();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn ness_special_guard_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ness_special_guard_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ness_special_guard_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ness_special_guard_burst_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn ness_special_guard_burst_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ness_special_guard_burst_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_guard_burst"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(ness_special_guard_burst_main_loop as *const () as _))
}

unsafe extern "C" fn ness_special_guard_burst_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn ness_special_guard_burst_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ness_special_guard_burst_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("ness_pkfl_hold"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_status_attack_up"), false, false);
    0.into()
}

unsafe extern "C" fn ness_special_guard_burst_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("ness_pkfl_hold"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_status_attack_up"), false, false);
    0.into()
}

pub fn install() {
    Agent::new("ness")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_APPEAL, ness_appeal_end_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, ness_attack_s4_main_status)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_S4, ness_attack_s4_check_attack_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_GUARD, ness_special_guard_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_GUARD, ness_special_guard_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_GUARD, ness_special_guard_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_GUARD, ness_special_guard_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_GUARD, ness_special_guard_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_GUARD, ness_special_guard_exit_status)
    .status(Pre, *FIGHTER_NESS_STATUS_KIND_SPECIAL_GUARD_BURST, ness_special_guard_burst_pre_status)
    .status(Init, *FIGHTER_NESS_STATUS_KIND_SPECIAL_GUARD_BURST, ness_special_guard_burst_init_status)
    .status(Main, *FIGHTER_NESS_STATUS_KIND_SPECIAL_GUARD_BURST, ness_special_guard_burst_main_status)
    .status(Exec, *FIGHTER_NESS_STATUS_KIND_SPECIAL_GUARD_BURST, ness_special_guard_burst_exec_status)
    .status(End, *FIGHTER_NESS_STATUS_KIND_SPECIAL_GUARD_BURST, ness_special_guard_burst_end_status)
    .status(Exit, *FIGHTER_NESS_STATUS_KIND_SPECIAL_GUARD_BURST, ness_special_guard_burst_exit_status)
    .install()
    ;
}