use super::*;

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_murabito_dash_attack_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT, false, -1);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
}

pub fn install() {
    Agent::new("murabito")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackdash", ssbexo_murabito_dash_attack_acmd, Low)
    .install()
    ;
}