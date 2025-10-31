use super::*;

unsafe extern "C" fn robot_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn robot_special_hi_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = original_status(Main, fighter, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK);
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, 0.33);
    original(fighter)
}

pub fn install() {
    Agent::new("robot")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, robot_special_hi_pre_status)
    .status(Main, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK, robot_special_hi_attack_main_status)
    .install()
    ;
}