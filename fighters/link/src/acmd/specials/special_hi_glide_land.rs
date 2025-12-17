use super::*;

//Up Special Glide Land ACMD
unsafe extern "C" fn ssbexo_link_special_hi_glide_land_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Up Special Glide Land Expression
unsafe extern "C" fn ssbexo_link_special_hi_glide_land_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhiglideland", ssbexo_link_special_hi_glide_land_acmd, Low)
    .expression_acmd("expression_specialhiglideland", ssbexo_link_special_hi_glide_land_expression, Low)
    .install()
    ;
}