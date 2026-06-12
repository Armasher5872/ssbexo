use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_captain_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        ssbexo_captain_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_captain_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_captain_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_captain_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_captain_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_captain_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_captain_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_captain_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        wait_loop_clear(agent);
    }
}

unsafe extern "C" fn ssbexo_captain_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 0.6, 361, 15, 0, 8, 2.25, 7.5, 0.8, -0.5, Some(2.0), Some(-1.0), Some(0.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.6, 361, 15, 0, 8, 6.5, 0.0, 8.0, 15.0, Some(0.0), Some(8.0), Some(12.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_captain_dash_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 65, 67, 0, 90, 6.2, 0.0, 10.0, 12.0, Some(0.0), Some(8.0), Some(12.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.5);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 67, 0, 90, 4.8, 0.0, 10.0, 12.0, Some(0.0), Some(6.0), Some(12.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.5);
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attack100", ssbexo_captain_rapid_jab_acmd, Low)
    .acmd("game_attackdash", ssbexo_captain_dash_attack_acmd, Low)
    .install()
    ;
}