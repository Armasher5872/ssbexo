use super::*;

//Up Special Catch ACMD
unsafe extern "C" fn ssbexo_armstrong_up_special_catch_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
}

//Up Special Catch Effect
unsafe extern "C" fn ssbexo_armstrong_up_special_catch_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("sys_catch"), Hash40::new("haver"), 1, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("armstrong_flame_grab"), Hash40::new("haver"), 3, 0, 0, 0, 0, 0, 1, true);
    }
}

//Up Special Catch Sound
unsafe extern "C" fn ssbexo_armstrong_up_special_catch_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_h02"));
    }
}

//Up Special Catch Expression
unsafe extern "C" fn ssbexo_armstrong_up_special_catch_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 3, 130, 2, 1, 0, 12, 30, 30, 80);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .game_acmd("game_specialhicatch", ssbexo_armstrong_up_special_catch_acmd, Low)
    .effect_acmd("effect_specialhicatch", ssbexo_armstrong_up_special_catch_effect, Low)
    .sound_acmd("sound_specialhicatch", ssbexo_armstrong_up_special_catch_sound, Low)
    .expression_acmd("expression_specialhicatch", ssbexo_armstrong_up_special_catch_expression, Low)
    .install()
    ;
}