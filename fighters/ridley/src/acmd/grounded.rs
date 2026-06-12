use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_ridley_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        ssbexo_ridley_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_ridley_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_ridley_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_ridley_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_ridley_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_ridley_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_ridley_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_ridley_rapid_jab_sub_acmd(agent);
        wait_loop_clear(agent);
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_ridley_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.7, 361, 10, 0, 14, 6.0, 0.0, 7.5, 11.0, None, None, None, 0.5, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.7, 361, 10, 0, 12, 6.0, 0.0, 7.5, 16.0, None, None, None, 0.5, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.7, 361, 10, 0, 8, 6.0, 0.0, 7.5, 21.0, None, None, None, 0.5, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 2.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 5.0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_ridley_dash_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.5, 42, 67, 0, 70, 7.0, 0.0, 6.5, 14.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    Agent::new("ridley")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attack100", ssbexo_ridley_rapid_jab_acmd, Low)
    .acmd("game_attack100sub", ssbexo_ridley_rapid_jab_sub_acmd, Low)
    .acmd("game_attackdash", ssbexo_ridley_dash_attack_acmd, Low)
    .install()
    ;
}