use super::*;

unsafe extern "C" fn robot_attack_s4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_AttackS4(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(robot_attack_s4_main_loop as *const () as _))
}

unsafe extern "C" fn robot_attack_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let combo = ComboModule::count(fighter.module_accessor) as i32;
    let s4_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s4_combo_max"), 0);
    let current_frame = MotionModule::frame(fighter.module_accessor);
    let energy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if combo < s4_combo_max && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            fighter.attack_s4_mtrans();
        }
    }
    else {
        fighter.attack_s4_mtrans();
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST) {
        if energy >= 860.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
            WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST)
    && current_frame < 16.0 {
        let rate = (16.0-current_frame)/(3.0+(16.0-current_frame));
        MotionModule::set_rate(fighter.module_accessor, rate);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL) {
        if current_frame >= 40.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into();
    }
    0.into()
}

unsafe extern "C" fn robot_attack_s4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
    0.into()
}

unsafe extern "C" fn robot_attack_hi4_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackHi4Start_common(hash40("attack_hi4").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(robot_attack_hi4_start_main_loop as *const () as _))
}

unsafe extern "C" fn robot_attack_hi4_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let energy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
    let current_frame = MotionModule::frame(fighter.module_accessor);
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let smash_hold = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    let log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if jump_attack_frame > 0 {
        if !StopModule::is_stop(fighter.module_accessor) {
            if fighter.sub_check_button_jump().get_bool() {
                let script_name = fighter.status_attack()[0xf40d7b92u64]["attack_hi4"].get_hash();
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, script_name, -1);
                fighter.change_status_jump_mini_attack(true.into());
                return 1.into();
            }
        }
    }
    if jump_attack_frame == 1 {
        if !fighter.global_table[IS_STOP].get_bool() {
            if log_attack_kind > 0 {
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST) {
        if energy > 96.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
            WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST)
    && current_frame < 8.0 {
        let rate = (8.0-current_frame)/(2.0+(8.0-current_frame));
        MotionModule::set_rate(fighter.module_accessor, rate);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD) {
        if smash_hold {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD.into(), true.into());
                return 1.into();
            } 
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4) {
        if smash_hold {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn robot_attack_hi4_start_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, frame, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
    }
    else {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
    }
    0.into()
}

unsafe extern "C" fn robot_attack_hi4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackHi4_common(hash40("attack_hi4").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(robot_attack_hi4_main_loop as *const () as _))
}

unsafe extern "C" fn robot_attack_hi4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = MotionModule::frame(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL) {
        if current_frame >= 40.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into();
    }
    0.into()
}

unsafe extern "C" fn robot_attack_hi4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
    0.into()
}

unsafe extern "C" fn robot_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn robot_special_hi_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = original_status(Main, fighter, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK);
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, 0.33);
    original(fighter)
}

pub fn install() {
    Agent::new("robot")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, robot_attack_s4_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, robot_attack_hi4_start_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4, robot_attack_hi4_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, robot_attack_s4_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI4, robot_attack_hi4_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, robot_attack_hi4_start_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, robot_special_hi_pre_status)
    .status(Main, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK, robot_special_hi_attack_main_status)
    .install()
    ;
}