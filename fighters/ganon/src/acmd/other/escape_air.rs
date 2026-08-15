use super::*;

//Airdodge ACMD
unsafe extern "C" fn ssbexo_ganon_airdodge_acmd(_agent: &mut L2CAgentBase) {}

//Air Dodge Effect
unsafe extern "C" fn ssbexo_ganon_airdodge_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_escapeair", ssbexo_ganon_airdodge_acmd, Low)
    .acmd("game_escapeairslide", ssbexo_ganon_airdodge_acmd, Low)
    .acmd("effect_escapeair", ssbexo_ganon_airdodge_effect, Low)
    .acmd("effect_escapeairslide", ssbexo_ganon_airdodge_effect, Low)
    .install()
    ;
}