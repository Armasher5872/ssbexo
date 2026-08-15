use super::*;

//Airdodge ACMD
unsafe extern "C" fn ssbexo_eflame_airdodge_acmd(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("eflame")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_escapeair", ssbexo_eflame_airdodge_acmd, Low)
    .acmd("game_escapeairslide", ssbexo_eflame_airdodge_acmd, Low)
    .install()
    ;
}