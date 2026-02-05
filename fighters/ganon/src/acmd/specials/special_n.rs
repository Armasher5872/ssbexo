use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_ganon_neutral_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_N_TRANSITION_ENABLE);
    }
}

//Neutral Special Effect
unsafe extern "C" fn ssbexo_ganon_neutral_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
}

//Neutral Special Sound
unsafe extern "C" fn ssbexo_ganon_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        let swipe = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ganon_special_n07"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swipe as i32, 3.0, 0);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_jump02"));
    }
}

//Neutral Special Expression
unsafe extern "C" fn ssbexo_ganon_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 1, 0, 300, 0.5, 12, 10, 30, 20, 50);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn", ssbexo_ganon_neutral_special_acmd, Low)
    .effect_acmd("effect_specialn", ssbexo_ganon_neutral_special_effect, Low)
    .sound_acmd("sound_specialn", ssbexo_ganon_neutral_special_sound, Low)
    .expression_acmd("expression_specialn", ssbexo_ganon_neutral_special_expression, Low)
    .game_acmd("game_specialairn", ssbexo_ganon_neutral_special_acmd, Low)
    .effect_acmd("effect_specialairn", ssbexo_ganon_neutral_special_effect, Low)
    .sound_acmd("sound_specialairn", ssbexo_ganon_neutral_special_sound, Low)
    .expression_acmd("expression_specialairn", ssbexo_ganon_neutral_special_expression, Low)
    .install()
    ;
}