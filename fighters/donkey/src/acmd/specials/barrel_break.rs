use super::*;

//Barrel Break Sound
unsafe extern "C" fn ssbexo_donkey_barrel_break_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_barrel_break"));
    }
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .sound_acmd("sound_break", ssbexo_donkey_barrel_break_sound, Low)
    .install()
    ;
}