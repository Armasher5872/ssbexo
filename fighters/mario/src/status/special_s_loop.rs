use super::*;

//Side Special Loop Pre Status
unsafe extern "C" fn mario_special_s_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

//Side Special Loop Init Status
unsafe extern "C" fn mario_special_s_loop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let max_jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let gravity = if get_sum_speed_y > -1.5 {0.18} else {0.0242};
    if jump_count < max_jump_count {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    }
    if prev_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.0*lr, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.0, 0.0);
    }
    else {
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.75, 0.0);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.07, 0.0);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -3.9);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 3.9);
    0.into()
}

//Side Special Loop Main Status
unsafe extern "C" fn mario_special_s_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_loop"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_s_loop_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_s_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let flick_y = fighter.global_table[FLICK_Y].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let slow_rate = SlowModule::rate(fighter.module_accessor);
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_flick_y")) as i32;
    let special_s_degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_DEGREE);
    let max_degree = 90.0;
    let change_degree_per_frame = 0.7*slow_rate;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
        }
    }
    if GroundModule::is_passable_ground(fighter.module_accessor) && stick_y < pass_stick_y && flick_y < pass_flick_y {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
    }
    if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL) {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_JUMP.into(), false.into());
        }
    }
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
        if lr == -1.0 {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_BONK.into(), false.into());
        }
    }
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        if lr == 1.0 {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_BONK.into(), false.into());
        }
    } 
    if special_s_degree > -max_degree {
        WorkModule::set_float(fighter.module_accessor, special_s_degree-change_degree_per_frame, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_DEGREE);
    }
    change_angle(fighter.module_accessor, special_s_degree, max_degree, "special_s_loop_down", "special_s_loop_down");
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Side Special Loop Exec Status
unsafe extern "C" fn mario_special_s_loop_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);
    let attack_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ATTACK_FRAME);
    let gravity = if get_sum_speed_y > -1.5 {0.18} else {0.0242};
    if stick_x*lr < -0.2 {
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.0065/*Base Horizontal Air Acceleration*/);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0/*Additional Horizontal Air Acceleration*/);
    }
    if stick_x*lr > 0.2 {
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.07/*Base Horizontal Air Acceleration*/);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.01/*Additional Horizontal Air Acceleration*/);
    }
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity);
    if attack_frame > 32.0 {
        AttackModule::clear_all(fighter.module_accessor);
    }
    else {
        WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ATTACK_FRAME);
    }
    0.into()
}

//Side Special Loop End Status
unsafe extern "C" fn mario_special_s_loop_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ATTACK_FRAME);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_DEGREE);
    }
    0.into()
}

//Side Special Loop Exit Status
unsafe extern "C" fn mario_special_s_loop_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ATTACK_FRAME);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_DEGREE);
    }
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP, mario_special_s_loop_pre_status)
    .status(Init, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP, mario_special_s_loop_init_status)
    .status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP, mario_special_s_loop_main_status)
    .status(Exec, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP, mario_special_s_loop_exec_status)
    .status(End, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP, mario_special_s_loop_end_status)
    .status(Exit, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP, mario_special_s_loop_exit_status)
    .install()
    ;
}