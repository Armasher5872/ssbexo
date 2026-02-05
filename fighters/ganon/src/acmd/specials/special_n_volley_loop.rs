use super::*;

//Neutral Special Volley Loop ACMD
unsafe extern "C" fn ssbexo_ganon_neutral_special_volley_loop_acmd(_agent: &mut L2CAgentBase) {}

//Grounded Neutral Special Volley Loop Effect
unsafe extern "C" fn ssbexo_ganon_grounded_neutral_special_volley_loop_effect(_agent: &mut L2CAgentBase) {}

//Aerial Neutral Special Volley Loop Effect
unsafe extern "C" fn ssbexo_ganon_aerial_neutral_special_volley_loop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("footr"), 0, 0, 0, -0.286, -45, 25, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("footl"), 0, 0, 0, -0.286, -45, 25, 1, true);
    }
}

//Neutral Special Volley Loop Sound
unsafe extern "C" fn ssbexo_ganon_neutral_special_volley_loop_sound(_agent: &mut L2CAgentBase) {}

//Neutral Special Volley Loop Expression
unsafe extern "C" fn ssbexo_ganon_neutral_special_volley_loop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        AREA_WIND_2ND_arg10(agent, 0, 2, 75, 2, 1, 0, 12, 50, 30, 50);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_23_hold"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnvolleyloop", ssbexo_ganon_neutral_special_volley_loop_acmd, Low)
    .effect_acmd("effect_specialnvolleyloop", ssbexo_ganon_grounded_neutral_special_volley_loop_effect, Low)
    .sound_acmd("sound_specialnvolleyloop", ssbexo_ganon_neutral_special_volley_loop_sound, Low)
    .expression_acmd("expression_specialnvolleyloop", ssbexo_ganon_neutral_special_volley_loop_expression, Low)
    .game_acmd("game_specialairnvolleyloop", ssbexo_ganon_neutral_special_volley_loop_acmd, Low)
    .effect_acmd("effect_specialairnvolleyloop", ssbexo_ganon_aerial_neutral_special_volley_loop_effect, Low)
    .sound_acmd("sound_specialairnvolleyloop", ssbexo_ganon_neutral_special_volley_loop_sound, Low)
    .expression_acmd("expression_specialairnvolleyloop", ssbexo_ganon_neutral_special_volley_loop_expression, Low)
    .game_acmd("game_specialnvolleymaxstart", ssbexo_ganon_neutral_special_volley_loop_acmd, Low)
    .effect_acmd("effect_specialnvolleymaxstart", ssbexo_ganon_grounded_neutral_special_volley_loop_effect, Low)
    .sound_acmd("sound_specialnvolleymaxstart", ssbexo_ganon_neutral_special_volley_loop_sound, Low)
    .expression_acmd("expression_specialnvolleymaxstart", ssbexo_ganon_neutral_special_volley_loop_expression, Low)
    .game_acmd("game_specialairnvolleymaxstart", ssbexo_ganon_neutral_special_volley_loop_acmd, Low)
    .effect_acmd("effect_specialairnvolleymaxstart", ssbexo_ganon_aerial_neutral_special_volley_loop_effect, Low)
    .sound_acmd("sound_specialairnvolleymaxstart", ssbexo_ganon_neutral_special_volley_loop_sound, Low)
    .expression_acmd("expression_specialairnvolleymaxstart", ssbexo_ganon_neutral_special_volley_loop_expression, Low)
    .game_acmd("game_specialnvolleymax", ssbexo_ganon_neutral_special_volley_loop_acmd, Low)
    .effect_acmd("effect_specialnvolleymax", ssbexo_ganon_grounded_neutral_special_volley_loop_effect, Low)
    .sound_acmd("sound_specialnvolleymax", ssbexo_ganon_neutral_special_volley_loop_sound, Low)
    .expression_acmd("expression_specialnvolleymax", ssbexo_ganon_neutral_special_volley_loop_expression, Low)
    .game_acmd("game_specialairnvolleymax", ssbexo_ganon_neutral_special_volley_loop_acmd, Low)
    .effect_acmd("effect_specialairnvolleymax", ssbexo_ganon_aerial_neutral_special_volley_loop_effect, Low)
    .sound_acmd("sound_specialairnvolleymax", ssbexo_ganon_neutral_special_volley_loop_sound, Low)
    .expression_acmd("expression_specialairnvolleymax", ssbexo_ganon_neutral_special_volley_loop_expression, Low)
    .install()
    ;
}