use super::*;

//Aerial Arm Rotor Finisher ACMD
unsafe extern "C" fn ssbexo_robot_aerial_arm_rotor_finisher_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 45, 180, 0, 40, 6.0, 0.0, 9.0, -4.5, Some(0.0), Some(10.0), Some(14.0), 2.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    FT_MOTION_RATE(agent, 1.1);
}

pub fn install() {
    Agent::new("robot")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialairsend", ssbexo_robot_aerial_arm_rotor_finisher_acmd, Low)
    .install()
    ;
}