use super::*;

//Up Special Catch ACMD
unsafe extern "C" fn ssbexo_armstrong_up_special_catch_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        FT_MOTION_RATE(agent, 30.0/24.0);
    }
}

//Up Special Catch Effect
unsafe extern "C" fn ssbexo_armstrong_up_special_catch_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("armstrong_flame_grab"), Hash40::new("havel"), 3, 0, 0, 0, 0, 0, 1, true);
    }
}

//Up Special Catch Sound
unsafe extern "C" fn ssbexo_armstrong_up_special_catch_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_h02"));
    }
}

//Up Special Catch Expression
unsafe extern "C" fn ssbexo_armstrong_up_special_catch_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 3, 130, 2, 1, 0, 12, 30, 30, 80);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_armstrong_costumes_acmd())
    .game_acmd("game_specialhicatch", ssbexo_armstrong_up_special_catch_acmd, Low)
    .effect_acmd("effect_specialhicatch", ssbexo_armstrong_up_special_catch_effect, Low)
    .sound_acmd("sound_specialhicatch", ssbexo_armstrong_up_special_catch_sound, Low)
    .expression_acmd("expression_specialhicatch", ssbexo_armstrong_up_special_catch_expression, Low)
    .install()
    ;
}