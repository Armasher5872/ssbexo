use super::*;

//Down Taunt ACMD
unsafe extern "C" fn ssbexo_younglink_down_taunt_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.333);
    }
}

//Down Taunt Expression
unsafe extern "C" fn ssbexo_younglink_down_taunt_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_MILK, true, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_MILK, Hash40::new("appeal_lw_r"), false, -1.0);
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_MILK, 0.333);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits_l"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 95.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_MILK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 98.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

pub fn install() {
    Agent::new("younglink")
    .game_acmd("game_appeallwl", ssbexo_younglink_down_taunt_acmd, Priority::Low)
    .game_acmd("game_appeallwr", ssbexo_younglink_down_taunt_acmd, Priority::Low)
    .expression_acmd("expression_appeallwl", ssbexo_younglink_down_taunt_expression, Priority::Low)
    .expression_acmd("expression_appeallwr", ssbexo_younglink_down_taunt_expression, Priority::Low)
    .install()
    ;
}