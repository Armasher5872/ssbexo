use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_master_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 6.0, 0.0, 7.0, 9.0, None, None, None, 0.4, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 6.0, 0.0, 7.0, 18.0, None, None, None, 0.4, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
            AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        if is_excute(agent) {
            ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, Hash40::new("attack_100"), false, -1.0);
        }
        frame(lua_state, 2.0);
        ssbexo_master_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_master_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_master_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_master_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_master_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_master_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_master_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_master_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 6.0, 0.0, 7.0, 9.0, None, None, None, 0.4, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 6.0, 0.0, 7.0, 18.0, None, None, None, 0.4, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
        AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_master_dash_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword1"), 9.0, 361, 68, 0, 61, 3.2, -0.6, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 13.0, 361, 68, 0, 61, 4.0, 6.0, -2.0, -2.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 13.0, 361, 68, 0, 61, 4.0, 10.5, -2.0, -2.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 13.0, 361, 68, 0, 61, 5.0, 0.0, 7.0, 13.0, Some(0.0), Some(7.0), Some(16.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 4, 0, Hash40::new("top"), 9.0, 361, 68, 0, 61, 3.5, 0.0, 6.0, 4.5, Some(0.0), Some(6.0), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 4.0);
    }
}

pub fn install() {
    Agent::new("master")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attack100", ssbexo_master_rapid_jab_acmd, Low)
    .acmd("game_attackdash", ssbexo_master_dash_attack_acmd, Low)
    .install()
    ;
}