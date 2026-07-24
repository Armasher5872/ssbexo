use super::*;

unsafe extern "C" fn edge_attack_hi3_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.clear_lua_stack();
    let mot = sv_fighter_util::get_attack_hi3_motion(fighter.lua_state_agent);
    fighter.status_AttackHi3_Common(mot.into(), mot.into());
    if !StopModule::is_stop(boma) {
        fighter.sub_attack3_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_attack_hi3_main_loop as *const () as _))
}

unsafe extern "C" fn edge_attack_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let count = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    let charge_frame = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    let motion_kind = MotionModule::motion_kind(boma);
    let is_attack = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4;
    fighter.status_AttackHi3_Main();
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if StatusModule::is_changing(boma) {
            return 0.into();
        }
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            let can_change = fighter.global_table[CMD_CAT1].get_i32() & is_attack != 0 || charge_frame >= 5;
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                if charge_frame < 5 {
                    WorkModule::inc_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
                }
            }
            if can_change {
                if !StatusModule::is_changing(boma) {
                    WorkModule::inc_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
                }
            }
        }
        if count == 1 && motion_kind == hash40("attack_hi3") {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            MotionModule::change_motion(boma, Hash40::new("attack_hi3_wing"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    0.into()
}

unsafe extern "C" fn edge_attack_hi3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI3, edge_attack_hi3_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI3, edge_attack_hi3_end_status)
    .install()
    ;
}