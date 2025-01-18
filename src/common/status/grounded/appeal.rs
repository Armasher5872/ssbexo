use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Appeal_common)]
unsafe fn status_pre_appeal_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_APPEAL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_APPEAL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_APPEAL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, 0, param_1.get_u32(), 0);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_Appeal_Main)]
unsafe fn status_appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if frame >= 2.0 {
            if 0 < attack_kind {
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            }
        }
        else {
            if FighterUtil::is_available_smash_appeal(module_accessor) 
            && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_SMASH != 0
            && FighterUtil::is_smash_appeal_timing(module_accessor) 
            && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_APPEAL_RANDOM) {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x296b2ba1f5), true);
                fighter.change_status(FIGHTER_STATUS_KIND_SMASH_APPEAL.into(), true.into());
                return 0.into();
            }
        }
        /*   START OF NEW ADDITIONS   */
        //Up Taunt Fire Fox Cancel
        if fighter_kind == *FIGHTER_KIND_FOX && (41.0..=44.0).contains(&frame) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH.into(), false.into());
        }
        /*   END OF NEW ADDITIONS   */
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            //status_pre_appeal_common,
            status_appeal_main
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}