use super::*;

//Airdodge ACMD
unsafe extern "C" fn ssbexo_master_airdodge_acmd(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("master")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_escapeair", ssbexo_master_airdodge_acmd, Low)
    .acmd("game_escapeairslide", ssbexo_master_airdodge_acmd, Low)
    .install()
    ;
}