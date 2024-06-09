use super::*;

//Aerial Kaclang ACMD
unsafe extern "C" fn ssbexo_brave_aerial_kaclang_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 270, 65, 0, 80, 7.0, 0.0, 2.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 15.0);
}

pub fn install() {
    Agent::new("brave")
    .game_acmd("game_specialairlw10", ssbexo_brave_aerial_kaclang_acmd)
    .install()
    ;
}