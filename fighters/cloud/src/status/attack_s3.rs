use super::*;

unsafe extern "C" fn cloud_attack_s3_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS3Common();
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_attack_s3_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_attack_s3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_attack_s3_main_loop_inner(fighter, FIGHTER_COMBO_KIND_S3.into());
    0.into()
}

unsafe extern "C" fn cloud_attack_s3_main_loop_inner(fighter: &mut L2CFighterCommon, combo_kind: L2CValue) -> L2CValue {
    let global_is_stop = fighter.global_table[IS_STOP].get_bool();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let combo_count = ComboModule::count(boma) as i32;
    let motion_kind = MotionModule::motion_kind(boma);
    let count = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    let mini_jump_attack_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let reserve_log_attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let s3_combo_max = WorkModule::get_param_int(boma, hash40("s3_combo_max"), 0);
    let status_attack = fighter.status_attack();
    let info = status_attack[0x10f40d7b92u64].get_i64();
    if CancelModule::is_enable_cancel(boma) && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if !StatusModule::is_changing(boma) {
        if combo_count < s3_combo_max && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE) && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            cloud_attack_s3_mtrans_param(fighter, combo_kind);
        }
    }
    else {
        cloud_attack_s3_mtrans_param(fighter, combo_kind);
    }
    if situation_kind != *SITUATION_KIND_AIR {
        if 0 < mini_jump_attack_frame {
            if !StopModule::is_stop(boma) && fighter.sub_check_button_jump().get_bool() {
                MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion_kind), -1);
                WorkModule::set_int64(boma, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                fighter.change_status_jump_mini_attack(true.into());
                return 1.into();
            }
        }
        if 1 == mini_jump_attack_frame {
            if !global_is_stop && reserve_log_attack_kind > 0 {
                FighterStatusModuleImpl::reset_log_action_info(boma, reserve_log_attack_kind);
                WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            }
        }
        if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
            if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    if !StatusModule::is_changing(boma) {
                        WorkModule::inc_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
                    }
                }
            }
            if count == 1 && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
                MotionModule::set_frame_sync_anim_cmd(boma, 97.0, true, false, false);
            }
            if (frame == 41.0 && count < 1) || (frame == 123.0 && count == 1) {
                CancelModule::enable_cancel(boma);
            }
            if frame == 96.0 {
                KineticModule::clear_speed_all(boma);
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
        if MotionModule::is_end(boma) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn cloud_attack_s3_mtrans_param(fighter: &mut L2CFighterCommon, combo_kind: L2CValue) {
    let status_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let mini_jump_attack_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let reserve_log_attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let jump_mini_attack_enable_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("jump_mini_attack_enable_frame"));
    let status_attack = fighter.status_attack();
    let log_infos = status_attack["log_infos"].clone();
    let attack_s3_s = log_infos["attack_s3_s"].get_u64();
    let mut cont = false;
    let mot;
    ControlModule::reset_trigger(boma);
    ControlModule::clear_command(boma, false);
    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE);
    WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT);
    ComboModule::set(boma, combo_kind.get_i32());
    if StatusModule::is_changing(boma) {
        if status_interrupt != prev_status {
            cont = true;
        }
        else {
            if FighterMotionModuleImpl::is_valid_cancel_frame(boma, -1, true) {
                cont = true;
            }
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        mot = Hash40::new("punish_attack_s3_s");
    }
    else {
        fighter.clear_lua_stack();
        mot = sv_fighter_util::get_attack_s3_s_motion(fighter.lua_state_agent);
    }
    MotionModule::change_motion(boma, mot, 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int64(boma, attack_s3_s as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if !StatusModule::is_changing(boma) {
        if 0 < mini_jump_attack_frame {
            WorkModule::set_int(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    else {
        if cont && !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            WorkModule::set_int(boma, jump_mini_attack_enable_frame+1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    if mini_jump_attack_frame != 0 {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            return;
        }
    }
    if 0 < reserve_log_attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(boma, reserve_log_attack_kind);
        WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
}

unsafe extern "C" fn cloud_attack_s3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let reserve_log_attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < reserve_log_attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(boma, reserve_log_attack_kind);
        WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S3, cloud_attack_s3_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S3, cloud_attack_s3_end_status)
    .install()
    ;
}