use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_armstrong_neutral_special_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Effect
unsafe extern "C" fn ssbexo_armstrong_neutral_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_flame_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

//Neutral Special Sound
unsafe extern "C" fn ssbexo_armstrong_neutral_special_sound(_agent: &mut L2CAgentBase) {}

//Neutral Special Expression
unsafe extern "C" fn ssbexo_armstrong_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
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
    .game_acmd("game_specialn", ssbexo_armstrong_neutral_special_acmd, Low)
    .effect_acmd("effect_specialn", ssbexo_armstrong_neutral_special_effect, Low)
    .sound_acmd("sound_specialn", ssbexo_armstrong_neutral_special_sound, Low)
    .expression_acmd("expression_specialn", ssbexo_armstrong_neutral_special_expression, Low)
    .game_acmd("game_specialairn", ssbexo_armstrong_neutral_special_acmd, Low)
    .effect_acmd("effect_specialairn", ssbexo_armstrong_neutral_special_effect, Low)
    .sound_acmd("sound_specialairn", ssbexo_armstrong_neutral_special_sound, Low)
    .expression_acmd("expression_specialairn", ssbexo_armstrong_neutral_special_expression, Low)
    .install()
    ;
}