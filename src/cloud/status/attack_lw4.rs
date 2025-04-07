use super::*;

unsafe extern "C" fn cloud_attack_lw4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_restart_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_attack_lw4"} else {"attack_lw4"};
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new(motion), smash_restart_frame, 1.0, 0.0);
    ControlModule::reset_trigger(fighter.module_accessor);
    ComboModule::set(fighter.module_accessor, *FIGHTER_COMBO_KIND_S4);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackLw4_Main as *const () as _))
}

pub fn install() {
    Agent::new("cloud")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, cloud_attack_lw4_main_status)
    .install()
    ;
}