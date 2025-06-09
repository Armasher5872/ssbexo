use super::*;

unsafe extern "C" fn cloud_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        cloud_check_attack_mtrans(fighter);
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(cloud_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Attack_Main as *const () as _))
}

unsafe extern "C" fn cloud_check_attack_mtrans(fighter: &mut L2CFighterCommon) -> L2CValue {
    let combo_count = ComboModule::count(fighter.module_accessor);
    let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if combo_count as i32 >= attack_combo_max {
            return 0.into();
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
            return 0.into();
        }
    }
    fighter.attack_mtrans_pre_process();
    cloud_attack_mtrans_post_process(fighter);
    0.into()
}

unsafe extern "C" fn cloud_attack_mtrans_post_process(fighter: &mut L2CFighterCommon) {
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let count = ComboModule::count(fighter.module_accessor);
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let punisher_mode = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    let attack_11_motion = if punisher_mode {Hash40::new("punish_attack_11")} else {Hash40::new("attack_11")};
    let attack_12_motion = if punisher_mode {Hash40::new("punish_attack_12")} else {Hash40::new("attack_12")};
    let attack_13_motion = if punisher_mode {Hash40::new("punish_attack_13")} else {Hash40::new("attack_13")};
    let reserve_log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let attack_jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("attack_jump_mini_attack_enable_frame"));
    let status_attack = fighter.status_attack();
    let log_infos = status_attack["log_infos"].clone();
    let attack_11 = log_infos["attack_11"].get_i64();
    let attack_12 = log_infos["attack_12"].get_i64();
    let attack_13 = log_infos["attack_13"].get_i64();
    let mut cont = false;
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_status_kind != status_kind_interrupt {
            if prev_status_kind != *FIGHTER_STATUS_KIND_ESCAPE {
                cont = true;
            }
        }
        else {
            if FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
                cont = true;
            }
        }
    }
    if count != 1 {
        if count != 2 {
            if count != 3 {
                if StatusModule::is_changing(fighter.module_accessor) {
                    if cont {
                        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
                            WorkModule::set_int(fighter.module_accessor, attack_jump_mini_attack_enable_frame+1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
                            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                        }
                    }
                }
                else {
                    if 0 < mini_jump_attack_frame {
                        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
                        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                    }
                }
                if mini_jump_attack_frame != 0 {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
                        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
                        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER);
                    }
                }
                if 0 < reserve_log_attack_kind {
                    FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
                    WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                }
            }
            MotionModule::change_motion(fighter.module_accessor, attack_13_motion, 0.0, 1.0, false, 0.0, false, false);
            fighter.clear_lua_stack();
            sv_kinetic_energy::set_motion_energy_update_flag(fighter.lua_state_agent);
            WorkModule::set_int64(fighter.module_accessor, attack_13, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, attack_12_motion, 0.0, 1.0, false, 0.0, false, false);
            fighter.clear_lua_stack();
            sv_kinetic_energy::set_motion_energy_update_flag(fighter.lua_state_agent);
            WorkModule::set_int64(fighter.module_accessor, attack_12, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, attack_11_motion, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::set_int64(fighter.module_accessor, attack_11, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        //Removed the Fighter Kind Check for Ryu/Ken
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK, cloud_attack_main_status)
    .install()
    ;
}