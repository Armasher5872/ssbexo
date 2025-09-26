use super::*;

//Grounded Neutral Special Loop ACMD
unsafe extern "C" fn ssbexo_ganon_grounded_neutral_special_loop_acmd(_agent: &mut L2CAgentBase) {}

//Aerial Neutral Special Loop ACMD
unsafe extern "C" fn ssbexo_ganon_aerial_neutral_special_loop_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.03833);
        sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.65);
    }
}

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
unsafe extern "C" fn ssbexo_ganon_neutral_special_loop_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n04"));
    }
}

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
    .game_acmd("game_specialn", ssbexo_ganon_grounded_neutral_special_loop_acmd, Low)
    .effect_acmd("effect_specialn", ssbexo_ganon_grounded_neutral_special_loop_effect, Low)
    .sound_acmd("sound_specialn", ssbexo_ganon_neutral_special_loop_sound, Low)
    .expression_acmd("expression_specialn", ssbexo_ganon_neutral_special_loop_expression, Low)
    .game_acmd("game_specialairn", ssbexo_ganon_aerial_neutral_special_loop_acmd, Low)
    .effect_acmd("effect_specialairn", ssbexo_ganon_aerial_neutral_special_loop_effect, Low)
    .sound_acmd("sound_specialairn", ssbexo_ganon_neutral_special_loop_sound, Low)
    .expression_acmd("expression_specialairn", ssbexo_ganon_neutral_special_loop_expression, Low)
    .install()
    ;
}