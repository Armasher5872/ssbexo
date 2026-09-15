use super::*;

//Down Special ACMD
unsafe extern "C" fn ssbexo_chrom_down_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
}

pub fn install() {
    Agent::new("chrom")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallw", ssbexo_chrom_down_special_acmd, Low)
    .acmd("game_specialairlw", ssbexo_chrom_down_special_acmd, Low)
    .install()
    ;
}