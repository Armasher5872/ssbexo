use super::*;

unsafe extern "C" fn edge_attack_lw3_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_attack_lw3_main_loop as *const () as _))
}

unsafe extern "C" fn edge_attack_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_stop = fighter.global_table[IS_STOP].get_bool();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let jump_attack_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let count = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    let charge_frame = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    let reserve_log_attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let status_attack = fighter.status_attack();
    let info = status_attack[0x10f40d7b92u64].get_i64();
    let is_attack = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4;
    if CancelModule::is_enable_cancel(boma) && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(boma) && fighter.sub_check_button_jump().get_bool() {
            MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion_kind), -1);
            WorkModule::set_int64(boma, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == jump_attack_frame {
        if !is_stop && reserve_log_attack_kind > 0 {
            FighterStatusModuleImpl::reset_log_action_info(boma, reserve_log_attack_kind);
            WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if StatusModule::is_changing(boma) {
        return 0.into();
    }
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            let can_change = fighter.global_table[CMD_CAT1].get_i32() & is_attack != 0 || charge_frame >= 3;
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                if charge_frame < 3 {
                    WorkModule::inc_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
                }
            }
            if can_change {
                if !StatusModule::is_changing(boma) {
                    WorkModule::inc_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
                }
            }
        }
        if count == 1 && motion_kind == hash40("attack_lw3") {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            MotionModule::change_motion(boma, Hash40::new("attack_lw3_wing"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn edge_attack_lw3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let reserve_log_attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < reserve_log_attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(boma, reserve_log_attack_kind);
        WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, edge_attack_lw3_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW3, edge_attack_lw3_end_status)
    .install()
    ;
}