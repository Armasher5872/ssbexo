use super::*;

//Fire Teraflare Fly ACMD
unsafe extern "C" fn ssbexo_edge_fire_teraflare_fly_acmd(_agent: &mut L2CAgentBase) {}

//Fire Teraflare Fly Effect
unsafe extern "C" fn ssbexo_edge_fire_teraflare_fly_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire4_bullet"), Hash40::new("top"), 0, -1, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_SCALE_W(agent, 1.0, 0.5, 1.0);
    }
}

//Fire Teraflare Fly Sound
unsafe extern "C" fn ssbexo_edge_fire_teraflare_fly_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 94.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_edge_special_n05_03"));
    }
}

pub fn install() {
    Agent::new("edge_fire")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialn4", ssbexo_edge_fire_teraflare_fly_acmd, Low)
    .acmd("effect_specialn4", ssbexo_edge_fire_teraflare_fly_effect, Low)
    .acmd("sound_specialn4", ssbexo_edge_fire_teraflare_fly_sound, Low)
    .install()
    ;
}