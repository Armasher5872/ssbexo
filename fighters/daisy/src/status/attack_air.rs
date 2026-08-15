use super::*;

unsafe extern "C" fn daisy_attack_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    despawn_glove(boma);
    0.into()
}

unsafe extern "C" fn daisy_attack_air_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    despawn_glove(boma);
    fighter.sub_attack_air_uniq_process_exit()
}

pub fn install() {
    Agent::new("daisy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, daisy_attack_air_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, daisy_attack_air_exit_status)
    .install()
    ;
}