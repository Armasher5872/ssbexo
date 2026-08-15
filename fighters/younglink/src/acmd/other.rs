use super::*;

//Airdodge ACMD
unsafe extern "C" fn ssbexo_younglink_airdodge_acmd(_agent: &mut L2CAgentBase) {}

//Down Taunt ACMD
unsafe extern "C" fn ssbexo_younglink_down_taunt_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        MotionModule::set_rate(boma, 0.333);
    }
}

//Down Taunt Expression
unsafe extern "C" fn ssbexo_younglink_down_taunt_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ArticleModule::generate_article(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_MILK, true, -1);
        ArticleModule::change_motion(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_MILK, Hash40::new("appeal_lw_r"), false, -1.0);
        ArticleModule::set_rate(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_MILK, 0.333);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits_l"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 95.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_MILK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 98.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("shield") as i64, hash40("shield_normal") as i64);
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

pub fn install() {
    Agent::new("younglink")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_escapeair", ssbexo_younglink_airdodge_acmd, Low)
    .acmd("game_escapeairslide", ssbexo_younglink_airdodge_acmd, Low)
    .acmd("game_appeallwl", ssbexo_younglink_down_taunt_acmd, Low)
    .acmd("game_appeallwr", ssbexo_younglink_down_taunt_acmd, Low)
    .acmd("expression_appeallwl", ssbexo_younglink_down_taunt_expression, Low)
    .acmd("expression_appeallwr", ssbexo_younglink_down_taunt_expression, Low)
    .install()
    ;
}