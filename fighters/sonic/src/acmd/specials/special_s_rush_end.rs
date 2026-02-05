use super::*;

//Side Special Rush End ACMD
unsafe extern "C" fn ssbexo_sonic_side_special_rush_end_acmd(_agent: &mut L2CAgentBase) {}

//Side Special Rush End Effect
unsafe extern "C" fn ssbexo_sonic_side_special_rush_end_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Side Special Rush End Sound
unsafe extern "C" fn ssbexo_sonic_side_special_rush_end_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_sonic_special_s02"));
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

//Side Special Rush End Expression
unsafe extern "C" fn ssbexo_sonic_side_special_rush_end_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialsstart", ssbexo_sonic_side_special_rush_end_acmd, Low)
    .effect_acmd("effect_specialsstart", ssbexo_sonic_side_special_rush_end_effect, Low)
    .sound_acmd("sound_specialsstart", ssbexo_sonic_side_special_rush_end_sound, Low)
    .expression_acmd("expression_specialsstart", ssbexo_sonic_side_special_rush_end_expression, Low)
    .game_acmd("game_specialairsstart", ssbexo_sonic_side_special_rush_end_acmd, Low)
    .effect_acmd("effect_specialairsstart", ssbexo_sonic_side_special_rush_end_effect, Low)
    .sound_acmd("sound_specialairsstart", ssbexo_sonic_side_special_rush_end_sound, Low)
    .expression_acmd("expression_specialairsstart", ssbexo_sonic_side_special_rush_end_expression, Low)
    .install()
    ;
}