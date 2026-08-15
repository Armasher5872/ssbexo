use super::*;

unsafe extern "C" fn robot_special_hi_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = original_status(Main, fighter, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK);
    let boma = fighter.module_accessor;
    AttackModule::set_shield_stiff_mul(boma, 0.33);
    original(fighter)
}

pub fn install() {
    Agent::new("robot")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK, robot_special_hi_attack_main_status)
    .install()
    ;
}