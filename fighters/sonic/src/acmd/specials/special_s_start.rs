use super::*;

//Side Special Start ACMD
unsafe extern "C" fn ssbexo_sonic_side_special_start_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 19.0/15.0);
    }
}

//Side Special Start Effect
unsafe extern "C" fn ssbexo_sonic_side_special_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Side Special Start Sound
unsafe extern "C" fn ssbexo_sonic_side_special_start_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sonic_swing_l"));
    }
}

//Side Special Start Expression
unsafe extern "C" fn ssbexo_sonic_side_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialsstart", ssbexo_sonic_side_special_start_acmd, Low)
    .effect_acmd("effect_specialsstart", ssbexo_sonic_side_special_start_effect, Low)
    .sound_acmd("sound_specialsstart", ssbexo_sonic_side_special_start_sound, Low)
    .expression_acmd("expression_specialsstart", ssbexo_sonic_side_special_start_expression, Low)
    .game_acmd("game_specialairsstart", ssbexo_sonic_side_special_start_acmd, Low)
    .effect_acmd("effect_specialairsstart", ssbexo_sonic_side_special_start_effect, Low)
    .sound_acmd("sound_specialairsstart", ssbexo_sonic_side_special_start_sound, Low)
    .expression_acmd("expression_specialairsstart", ssbexo_sonic_side_special_start_expression, Low)
    .install()
    ;
}