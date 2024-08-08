use super::*;

//Up Special ACMD
unsafe extern "C" fn ssbexo_reflet_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, -1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    Agent::new("reflet")
    .game_acmd("game_specialhi", ssbexo_reflet_up_special_acmd, Priority::Low)
    .game_acmd("game_specialairhi", ssbexo_reflet_up_special_acmd, Priority::Low)
    .install()
    ;
}