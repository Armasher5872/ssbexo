use super::*;

unsafe extern "C" fn cloud_attack_lw4_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_status_attacklw4start_common(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackLw4Start_Main as *const () as _))
}

unsafe extern "C" fn cloud_status_attacklw4start_common(fighter: &mut L2CFighterCommon) {
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_attack_lw4"} else {"attack_lw4"};
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new(motion), 0.0, 1.0, false, 0.0, false, false);
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, cloud_attack_lw4_start_main_status)
    .install()
    ;
}