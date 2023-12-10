use super::*;

unsafe extern "C" fn ssbexo_tantan_attackshortstartl1_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_COMBO_ENABLE);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_COMBO_ENABLE);
    }
}

pub fn install() {
    Agent::new("tantan")
    .game_acmd("game_attackshortstartl1", ssbexo_tantan_attackshortstartl1_acmd)
    .install()
    ;
}