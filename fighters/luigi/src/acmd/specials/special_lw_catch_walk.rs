use super::*;

//Down Special Catch Walk ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_catch_walk_acmd(_agent: &mut L2CAgentBase) {}

//Down Special Catch Walk Effect
unsafe extern "C" fn ssbexo_luigi_down_special_catch_walk_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Special Catch Walk Sound
unsafe extern "C" fn ssbexo_luigi_down_special_catch_walk_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_luigi_step_left_m"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_luigi_step_right_m"));
    }
}

//Down Special Catch Walk Expression
unsafe extern "C" fn ssbexo_luigi_down_special_catch_walk_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwcatchwalk", ssbexo_luigi_down_special_catch_walk_acmd, Low)
    .acmd("effect_speciallwcatchwalk", ssbexo_luigi_down_special_catch_walk_effect, Low)
    .acmd("sound_speciallwcatchwalk", ssbexo_luigi_down_special_catch_walk_sound, Low)
    .acmd("expression_speciallwcatchwalk", ssbexo_luigi_down_special_catch_walk_expression, Low)
    .install()
    ;
}