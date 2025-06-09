use super::*;

unsafe extern "C" fn cloud_fall_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_sub_fall_uniq_process_init_param(fighter, FIGHTER_STATUS_KIND_FALL.into(), FIGHTER_STATUS_KIND_FALL_AERIAL.into(), FIGHTER_STATUS_KIND_WALK.into());
    0.into()
}

unsafe extern "C" fn cloud_sub_fall_uniq_process_init_param(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue) {
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let fall_x_accel_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_ACCEL_MUL);
    let mul_fall_x_accel = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
    let fall_x_max_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    let fall_hop_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_HOP_Y);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let final_end_fall_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("final_end_fall_speed_y"));
    let cliff_release_disable_wall_jump_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_release_disable_wall_jump_frame"));
    if status_kind_interrupt != param_2.get_i32() {
        if status_kind_interrupt == param_3.get_i32() {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
                WorkModule::set_int64(fighter.module_accessor, hash40("punish_fall_aerial_f") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
                WorkModule::set_int64(fighter.module_accessor, hash40("punish_fall_aerial_b") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, hash40("fall_aerial_f") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
                WorkModule::set_int64(fighter.module_accessor, hash40("fall_aerial_b") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
            }
        }
        if status_kind_interrupt == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            WorkModule::set_int64(fighter.module_accessor, hash40("fall_special_f") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
            WorkModule::set_int64(fighter.module_accessor, hash40("fall_special_b") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_FIGHTER_AUDIENCE_CHECK_TUMULT_FALL_SPECIAL, entry_id, pos_y);
            sv_fighter_audience_notify_event_msc_cmd(fighter.lua_state_agent);
            EffectModule::req_common(fighter.module_accessor, Hash40::new("fall_special"), 0.0);
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
            WorkModule::set_int64(fighter.module_accessor, hash40("punish_fall_f") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
            WorkModule::set_int64(fighter.module_accessor, hash40("punish_fall_b") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
        }
        else {
            WorkModule::set_int64(fighter.module_accessor, hash40("fall_f") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
            WorkModule::set_int64(fighter.module_accessor, hash40("fall_b") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_END_TO_FALL) {
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, final_end_fall_speed_y);
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FALL_HOP) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FALL_HOP);
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fall_hop_y);
            }
        }
    }
    WorkModule::set_int64(fighter.module_accessor, hash40("invalid") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_2ND);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_FALL_WORK_FLOAT_MOTION_VALUE);
    if 0.0 <= fall_x_accel_mul {
        sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, fall_x_accel_mul);
    }
    if mul_fall_x_accel != 1.0 {
        sv_kinetic_energy!(mul_x_accel_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, mul_fall_x_accel);
        sv_kinetic_energy!(mul_x_accel_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, mul_fall_x_accel);
    }
    if fall_x_max_mul != 1.0 {
        sv_kinetic_energy!(mul_x_speed_max, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, fall_x_max_mul);
    }
    if ![param_4.get_i32(), *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&prev_status_kind) {
        if prev_status_kind == *FIGHTER_STATUS_KIND_CLIFF_WAIT {
            WorkModule::set_int(fighter.module_accessor, cliff_release_disable_wall_jump_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_WALL_JUMP_FRAME);
        }
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_FALL_FLAG_FROM_GROUND);
    }
}

unsafe extern "C" fn cloud_fall_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_status_fall_sub(fighter, 0.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_fall_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_status_fall_sub(fighter: &mut L2CFighterCommon, value: L2CValue) {
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_fall"} else {"fall"};
    fighter.status_FallSub_param(value.into(), hash40(motion).into());
}

unsafe extern "C" fn cloud_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let fall_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_FALL_WORK_INT_FRAME);
    let jump_no_aerial_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_no_aerial_frame"));
    let jump_run_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_run_stick_y"));
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !fighter.sub_fall().get_bool() {
            if fall_frame <= jump_no_aerial_frame {
                if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                    if fighter.sub_check_button_jump().get_bool() {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_FALL_FLAG_FROM_GROUND) {
                            fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), true.into());
                            return 0.into();
                        }
                    }
                }
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT) {
                    if jump_run_stick_y <= stick_y {
                        if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0 {
                            if ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_FALL_FLAG_FROM_GROUND) {
                                    fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), true.into());
                                    return 0.into();
                                }
                            }
                        }
                    }
                }
            }
            if !fighter.sub_air_check_fall_common().get_bool() {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_FALL_FLAG_DISABLE_MOTION_INTP) {
                    if MotionModule::is_end(fighter.module_accessor) {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("punish_fall"), 0.0, 1.0, false, 0.0, false, false);
                        }
                        else {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
                        }
                        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_FALL_FLAG_DISABLE_MOTION_INTP);
                    }
                }
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAMON_EXHAUST) {
                    fighter.change_status(FIGHTER_STATUS_KIND_METAMON_OUT.into(), false.into());
                    return 1.into();
                }
                fighter.sub_air_check_superleaf_fall_slowly();
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_FALL, cloud_fall_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_FALL, cloud_fall_main_status)
    .install()
    ;
}