use super::*;

#[skyline::hook(replace = L2CFighterCommon_attack_mtrans_post_process)]
unsafe extern "C" fn attack_mtrans_post_process(fighter: &mut L2CFighterCommon) {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    //let stick_x = fighter.global_table[STICK_X].get_f32();
    //let stick_y = fighter.global_table[STICK_Y].get_f32();
    //let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let count = ComboModule::count(fighter.module_accessor);
    //let lr = PostureModule::lr(fighter.module_accessor);
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let attack_11_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
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
                        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
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
            /*if stick_x*lr > 0.7 || (cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S3.into(), true.into());
            }
            else if stick_y > 0.7 || (cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), true.into());
            }
            else if stick_y < -0.7 || (cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
            }
            else {*/
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_13"), 0.0, 1.0, false, 0.0, false, false);
                fighter.clear_lua_stack();
                sv_kinetic_energy::set_motion_energy_update_flag(fighter.lua_state_agent);
                WorkModule::set_int64(fighter.module_accessor, attack_13, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            /*}*/
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_12"), 0.0, 1.0, false, 0.0, false, false);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            sv_kinetic_energy::clear_speed_ex(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            sv_kinetic_energy::set_motion_energy_update_flag(fighter.lua_state_agent);
            WorkModule::set_int64(fighter.module_accessor, attack_12, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(attack_11_motion as u64), 0.0, 1.0, false, 0.0, false, false);
        if ![*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN].contains(&fighter_kind) {
            WorkModule::set_int64(fighter.module_accessor, attack_11, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(
            attack_mtrans_post_process
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}