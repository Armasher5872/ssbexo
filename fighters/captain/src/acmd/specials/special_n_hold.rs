use super::*;

unsafe extern "C" fn ssbexo_captain_neutral_special_hold_acmd(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn ssbexo_captain_neutral_special_hold_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_hold"), Hash40::new("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 1, true);
        FLASH(agent, 1, 1, 0.392, 0.392);
    }
    frame(agent.lua_state_agent, 1.0);
    for _ in 0..60 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

unsafe extern "C" fn ssbexo_captain_neutral_special_hold_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    for _ in 0..3 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_captain_special_n04"));
        }
        wait(agent.lua_state_agent, 30.0);
    }
    frame(agent.lua_state_agent, 95.0);
    for _ in 0..6 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_captain_special_n04"));
        }
        wait(agent.lua_state_agent, 15.0);
    }
}

unsafe extern "C" fn ssbexo_captain_neutral_special_hold_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 180, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnhold", ssbexo_captain_neutral_special_hold_acmd, Low)
    .game_acmd("game_specialairnhold", ssbexo_captain_neutral_special_hold_acmd, Low)
    .effect_acmd("effect_specialnhold", ssbexo_captain_neutral_special_hold_effect, Low)
    .effect_acmd("effect_specialairnhold", ssbexo_captain_neutral_special_hold_effect, Low)
    .sound_acmd("sound_specialnhold", ssbexo_captain_neutral_special_hold_sound, Low)
    .sound_acmd("sound_specialairnhold", ssbexo_captain_neutral_special_hold_sound, Low)
    .expression_acmd("expression_specialnhold", ssbexo_captain_neutral_special_hold_expression, Low)
    .expression_acmd("expression_specialairnhold", ssbexo_captain_neutral_special_hold_expression, Low)
    .install()
    ;
}