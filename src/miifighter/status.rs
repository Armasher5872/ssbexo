use super::*;

//Neutral Special

pub unsafe extern "C" fn miifighter_special_n3_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_n3_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_n3_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n3"), L2CValue::Hash40s("special_air_n3"), false.into());
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_03 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_03 - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_n3_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_n3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if fighter.sub_air_check_fall_common().get_bool() || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n3"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n3"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_RISE.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_n3_rise_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_n3_rise_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n3_rise"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_n3_rise_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_n3_rise_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if fighter.sub_air_check_fall_common().get_bool() || fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_LAND.into(), false.into());
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n3_rise"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_DIVE.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_n3_dive_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_n3_dive_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::SET_SPEED_EX(fighter, 3.0, -6.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_n3_dive_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n3_dive"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_n3_dive_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_n3_dive_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_LAND.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_n3_land_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_n3_land_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n3_land"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_n3_land_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_n3_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

//Side Special

pub unsafe extern "C" fn miifighter_special_s1_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let s1_speed_coef = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("s1_speed_coef"));
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_SHIELD_HIT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT_CHECK_ONOFF);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_CLIFF_FALL_ONOFF);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_GRAVITY_ONOFF);
    WorkModule::set_float(fighter.module_accessor, s1_speed_coef, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if situation_kind != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1_start"), 0.0, 1.0, false, 0.0, false, false);
        fun_7100020d60(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        if !StopModule::is_stop(fighter.module_accessor) {
            fun_7100020e00(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_7100020e00 as *const () as _));
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s1_start"), 0.0, 1.0, false, 0.0, false, false);
        fun_7100020b40(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        if !StopModule::is_stop(fighter.module_accessor) {
            fun_7100020be0(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_7100020be0 as *const () as _));
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04 - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_s1_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100020d60(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
}

unsafe extern "C" fn fun_7100020e00(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    if !bool_check.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_GRAVITY_ONOFF) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    0.into()
}

unsafe extern "C" fn fun_7100020b40(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
}

unsafe extern "C" fn fun_7100020be0(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    if !bool_check.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_CLIFF_FALL_ONOFF) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }
    0.into()
}

unsafe extern "C" fn miifighter_special_s1_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION);
    let s1_start_motion_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("s1_start_motion_speed_mul"));
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if start_situation == *SITUATION_KIND_AIR
    && situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    if start_situation == *SITUATION_KIND_GROUND 
    && situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT) {
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END.into(), false.into());
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_SHIELD_HIT) {
            smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_SHIELD_HIT);
            fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END.into(), false.into());
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_MOTION_SPEED_MUL) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_MOTION_SPEED_MUL);
            sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, s1_start_motion_speed_mul);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_s1_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION);
    if start_situation != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1_end"), 0.0, 1.0, false, 0.0, false, false);
        fun_7100020d60(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s1_end"), 0.0, 1.0, false, 0.0, false, false);
        fun_7100020b40(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING);
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_s1_end_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_s1_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_START_SITUATION);
    fighter.sub_off_passive_opponent(FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_HIT_OBJECT_ID.into(), FIGHTER_MIIFIGHTER_STATUS_WORK_ID_INT_100KICK_HIT_OBJECT_NUM.into(), true.into());
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if start_situation == *SITUATION_KIND_AIR
    && situation_kind == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    if start_situation == *SITUATION_KIND_GROUND 
    && situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

//Down Special

pub unsafe extern "C" fn miifighter_special_lw1_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw1"), L2CValue::Hash40s("special_air_lw1"), false.into());
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10 - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_lw1_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_lw1_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let armor = WorkModule::get_float(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_ARMOR);
    if fighter.sub_air_check_fall_common().get_bool() || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw1"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw1"), -1.0, 1.0, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_ACTIVE_ARMOR) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, armor);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_DASH_CANCEL) {
        if fighter.is_cat_flag(Cat1::Dash) && situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_CHARGE.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_charge_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_charge_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_charge_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw1_charge"), L2CValue::Hash40s("special_air_lw1_charge"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_lw1_charge_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_lw1_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let armor = WorkModule::get_float(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_ARMOR);
    if fighter.sub_air_check_fall_common().get_bool() || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw1_charge"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw1_charge"), -1.0, 1.0, 0.0, false, false);
    }
    if current_frame == 30.0 {
        WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_ARMOR_CRUSHING_THUNDER_KICK_ATTRIBUTE);
    }
    WorkModule::set_float(fighter.module_accessor, 12.0+(12.0*current_frame/60.0), FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_DAMAGE);
    WorkModule::set_float(fighter.module_accessor, armor+1.5, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_ARMOR);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_ACTIVE_ARMOR) {
        if current_frame < 40.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, armor);
        }
        else {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_DASH_CANCEL) {
        if fighter.is_cat_flag(Cat1::Dash) && situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), false.into());
        }
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_ARMOR_CRUSHING_THUNDER_KICK_ATTRIBUTE);
        fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_attack_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw1_attack"), L2CValue::Hash40s("special_air_lw1_attack"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_lw1_attack_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_lw1_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw1_attack"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw1_attack"), -1.0, 1.0, 0.0, false, false);
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

pub unsafe extern "C" fn miifighter_special_lw1_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 12.0, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_DAMAGE);
    WorkModule::set_float(fighter.module_accessor, 10.0, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_ARMOR);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_ARMOR_CRUSHING_THUNDER_KICK_ATTRIBUTE);
    0.into()
}

pub unsafe extern "C" fn miifighter_special_lw1_attack_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 12.0, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_DAMAGE);
    WorkModule::set_float(fighter.module_accessor, 10.0, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_ARMOR);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_ARMOR_CRUSHING_THUNDER_KICK_ATTRIBUTE);
    0.into()
}

unsafe extern "C" fn miifighter_special_lw3_throw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, module_accessor);
    if situation_kind == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    0.into()
}

unsafe extern "C" fn miifighter_special_lw3_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let get_node_object_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    let counter_attack_power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
    let lw3_attack_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_attack_mul"));
    let lw3_attack_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_attack_max"));
    let lw3_attack_max_for_enemy = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_attack_max_for_enemy"));
    let lw3_attack_power_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_attack_power_limit"));
    let mut attack_power = counter_attack_power*lw3_attack_mul;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw3_throw_toss"), L2CValue::Hash40s("special_air_lw3_throw_toss"), false.into());
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_CATCH_SET_CATCH);
        sv_module_access::_catch(fighter.lua_state_agent);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        if attack_power < lw3_attack_power_limit {
            attack_power = 0.0;
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_IS_ATTACK_ENEMY) {
            if lw3_attack_max_for_enemy < attack_power {
                attack_power = lw3_attack_max_for_enemy;
            }
        }
        else {
            if lw3_attack_max < attack_power {
                attack_power = lw3_attack_max
            }
        }
        WorkModule::set_float(fighter.module_accessor, attack_power, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
        WorkModule::set_int(fighter.module_accessor, get_node_object_id as i32, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw3_throw"), L2CValue::Hash40s("special_air_lw3_throw"), false.into());
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_lw3_throw_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_lw3_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let attack_power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
    let counter_throw_object_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
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
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING) {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw3_throw_toss"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw3_throw"), -1.0, 1.0, 0.0, false, false);
                }
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK) {
        if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                let init_life = WorkModule::get_int(counter_throw_boma, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
                WorkModule::set_int(counter_throw_boma, init_life*2, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                GroundModule::set_ignore_boss(counter_throw_boma, true);
                GroundModule::set_passable_check(counter_throw_boma, false);
                GroundModule::set_collidable(counter_throw_boma, false);
                JostleModule::set_status(counter_throw_boma, false);
                if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                    LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
                }
                if !LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                    VisibilityModule::set_whole(counter_throw_boma, true);
                    LinkModule::link(counter_throw_boma, *LINK_NO_ARTICLE, (*fighter.module_accessor).battle_object_id);
                    LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *LINK_NO_ARTICLE, Hash40::new("rot"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                    LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
                }
            }
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                let init_life = WorkModule::get_int(counter_throw_boma, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
                WorkModule::set_int(counter_throw_boma, init_life*2, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                GroundModule::set_ignore_boss(counter_throw_boma, true);
                GroundModule::set_passable_check(counter_throw_boma, false);
                GroundModule::set_collidable(counter_throw_boma, false);
                JostleModule::set_status(counter_throw_boma, false);
                if LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                    LinkModule::unlink(counter_throw_boma, *ITEM_LINK_NO_HAVE);
                }
                if !LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                    VisibilityModule::set_whole(counter_throw_boma, true);
                    LinkModule::link(counter_throw_boma, *ITEM_LINK_NO_HAVE, (*fighter.module_accessor).battle_object_id);
                    LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *ITEM_LINK_NO_HAVE, Hash40::new("rot"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                    LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
                }
            }
        }
    }
    else {
        if 0.0 < attack_power {
            AttackModule::set_power(fighter.module_accessor, 0, attack_power, true);
        }
        if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                    LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
                }
            }
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                if LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                    LinkModule::unlink(counter_throw_boma, *ITEM_LINK_NO_HAVE);
                }
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn miifighter_special_lw3_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let counter_throw_object_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    CatchModule::catch_cut(fighter.module_accessor, false, false);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
    if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    }
    0.into()
}

unsafe extern "C" fn miifighter_special_lw3_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let counter_throw_object_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
    if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    }
    0.into()
}

pub fn install() {
    Agent::new("miifighter")
    .status(Init, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, miifighter_special_lw3_throw_init_status)
    .status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, miifighter_special_lw3_throw_main_status)
    .status(End, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, miifighter_special_lw3_throw_end_status)
    .status(Exit, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, miifighter_special_lw3_throw_exit_status)
    .install()
    ;
}