use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_pit_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        ssbexo_pit_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_pit_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_pit_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_pit_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_pit_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_pit_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_pit_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_pit_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        wait_loop_clear(agent);
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_pit_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 4.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(19.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_pit_dash_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 74, 0, 80, 3.5, 0.0, 4.5, 16.0, Some(0.0), Some(7.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.37);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    Agent::new("pit")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attack100", ssbexo_pit_rapid_jab_acmd, Low)
    .acmd("game_attackdash", ssbexo_pit_dash_attack_acmd, Low)
    .install()
    ;
}