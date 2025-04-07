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
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let combo_count = ComboModule::count(fighter.module_accessor) as i32;
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let reserve_log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let s3_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s3_combo_max"), 0);
    let status_attack = fighter.status_attack();
    let info = status_attack[0x10f40d7b92u64].get_i64();
    if CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if combo_count < s3_combo_max && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE) && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            cloud_attack_s3_mtrans_param(fighter, combo_kind);
        }
    }
    else {
        cloud_attack_s3_mtrans_param(fighter, combo_kind);
    }
    if situation_kind != *SITUATION_KIND_AIR {
        if 0 < mini_jump_attack_frame {
            if !StopModule::is_stop(fighter.module_accessor) && fighter.sub_check_button_jump().get_bool() {
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion_kind), -1);
                WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                fighter.change_status_jump_mini_attack(true.into());
                return 1.into();
            }
        }
        if 1 == mini_jump_attack_frame {
            if !global_is_stop && reserve_log_attack_kind > 0 {
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
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
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let reserve_log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
    let status_attack = fighter.status_attack();
    let log_infos = status_attack["log_infos"].clone();
    let attack_s3_s = log_infos["attack_s3_s"].get_u64();
    let mut cont = false;
    let mot;
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::clear_command(fighter.module_accessor, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT);
    ComboModule::set(fighter.module_accessor, combo_kind.get_i32());
    if StatusModule::is_changing(fighter.module_accessor) {
        if status_interrupt != prev_status {
            cont = true;
        }
        else {
            if FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
                cont = true;
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        mot = Hash40::new("punish_attack_s3_s");
    }
    else {
        fighter.clear_lua_stack();
        mot = sv_fighter_util::get_attack_s3_s_motion(fighter.lua_state_agent);
    }
    MotionModule::change_motion(fighter.module_accessor, mot, 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int64(fighter.module_accessor, attack_s3_s as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if 0 < mini_jump_attack_frame {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    else {
        if cont && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame+1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    if mini_jump_attack_frame != 0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            return;
        }
    }
    if 0 < reserve_log_attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
}

pub fn install() {
    Agent::new("cloud")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S3, cloud_attack_s3_main_status)
    .install()
    ;
}