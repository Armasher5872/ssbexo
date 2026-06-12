use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_bayonetta_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        ssbexo_bayonetta_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_bayonetta_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_bayonetta_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_bayonetta_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_bayonetta_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_bayonetta_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_bayonetta_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_bayonetta_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CHECK_COMBO_NUM);
        wait_loop_clear(agent);
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_bayonetta_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.2, 361, 20, 0, 10, 6.7, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(9.0), 0.6, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.2, 361, 20, 0, 8, 6.7, 0.0, 9.0, 18.0, Some(0.0), Some(9.0), Some(9.0), 0.6, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 15.0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_bayonetta_dash_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 15, 5, true);
    }
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.4);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 50, 71, 0, 78, 5.0, 0.0, 10.0, 10.5, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 50, 71, 0, 78, 1.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 50, 65, 0, 80, 4.5, 0.0, 10.0, 10.5, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 50, 65, 0, 80, 1.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
    frame(lua_state, 28.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
}

pub fn install() {
    Agent::new("bayonetta")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attack100", ssbexo_bayonetta_rapid_jab_acmd, Low)
    .acmd("game_attackdash", ssbexo_bayonetta_dash_attack_acmd, Low)
    .install()
    ;
}