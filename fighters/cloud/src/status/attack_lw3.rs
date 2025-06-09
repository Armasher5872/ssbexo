use super::*;

unsafe extern "C" fn cloud_attack_lw3_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_status_attack_lw3_common(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_attack_lw3_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_status_attack_lw3_common(fighter: &mut L2CFighterCommon) {
    let status_attack = fighter.status_attack();
    let log_infos = status_attack["log_infos"].clone();
    let attack_lw3 = log_infos["attack_lw3"].get_u64();
    cloud_status_attack_lw3_common_param(fighter, true.into(), L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _).into(), attack_lw3.into());
}

unsafe extern "C" fn cloud_status_attack_lw3_common_param(fighter: &mut L2CFighterCommon, param_2: L2CValue, l2cfightercommonptr: L2CValue, log_infos: L2CValue) {
    let status_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let get_attack_lw3_fb_kind = ControlModule::get_attack_lw3_fb_kind(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let reserve_log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
    let mut cont = false;
    let mot;
    if prev_status != status_interrupt {
        if prev_status != *FIGHTER_STATUS_KIND_ESCAPE {
            cont = true;
        }
    }
    else {
        if FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
            cont = true;
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        mot = Hash40::new("punish_attack_lw3");
    }
    else {
        fighter.clear_lua_stack();
        mot = sv_fighter_util::get_attack_lw3_motion(fighter.lua_state_agent);
    }
    MotionModule::change_motion(fighter.module_accessor, mot, 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int64(fighter.module_accessor, log_infos.get_i64(), *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if cont {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame+1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    if mini_jump_attack_frame != 0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            return;
        }
    }
    else {
        if 0 < reserve_log_attack_kind {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    if get_attack_lw3_fb_kind == *FIGHTER_COMMAND_ATTACK3_KIND_B {
        if param_2.get_bool() {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            sv_kinetic_energy!(set_chara_dir, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, lr);
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(l2cfightercommonptr.get_ptr());
        callable(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(l2cfightercommonptr.get_ptr() as *const () as _));
}

unsafe extern "C" fn cloud_attack_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_stop = fighter.global_table[IS_STOP].get_bool();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    let reserve_log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let status_attack = fighter.status_attack();
    let info = status_attack[0x10f40d7b92u64].get_i64();
    if CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor) && fighter.sub_check_button_jump().get_bool() {
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion_kind), -1);
            WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == jump_attack_frame {
        if !is_stop && reserve_log_attack_kind > 0 {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if !StatusModule::is_changing(fighter.module_accessor) {
                WorkModule::inc_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
            }
        }
    }
    if count == 1 && motion_kind == hash40("punish_attack_lw3") {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("punish_attack_lw32"), 0.0, 1.0, false, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn cloud_attack_lw3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let reserve_log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < reserve_log_attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, cloud_attack_lw3_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW3, cloud_attack_lw3_end_status)
    .install()
    ;
}