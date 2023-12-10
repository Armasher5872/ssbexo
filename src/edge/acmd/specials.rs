use super::*;

//Flare Charge ACMD
unsafe extern "C" fn ssbexo_edge_flare_charge_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_EDGE_SPECIAL_N_S, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_EDGE_SPECIAL_N_M, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(agent.lua_state_agent, 79.0);
    MotionModule::set_rate(agent.module_accessor, 0.8);
    frame(agent.lua_state_agent, 99.0);
    MotionModule::set_rate(agent.module_accessor, 1.0);
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_EDGE_SPECIAL_N_L, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(agent.lua_state_agent, 105.0);
    MotionModule::set_rate(agent.module_accessor, 0.625);
    frame(agent.lua_state_agent, 115.0);
    MotionModule::set_rate(agent.module_accessor, 1.0);
    frame(agent.lua_state_agent, 120.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, 0);
    }
    frame(agent.lua_state_agent, 140.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}

//Flare ACMD
unsafe extern "C" fn ssbexo_edge_flare_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 82, 15, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 4.5);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 5.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 5.5);
    }
}

pub fn install() {
    Agent::new("edge")
    .game_acmd("game_specialnstart", ssbexo_edge_flare_charge_acmd)
    .game_acmd("game_specialairnstart", ssbexo_edge_flare_charge_acmd)
    .install()
    ;
    Agent::new("edge_fire")
    .game_acmd("game_specialn1", ssbexo_edge_flare_acmd)
    .install()
    ;
}