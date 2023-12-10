use super::*;

unsafe extern "C" fn ssbexo_tantan_attacklongstart1_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.5, 3.5);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_START_HOLD);
    }
}

pub fn install() {
    Agent::new("tantan")
    .game_acmd("game_attacklongstartl1", ssbexo_tantan_attacklongstart1_acmd)
    .game_acmd("game_attacklongstartlb1", ssbexo_tantan_attacklongstart1_acmd)
    .game_acmd("game_attacklongstartlr1", ssbexo_tantan_attacklongstart1_acmd)
    .game_acmd("game_attacklongstartlrb1", ssbexo_tantan_attacklongstart1_acmd)
    .install()
    ;
}