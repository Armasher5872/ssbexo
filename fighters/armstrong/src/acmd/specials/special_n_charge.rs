use super::*;

//Neutral Special Charge ACMD
unsafe extern "C" fn ssbexo_armstrong_neutral_special_charge_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Charge Effect
unsafe extern "C" fn ssbexo_armstrong_neutral_special_charge_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_flame_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_flame_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

//Neutral Special Charge Sound
unsafe extern "C" fn ssbexo_armstrong_neutral_special_charge_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_n01"));
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n01"));
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_attack05"));
    }
}

//Neutral Special Charge Expression
unsafe extern "C" fn ssbexo_armstrong_neutral_special_charge_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_23_hold"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    Agent::new("ganon")
    .set_costume(costume.to_vec())
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