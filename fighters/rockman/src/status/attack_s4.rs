use super::*;

unsafe extern "C" fn rockman_attack_s4_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AttackS4()
}

unsafe extern "C" fn rockman_attack_s4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4()
}

unsafe extern "C" fn rockman_attack_s4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb6), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 0);
    }
    fighter.status_end_AttackS4();
    0.into()
}

pub fn install() {
    Agent::new("rockman")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S4, rockman_attack_s4_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, rockman_attack_s4_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, rockman_attack_s4_end_status)
    .install()
    ;
}