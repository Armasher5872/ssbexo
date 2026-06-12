use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_inkling_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        ssbexo_inkling_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_inkling_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_inkling_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_inkling_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_inkling_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_inkling_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_inkling_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_inkling_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        wait_loop_clear(agent);
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_inkling_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
            AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 6.0);
            AttackModule::set_ink_value(boma, 0, 10.0);
            AttackModule::set_ink_value(boma, 1, 10.0);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_inkling_dash_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        inkling_generate_squid_helper(agent)
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 60, 43, 0, 110, 4.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.5);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 43, 0, 100, 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.5);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    Agent::new("inkling")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attack100", ssbexo_inkling_rapid_jab_acmd, Low)
    .acmd("game_attackdash", ssbexo_inkling_dash_attack_acmd, Low)
    .install()
    ;
}