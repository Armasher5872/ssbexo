use super::*;

//Nejiri Uraken Cancel ACMD
unsafe extern "C" fn ssbexo_demon_nejiri_uraken_cancel_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_FLAG_CHECK_STEP);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attacklw3cancel", ssbexo_demon_nejiri_uraken_cancel_acmd, Low)
    .install()
    ;
}