use super::*;

//Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_roy_neutral_special_start_acmd(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 5.0/10.0);
}

pub fn install() {
    Agent::new("roy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnstart", ssbexo_roy_neutral_special_start_acmd, Low)
    .acmd("game_specialairnstart", ssbexo_roy_neutral_special_start_acmd, Low)
    .install()
    ;
}