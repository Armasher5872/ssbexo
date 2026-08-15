use super::*;

//Airdodge ACMD
unsafe extern "C" fn ssbexo_springtrap_airdodge_acmd(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_springtrap_costumes_acmd())
    .acmd("game_escapeair", ssbexo_springtrap_airdodge_acmd, Low)
    .acmd("game_escapeairslide", ssbexo_springtrap_airdodge_acmd, Low)
    .install()
    ;
}