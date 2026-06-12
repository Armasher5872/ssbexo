use super::*;

//Crouch Spin Kick ACMD
unsafe extern "C" fn ssbexo_demon_crouch_spin_kick_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 40, 0, 0, 80, 3.6, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(11.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 180, 0, 0, 40, 3.6, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(11.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_down_only(boma, 0, true);
        AttackModule::set_add_reaction_frame(boma, 0, 9.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 9.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.5);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.5);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attacksquat3", ssbexo_demon_crouch_spin_kick_acmd, Low)
    .install()
    ;
}