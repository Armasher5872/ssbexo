use super::*;

unsafe extern "C" fn sonic_special_s_boost_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32, 0);
    0.into()
}

unsafe extern "C" fn sonic_special_s_boost_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.9);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

unsafe extern "C" fn sonic_special_s_boost_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_DASH);
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_loop"), L2CValue::Hash40s("special_air_s_loop"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_s_boost_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_s_boost_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stickx = fighter.global_table[STICK_X].get_f32();
    let stick_x = stickx*PostureModule::lr(fighter.module_accessor);
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let boost_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_SPEED);
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
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
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_loop"), -1.0, 1.0, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_DASH);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_loop"), -1.0, 1.0, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.clear_lua_stack();
        lua_args!(fighter, boost_speed, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_animcmd::SET_SPEED_EX(fighter.lua_state_agent);
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter, boost_speed, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_animcmd::SET_SPEED_EX(fighter.lua_state_agent);
    }
    if frame >= 5.0 {
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            if situation_kind == *SITUATION_KIND_AIR {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL)
                && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                    fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), false.into());
                }
            }
            else {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT) {
                    fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), false.into());
                }
            }
        }
        if fighter.down_input()
        && situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), false.into());
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && situation_kind == *SITUATION_KIND_AIR {
            if stick_x >= 0.7
            && (stick_y < 0.7 || stick_y > -0.7) {
                fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_EAGLE.into(), false.into());
            }
            if stick_y >= 0.7 {
                fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_ADVENTURE.into(), false.into());
            }
            if stick_y <= -0.7 {
                fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DROP.into(), false.into());
            }
        }
    }
    if frame >= 10.0 {
        let boost_gauge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE_DECREASE);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE_DECREASE) >= 5 {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE_DECREASE);
            if boost_gauge > 0 {
                WorkModule::sub_int(fighter.module_accessor, 1, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE);
            }
        }
        if boost_gauge <= 0
        || ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if situation_kind == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            }
            else {
                if stick_x >= 0.3 {
                    fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), false.into());
                }
                else if stick_x <= -0.3 {
                    fighter.change_status(FIGHTER_STATUS_KIND_TURN_RUN.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END.into(), false.into());
                }
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_BOOST.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_boost_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, Hash40::new("sys_speedbooster"), true, true);
    sv_animcmd::EFFECT_OFF_KIND(fighter.lua_state_agent);
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_BOOST, sonic_special_s_boost_pre_status)
    .status(Init, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_BOOST, sonic_special_s_boost_init_status)
    .status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_BOOST, sonic_special_s_boost_main_status)
    .status(End, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_BOOST, sonic_special_s_boost_end_status)
    .install()
    ;
}