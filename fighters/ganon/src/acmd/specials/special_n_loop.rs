use super::*;

//Neutral Special Loop ACMD
unsafe extern "C" fn ssbexo_ganon_neutral_special_loop_acmd(_agent: &mut L2CAgentBase) {}

//Grounded Neutral Special Loop Effect
unsafe extern "C" fn ssbexo_ganon_grounded_neutral_special_loop_effect(_agent: &mut L2CAgentBase) {}

//Aerial Neutral Special Loop Effect
unsafe extern "C" fn ssbexo_ganon_aerial_neutral_special_loop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("footr"), 0, 0, 0, -0.286, -45, 25, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("footl"), 0, 0, 0, -0.286, -45, 25, 1, true);
    }
}

//Neutral Special Loop Sound
unsafe extern "C" fn ssbexo_ganon_neutral_special_loop_sound(_agent: &mut L2CAgentBase) {}

//Neutral Special Loop Expression
unsafe extern "C" fn ssbexo_ganon_neutral_special_loop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        AREA_WIND_2ND_arg10(agent, 0, 2, 75, 2, 1, 0, 12, 50, 30, 50);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_23_hold"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn", ssbexo_ganon_neutral_special_loop_acmd, Low)
    .effect_acmd("effect_specialn", ssbexo_ganon_grounded_neutral_special_loop_effect, Low)
    .sound_acmd("sound_specialn", ssbexo_ganon_neutral_special_loop_sound, Low)
    .expression_acmd("expression_specialn", ssbexo_ganon_neutral_special_loop_expression, Low)
    .game_acmd("game_specialairn", ssbexo_ganon_neutral_special_loop_acmd, Low)
    .effect_acmd("effect_specialairn", ssbexo_ganon_aerial_neutral_special_loop_effect, Low)
    .sound_acmd("sound_specialairn", ssbexo_ganon_neutral_special_loop_sound, Low)
    .expression_acmd("expression_specialairn", ssbexo_ganon_neutral_special_loop_expression, Low)
    .game_acmd("game_specialnmaxstart", ssbexo_ganon_neutral_special_loop_acmd, Low)
    .effect_acmd("effect_specialnmaxstart", ssbexo_ganon_grounded_neutral_special_loop_effect, Low)
    .sound_acmd("sound_specialnmaxstart", ssbexo_ganon_neutral_special_loop_sound, Low)
    .expression_acmd("expression_specialnmaxstart", ssbexo_ganon_neutral_special_loop_expression, Low)
    .game_acmd("game_specialairnmaxstart", ssbexo_ganon_neutral_special_loop_acmd, Low)
    .effect_acmd("effect_specialairnmaxstart", ssbexo_ganon_aerial_neutral_special_loop_effect, Low)
    .sound_acmd("sound_specialairnmaxstart", ssbexo_ganon_neutral_special_loop_sound, Low)
    .expression_acmd("expression_specialairnmaxstart", ssbexo_ganon_neutral_special_loop_expression, Low)
    .game_acmd("game_specialnmax", ssbexo_ganon_neutral_special_loop_acmd, Low)
    .effect_acmd("effect_specialnmax", ssbexo_ganon_grounded_neutral_special_loop_effect, Low)
    .sound_acmd("sound_specialnmax", ssbexo_ganon_neutral_special_loop_sound, Low)
    .expression_acmd("expression_specialnmax", ssbexo_ganon_neutral_special_loop_expression, Low)
    .game_acmd("game_specialairnmax", ssbexo_ganon_neutral_special_loop_acmd, Low)
    .effect_acmd("effect_specialairnmax", ssbexo_ganon_aerial_neutral_special_loop_effect, Low)
    .sound_acmd("sound_specialairnmax", ssbexo_ganon_neutral_special_loop_sound, Low)
    .expression_acmd("expression_specialairnmax", ssbexo_ganon_neutral_special_loop_expression, Low)
    .install()
    ;
}