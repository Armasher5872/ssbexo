use super::*;

unsafe extern "C" fn wario_appeal_kamikaze_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn wario_appeal_kamikaze_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn wario_appeal_kamikaze_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_kamikaze"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_appeal_kamikaze_main_loop as *const () as _))
}

unsafe extern "C" fn wario_appeal_kamikaze_main_loop(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn wario_appeal_kamikaze_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn wario_appeal_kamikaze_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn wario_appeal_kamikaze_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_APPEAL_KAMIKAZE, wario_appeal_kamikaze_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_APPEAL_KAMIKAZE, wario_appeal_kamikaze_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_APPEAL_KAMIKAZE, wario_appeal_kamikaze_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_APPEAL_KAMIKAZE, wario_appeal_kamikaze_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_APPEAL_KAMIKAZE, wario_appeal_kamikaze_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_APPEAL_KAMIKAZE, wario_appeal_kamikaze_exit_status)
    .install()
    ;
}