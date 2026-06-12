use super::*;

unsafe extern "C" fn ssbexo_captain_aerial_down_special_bounce_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 30.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn ssbexo_captain_aerial_down_special_bounce_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 7.5, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn ssbexo_captain_aerial_down_special_bounce_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_h03"));
        PLAY_SE(agent, Hash40::new("vc_captain_appeal03"));
    }
    wait(lua_state, 40.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_s01"));
    }
}

unsafe extern "C" fn ssbexo_captain_aerial_down_special_bounce_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialairlwbounce", ssbexo_captain_aerial_down_special_bounce_acmd, Low)
    .acmd("effect_specialairlwbounce", ssbexo_captain_aerial_down_special_bounce_effect, Low)
    .acmd("sound_specialairlwbounce", ssbexo_captain_aerial_down_special_bounce_sound, Low)
    .acmd("expression_specialairlwbounce", ssbexo_captain_aerial_down_special_bounce_expression, Low)
    .install()
    ;
}