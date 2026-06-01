use super::*;

//Neutral Special Charge ACMD
unsafe extern "C" fn ssbexo_armstrong_neutral_special_charge_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Charge Effect
unsafe extern "C" fn ssbexo_armstrong_neutral_special_charge_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_flame_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_flame_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

//Neutral Special Charge Sound
unsafe extern "C" fn ssbexo_armstrong_neutral_special_charge_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_n01"));
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n01"));
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_attack05"));
    }
}

//Neutral Special Charge Expression
unsafe extern "C" fn ssbexo_armstrong_neutral_special_charge_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_23_hold"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_armstrong_costumes_acmd())
    .game_acmd("game_specialncharge", ssbexo_armstrong_neutral_special_charge_acmd, Low)
    .effect_acmd("effect_specialncharge", ssbexo_armstrong_neutral_special_charge_effect, Low)
    .sound_acmd("sound_specialncharge", ssbexo_armstrong_neutral_special_charge_sound, Low)
    .expression_acmd("expression_specialncharge", ssbexo_armstrong_neutral_special_charge_expression, Low)
    .game_acmd("game_specialairncharge", ssbexo_armstrong_neutral_special_charge_acmd, Low)
    .effect_acmd("effect_specialairncharge", ssbexo_armstrong_neutral_special_charge_effect, Low)
    .sound_acmd("sound_specialairncharge", ssbexo_armstrong_neutral_special_charge_sound, Low)
    .expression_acmd("expression_specialairncharge", ssbexo_armstrong_neutral_special_charge_expression, Low)
    .install()
    ;
}