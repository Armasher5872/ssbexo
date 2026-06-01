use super::*;

//Down Special Bounce Pre Status
unsafe extern "C" fn captain_special_lw_bounce_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

//Down Special Bounce Init Status
unsafe extern "C" fn captain_special_lw_bounce_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Down Special Bounce Main Status
unsafe extern "C" fn captain_special_lw_bounce_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("special_air_lw_bounce"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(captain_special_lw_bounce_main_loop as *const () as _))
}

//Down Special Bounce Main Loop Status
unsafe extern "C" fn captain_special_lw_bounce_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

//Down Special Bounce Exec Status
unsafe extern "C" fn captain_special_lw_bounce_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Down Special Bounce End Status
unsafe extern "C" fn captain_special_lw_bounce_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Down Special Bounce Exit Status
unsafe extern "C" fn captain_special_lw_bounce_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_BOUNCE, captain_special_lw_bounce_pre_status)
    .status(Init, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_BOUNCE, captain_special_lw_bounce_init_status)
    .status(Main, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_BOUNCE, captain_special_lw_bounce_main_status)
    .status(Exec, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_BOUNCE, captain_special_lw_bounce_exec_status)
    .status(End, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_BOUNCE, captain_special_lw_bounce_end_status)
    .status(Exit, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_BOUNCE, captain_special_lw_bounce_exit_status)
    .install()
    ;
}