use super::*;

//Down Special Loop ACMD
unsafe extern "C" fn ssbexo_ike_down_special_loop_acmd(agent: &mut L2CAgentBase) {
    let mut armor_value = 5.0;
    for _ in 0..24 {
        if is_excute(agent) {
            armor_value += 1.0;
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, armor_value);
        }
        wait(agent.lua_state_agent, 5.0);
    }
    frame(agent.lua_state_agent, 120.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
}

pub fn install() {
    Agent::new("ike")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnloop", ssbexo_ike_down_special_loop_acmd, Low)
    .game_acmd("game_specialairnloop", ssbexo_ike_down_special_loop_acmd, Low)
    .install()
    ;
}