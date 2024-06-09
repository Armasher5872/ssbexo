use super::*;

//Run Brake
#[skyline::hook(replace = L2CFighterCommon_status_TurnRun_Main)]
unsafe fn status_turnrun_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32(); //New
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32(); //New
    let flick_y = fighter.global_table[FLICK_Y].get_i32(); //New
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32(); //New
    let lr = PostureModule::lr(fighter.module_accessor);
    let lr_stick_x = stick_x*lr;
    let run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x"));
    let run_turn_cancel_start_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("run_turn_cancel_start_frame"));
    let run_turn_cancel_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("run_turn_cancel_enable_frame"));
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y")); //New
    let pass_flick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_flick_y")) as i32; //New
    let run_turn = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_RUN_TURN);
    let notify_taunt_hash = {fighter.clear_lua_stack(); fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be)); smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()}; //New
    if fighter.global_table[DASH_COMMON_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[DASH_COMMON_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 {
            if !ItemModule::is_have_item(fighter.module_accessor, 0) {
                WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
                WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
                return 1.into();
            }
        }
    }
    if run_turn <= run_turn_cancel_start_frame+run_turn_cancel_enable_frame {
        if fighter.sub_transition_group_check_ground_item().get_bool() {
            return 1.into();
        }
        if fighter.sub_transition_group_check_ground_special().get_bool() {
            return 1.into();
        }
        if fighter.sub_transition_group_check_ground_attack().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B != 0 {
            WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_ground_jump().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_RUN_FLAG_TO_RUN) {
            if run_stick_x <= lr_stick_x {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN) {
                    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
                    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
                    fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), false.into());
                }
                else {
                    //WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
                    //WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
                    fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), false.into());
                }
                return 1.into();
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_RUN_FLAG_STOP_START) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_RUN_FLAG_STOP) {
            if stick_x.abs() < run_stick_x {
                //WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
                //WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
                fighter.change_status(FIGHTER_STATUS_KIND_TURN_RUN_BRAKE.into(), false.into());
                return 1.into();
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_RUN_FLAG_STOP_START) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_RUN_FLAG_STOP) {
            if 0.0 < lr_stick_x {
                WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
                WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
                return 1.into();
            }
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_RUN_FLAG_STOP_SHAKE) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        WorkModule::set_float(fighter.module_accessor, speed_x, *FIGHTER_STATUS_TURN_RUN_WORK_FLOAT_START_SPEED);
        let run_brake_stop_shake_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_brake_stop_shake_speed"));
        if speed_x < run_brake_stop_shake_speed {
            ShakeModule::stop_kind(fighter.module_accessor, Hash40::new("brake"));
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_RUN_FLAG_STOP_SHAKE);
        }
        else {
            let brake_start_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_TURN_RUN_WORK_FLOAT_START_SPEED);
            let diff = brake_start_speed-run_brake_stop_shake_speed;
            let diff2 = speed_x-run_brake_stop_shake_speed;
            let shake_data_brake_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("shake_data_brake_scale"));
            let run_brake_stop_shake_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_brake_stop_shake_scale"));
            let ratio = diff2/diff;
            let lerp = fighter.lerp(run_brake_stop_shake_scale.into(), 1.0_f32.into(), ratio.into()).get_f32();
            let mul = lerp*shake_data_brake_scale;
            ShakeModule::set_scale_kind(fighter.module_accessor, Hash40::new("brake"), mul);
        }
    }
    /* START OF NEW ADDITION */
    //Allows platform drops out of turn run
    if GroundModule::is_passable_ground(fighter.module_accessor) && stick_y < pass_stick_y && flick_y < pass_flick_y {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        return 0.into();
    }
    //Allows taunts out of turn run
    if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0) 
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0) 
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0))
    && notify_taunt_hash {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
        fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
        return 0.into();
    }
    //R.O.B. Snaking
    if fighter_kind == *FIGHTER_KIND_ROBOT {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
            return 0.into();
        }
    }
    /* END OF NEW ADDITION */
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_turnrun_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}