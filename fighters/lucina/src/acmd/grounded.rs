use super::*;

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_lucina_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 10.925, 45, 67, 0, 75, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 10.925, 45, 67, 0, 75, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("claviclel"), 10.925, 45, 67, 0, 75, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 10.925, 45, 67, 0, 75, 3.5, 1.0, 0.0, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 1.5);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 10.925, 45, 67, 0, 75, 3.5, 1.0, 0.0, 2.0, Some(8.0), Some(1.5), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 10.925, 45, 67, 0, 75, 3.5, 1.0, 0.0, 6.5, Some(12.0), Some(1.5), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("lucina")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackdash", ssbexo_lucina_dash_attack_acmd, Low)
    .install()
    ;
}