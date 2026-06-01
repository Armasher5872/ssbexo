use super::*;

unsafe extern "C" fn cloud_attack_lw4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let smash_restart_frame = WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    let motion = if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_attack_lw4"} else {"attack_lw4"};
    WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    MotionModule::change_motion_force_inherit_frame(boma, Hash40::new(motion), smash_restart_frame, 1.0, 0.0);
    ControlModule::reset_trigger(boma);
    ComboModule::set(boma, *FIGHTER_COMBO_KIND_S4);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackLw4_Main as *const () as _))
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, cloud_attack_lw4_main_status)
    .install()
    ;
}