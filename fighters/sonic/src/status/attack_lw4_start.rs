use super::*;

unsafe extern "C" fn sonic_attack_lw4_start_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_PHANTOM_BOOSTED_MOTION_RATE);
    fighter.status_end_AttackXX4Start();
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, sonic_attack_lw4_start_end_status)
    .install()
    ;
}