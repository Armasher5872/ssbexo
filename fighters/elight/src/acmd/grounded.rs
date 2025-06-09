use super::*;

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_elight_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 10.0);
    if IS_EXIST_ARTICLE(agent, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(agent.lua_state_agent, 12.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.5, 361, 55, 0, 72, 5.0, 0.0, 7.0, 13.0, Some(0.0), Some(7.0), Some(17.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 361, 54, 0, 72, 5.0, 0.0, 7.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if IS_EXIST_ARTICLE(agent, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    FT_MOTION_RATE(agent, 0.75);
}

pub fn install() {
    Agent::new("elight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackdash", ssbexo_elight_dash_attack_acmd, Low)
    .install()
    ;
}