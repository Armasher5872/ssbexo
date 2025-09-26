use super::*;

unsafe extern "C" fn cloud_special_s_lb_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0);
    0.into()
}

unsafe extern "C" fn cloud_special_s_lb_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    KineticModule::clear_speed_all(fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn cloud_special_s_lb_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04-1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04-1);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_lb"), L2CValue::Hash40s("special_air_s_lb"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_special_s_lb_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_special_s_lb_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let air_s2_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_s2_accel_y"));
    let air_s2_speed_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_s2_speed_max_y"));
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND 
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_lb"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR 
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
            KineticModule::clear_speed_all(fighter.module_accessor);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_lb"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if current_frame == 50.0 {
        if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_s2_accel_y);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_s2_speed_max_y);
        }
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

unsafe extern "C" fn cloud_special_s_lb_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn cloud_special_s_lb_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
    MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke1_r"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke1_l"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke2_r"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke2_l"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke3_r"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke3_l"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke4_r"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke4_l"), 5);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_NONE, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    display_final_window(false);
    0.into()
}

unsafe extern "C" fn cloud_special_s_lb_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let limit_level = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    WorkModule::set_int(fighter.module_accessor, limit_level-1, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
    MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke1_r"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke1_l"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke2_r"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke2_l"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke3_r"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke3_l"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke4_r"), 5);
    EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke4_l"), 5);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_NONE, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    display_final_window(false);
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LIMIT_BREAK, cloud_special_s_lb_pre_status)
    .status(Init, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LIMIT_BREAK, cloud_special_s_lb_init_status)
    .status(Main, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LIMIT_BREAK, cloud_special_s_lb_main_status)
    .status(Exec, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LIMIT_BREAK, cloud_special_s_lb_exec_status)
    .status(End, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LIMIT_BREAK, cloud_special_s_lb_end_status)
    .status(Exit, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LIMIT_BREAK, cloud_special_s_lb_exit_status)
    .install()
    ;
}