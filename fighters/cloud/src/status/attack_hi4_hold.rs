use super::*;

unsafe extern "C" fn cloud_attack_hi4_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_attack_hi4_hold"} else {"attack_hi4_hold"};
    fighter.status_AttackHi4Hold_common(hash40(motion).into())
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, cloud_attack_hi4_hold_main_status)
    .install()
    ;
}