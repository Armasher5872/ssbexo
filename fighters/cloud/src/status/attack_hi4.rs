use super::*;

unsafe extern "C" fn cloud_attack_hi4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {0x11f196e053u64} else {0xa5598d745u64};
    fighter.status_AttackHi4_common(motion.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackHi4_Main as *const () as _))
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4, cloud_attack_hi4_main_status)
    .install()
    ;
}