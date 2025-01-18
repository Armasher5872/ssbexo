/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Dash Attack
#[skyline::hook(replace = L2CFighterCommon_status_AttackDash)]
unsafe fn status_attackdash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
    let jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
    let log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let log_infos = fighter.status_attack()["log_infos"]["attack_dash"].get_int();
    let mini_jump_attack = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_FLAG_ATTACK_HI4_DISABLE);
    WorkModule::set_int64(fighter.module_accessor, log_infos as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame+1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    if mini_jump_attack != 0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) && log_attack_kind > 0 {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack_dash_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackDash_Main as *const () as _))
}

//Dash Attack Main
#[skyline::hook(replace = L2CFighterCommon_status_AttackDash_Main)]
unsafe extern "C" fn status_attackdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(boma);
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let status_attack = fighter.status_attack();
    let status_attack_info = status_attack[0x10f40d7b92u64].get_i64();
    let motion_kind = MotionModule::motion_kind(boma);
    let reserve_log = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let fall_mul = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    let mini_jump_attack_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let turn_run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_run_stick_x"));
    let get_correct = GroundModule::get_correct(boma);
    if CancelModule::is_enable_cancel(boma) && fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    if 0 < mini_jump_attack_frame {
        if !StopModule::is_stop(boma) && fighter.sub_check_button_jump().get_bool() {
            MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion_kind), -1);
            WorkModule::set_int64(boma, status_attack_info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == mini_jump_attack_frame {
        if !fighter.global_table[IS_STOP].get_bool() && reserve_log > 0 {
            FighterStatusModuleImpl::reset_log_action_info(boma, reserve_log);
            WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    /* START OF NEW ADDITIONS */
    //Gatlings (DACUS/DACDS)
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
        && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
            return 1.into();
        }
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0
        && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4_START.into(), true.into());
            return 1.into();
        }
    }
    //Adjusts the ground correct depending on if the Dash Attack edge cancels or goes off ledges.
    if situation_kind == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL) {
            if get_correct != *GROUND_CORRECT_KIND_GROUND {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
        }
        else {
            if get_correct != *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            }
        }
    }
    //Enables Dash Attack Gravity. Code runs once and only whilst in the air, but enables a second flag that checks when your situation kind changes, and if so, reenable gravity.
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY) && situation_kind != *SITUATION_KIND_GROUND {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
        if fall_mul.signum() != -1.0 {
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y*fall_mul);
        }
    }
    //Checks if a Dash Attack should edge cancel or go off ledges. Checked in a 'is_situation_changed' check to prevent the code running every frame.
    if StatusModule::is_situation_changed(boma) {
        if situation_kind != *SITUATION_KIND_GROUND {
            //Checks if your dash attack should roll off or not.
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE) {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                //Enables gravity
                sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                if fall_mul.signum() != -1.0 {
                    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y*fall_mul);
                }
                //Changes to Attack Air Dash if you have the motion kind in your motion_list.bin.
                if MotionModule::is_anim_resource(boma, Hash40::new("attack_air_dash")) {
                    GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                    MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_air_dash"), -1.0, 1.0, 0.0, false, false);
                }
                //Previously mentioned flag that's only checked when your situation_kind changes
                if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED) {
                    if fall_mul.signum() != -1.0 {
                        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y*fall_mul);
                    }
                }
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 0.into();
            }
        }
        else {
            //Checks if the Enable Air Landing Flag is enabled, and if so, change into the landing status script, rather than continuing the dash attack.
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING) {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 0.into();
            }
            else {
                //Otherwise, change to the dash attack animation if you were previously in the Aerial Dash Attack animation.
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_dash"), -1.0, 1.0, 0.0, false, false);
                KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            }
        }
    }
    //Link Dash Attack Bound
    if fighter_kind == *FIGHTER_KIND_LINK && (14.0..24.0).contains(&frame) && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_ATTACK_DASH_BOUND.into(), false.into());
    }
    //Kirby Aerial Dash Attack
    if fighter_kind == *FIGHTER_KIND_KIRBY && situation_kind == *SITUATION_KIND_AIR && fighter.check_jump_cancel(false, false) && frame > 25.0 {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
    }
    //Snake Dash Attack Item Toss
    if fighter_kind == *FIGHTER_KIND_SNAKE && frame > 12.0 && ItemModule::is_have_item(boma, 0) && motion_kind != hash40("attack_dash_throw") {
        MotionModule::change_motion(boma, Hash40::new("attack_dash_throw"), 0.0, 1.0, false, 0.0, false, false);
        AttackModule::clear_all(boma);
    }
    /* END OF NEW ADDITIONS */
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN)
    && stick_x <= turn_run_stick_x
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
    && !ItemModule::is_have_item(boma, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && !ItemModule::is_have_item(boma, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
        return 0.into();
    }
    if MotionModule::is_end(boma) {
        let status = if situation_kind != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else if WorkModule::get_param_int(boma, 0x17e10662a4, 0) == *FIGHTER_ATTACK_DASH_TYPE_SQUAT_WAIT {
            FIGHTER_STATUS_KIND_SQUAT_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

//Status End Dash Attack, resets the gatling flag
#[skyline::hook(replace = L2CFighterCommon_status_end_AttackDash)]
unsafe extern "C" fn status_end_attackdash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int64(fighter.module_accessor, 0i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    /* START OF NEW ADDITIONS */
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    /* END OF NEW ADDITIONS */
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_attackdash,
            status_attackdash_main,
            status_end_attackdash
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}