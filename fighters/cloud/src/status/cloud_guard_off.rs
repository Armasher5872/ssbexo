use super::*;

//Cloud Guard Off Pre Status
unsafe extern "C" fn cloud_guard_off_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, true, false, false, 0, *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32, 0, 0);
    0.into()
}

//Cloud Guard Off Init Status
unsafe extern "C" fn cloud_guard_off_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Cloud Guard Off Main Status
unsafe extern "C" fn cloud_guard_off_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("punish_guard_off"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_GuardOff_Main as *const () as _))
}

//Cloud Guard Off Exec Status
unsafe extern "C" fn cloud_guard_off_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Cloud Guard Off End Status
unsafe extern "C" fn cloud_guard_off_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Cloud Guard Off Exit Status
unsafe extern "C" fn cloud_guard_off_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_CLOUD_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, 1.0);
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_CLOUD_STATUS_KIND_GUARD_OFF, cloud_guard_off_pre_status)
    .status(Init, *FIGHTER_CLOUD_STATUS_KIND_GUARD_OFF, cloud_guard_off_init_status)
    .status(Main, *FIGHTER_CLOUD_STATUS_KIND_GUARD_OFF, cloud_guard_off_main_status)
    .status(Exec, *FIGHTER_CLOUD_STATUS_KIND_GUARD_OFF, cloud_guard_off_exec_status)
    .status(End, *FIGHTER_CLOUD_STATUS_KIND_GUARD_OFF, cloud_guard_off_end_status)
    .status(Exit, *FIGHTER_CLOUD_STATUS_KIND_GUARD_OFF, cloud_guard_off_exit_status)
    .install()
    ;
}