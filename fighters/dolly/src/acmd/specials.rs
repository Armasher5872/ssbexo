use super::*;

//Power Geyser ACMD
unsafe extern "C" fn ssbexo_dolly_power_geyser_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 4.0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, -1);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.5);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 0.0, 8.0);
    }
}

//Buster Wolf ACMD
unsafe extern "C" fn ssbexo_dolly_buster_wolf_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        MotionModule::set_rate(boma, 2.0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 0.5);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 2.0);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 45, 100, 100, 30, 6.0, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(boma, true, false);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 45, 100, 100, 30, 6.0, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        ArticleModule::generate_article(boma, *FIGHTER_DOLLY_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
    }
}

pub fn install() {
    Agent::new("dolly")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_superspecial", ssbexo_dolly_power_geyser_acmd, Low)
    .acmd("game_superspecial2start", ssbexo_dolly_buster_wolf_acmd, Low)
    .install()
    ;
}