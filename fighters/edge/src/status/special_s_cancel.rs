use super::*;

//Shadow Flare Cancel Pre Status
unsafe extern "C" fn edge_special_s_cancel_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

//Shadow Flare Cancel Init Status
unsafe extern "C" fn edge_special_s_cancel_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Shadow Flare Cancel Main Status
unsafe extern "C" fn edge_special_s_cancel_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_cancel").into(), Hash40::new("special_air_s_cancel").into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_s_cancel_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_s_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(boma) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_cancel").into(), Hash40::new("special_air_s_cancel").into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
        edge_special_kinetic_handler(fighter, false);
    }
    if MotionModule::is_end(boma) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 0.into();
    }
    0.into()
}

//Shadow Flare Cancel Exec Status
unsafe extern "C" fn edge_special_s_cancel_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Shadow Flare Cancel End Status
unsafe extern "C" fn edge_special_s_cancel_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Shadow Flare Cancel Exit Status
unsafe extern "C" fn edge_special_s_cancel_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CANCEL, edge_special_s_cancel_pre_status)
    .status(Init, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CANCEL, edge_special_s_cancel_init_status)
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CANCEL, edge_special_s_cancel_main_status)
    .status(Exec, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CANCEL, edge_special_s_cancel_exec_status)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CANCEL, edge_special_s_cancel_end_status)
    .status(Exit, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CANCEL, edge_special_s_cancel_exit_status)
    .install()
    ;
}