use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_murabito_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BOWLING_BALL, false, -1);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BOWLING_BALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

pub fn install() {
    Agent::new("murabito")
    .game_acmd("game_attacks4", ssbexo_murabito_forward_smash_acmd, Priority::Low)
    .install()
    ;
}