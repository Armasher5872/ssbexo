use super::*;

unsafe extern "C" fn daisy_attack_s4_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_xx4_uniq_process_init();
    0.into()
}

unsafe extern "C" fn daisy_attack_s4_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_s4_uniq_process_exit();
    0.into()
}

pub fn install() {
    Agent::new("daisy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_ATTACK_S4, daisy_attack_s4_init_status)
    .status(Exit, *FIGHTER_STATUS_KIND_ATTACK_S4, daisy_attack_s4_exit_status)
    .install()
    ;
}