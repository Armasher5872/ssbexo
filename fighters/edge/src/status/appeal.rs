use super::*;

unsafe extern "C" fn edge_appeal_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Appeal_common_uniq(L2CValue::Void());
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_appeal_main_loop as *const () as _))
}

unsafe extern "C" fn edge_appeal_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let boma = fighter.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if current_frame >= 2.0 {
        let mot = if PostureModule::lr(boma) < 0.0 {WorkModule::get_int64(boma, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L)} else {WorkModule::get_int64(boma, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R)};
        edge_taunt_hold(fighter, mot as u64, WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_APPEAL_RESET_FRAME));
        if 0 < attack_kind {
            FighterStatusModuleImpl::reset_log_action_info(boma, attack_kind);
            WorkModule::set_int64(boma, 0i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    else {
        if FighterUtil::is_available_smash_appeal(module_accessor) {
            if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_SMASH != 0 {
                if FighterUtil::is_smash_appeal_timing(module_accessor) {
                    if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_APPEAL_RANDOM) {
                        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x296b2ba1f5), true);
                        fighter.change_status(FIGHTER_STATUS_KIND_SMASH_APPEAL.into(), true.into());
                    }
                }
            }
        }
    }
    if MotionModule::is_end(boma) {
        if [hash40("appeal_hi_l_loop"), hash40("appeal_hi_r_loop"), hash40("appeal_s_l_loop"), hash40("appeal_s_r_loop"), hash40("appeal_lw_l_loop"), hash40("appeal_lw_r_loop")].contains(&motion_kind) {
            MotionModule::change_motion(boma, Hash40::new_raw(motion_kind), 0.0, 1.0, false, 0.0, false, false);
        }
        else if [hash40("appeal_s_l_trans"), hash40("appeal_s_r_trans")].contains(&motion_kind) {
            let mot = if PostureModule::lr(boma) < 0.0 {"appeal_s_l_loop"} else {"appeal_s_r_loop"};
            MotionModule::change_motion(boma, Hash40::new(mot), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_APPEAL, edge_appeal_main_status)
    .install()
    ;
}