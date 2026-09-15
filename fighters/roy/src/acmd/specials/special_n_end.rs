use super::*;

//Neutral Special End ACMD
unsafe extern "C" fn ssbexo_roy_neutral_special_end_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        KineticModule::set_consider_ground_friction(boma, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    FT_MOTION_RATE(agent, 7.0/10.0);
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 49, 110, 0, 40, 8.7, 0.0, 8.5, 15.7, Some(0.0), Some(9.0), Some(15.9), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    Agent::new("roy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnend", ssbexo_roy_neutral_special_end_acmd, Low)
    .acmd("game_specialairnend", ssbexo_roy_neutral_special_end_acmd, Low)
    .acmd("game_specialnend2", ssbexo_roy_neutral_special_end_acmd, Low)
    .acmd("game_specialairnend2", ssbexo_roy_neutral_special_end_acmd, Low)
    .acmd("game_specialnend3", ssbexo_roy_neutral_special_end_acmd, Low)
    .acmd("game_specialairnend3", ssbexo_roy_neutral_special_end_acmd, Low)
    .install()
    ;
}