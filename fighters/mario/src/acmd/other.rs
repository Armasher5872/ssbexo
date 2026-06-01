use super::*;

//Down Taunt Effect
unsafe extern "C" fn ssbexo_mario_down_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 29.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_mario_down_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_appeal_l01"));
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_landing03"));
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_appeal_l02"));
    }
}

//Down Taunt Expression
unsafe extern "C" fn ssbexo_mario_down_taunt_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
}

//Crawl ACMD
unsafe extern "C" fn ssbexo_mario_crawl_acmd(_agent: &mut L2CAgentBase) {}

//Crawl Effect
unsafe extern "C" fn ssbexo_mario_crawl_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 25.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 3, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 41.0);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 3, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        wait_loop_clear(agent);
    }
}

//Crawl Sound
unsafe extern "C" fn ssbexo_mario_crawl_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 24.0);
        if is_excute(agent) {
            PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_mario_step_right_s"), Hash40::new("se_mario_step_left_s"));
        }
        frame(lua_state, 40.0);
        PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_mario_step_left_s"), Hash40::new("se_mario_step_right_s"));
        wait_loop_clear(agent);
    }
}

//Crawl Expression
unsafe extern "C" fn ssbexo_mario_crawl_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6);
        }
        frame(lua_state, 6.0);
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 14, true);
        }
        frame(lua_state, 20.0);
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 13, true);
        }
        frame(lua_state, 24.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 33.0);
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 12, true);
        }
        frame(lua_state, 40.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        wait_loop_clear(agent);
    }
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_appeallwr", ssbexo_mario_down_taunt_effect, Low)
    .sound_acmd("sound_appeallwr", ssbexo_mario_down_taunt_sound, Low)
    .expression_acmd("expression_appeallwr", ssbexo_mario_down_taunt_expression, Low)
    .effect_acmd("effect_appeallwl", ssbexo_mario_down_taunt_effect, Low)
    .sound_acmd("sound_appeallwl", ssbexo_mario_down_taunt_sound, Low)
    .expression_acmd("expression_appeallwl", ssbexo_mario_down_taunt_expression, Low)
    .game_acmd("game_squatf", ssbexo_mario_crawl_acmd, Low)
    .effect_acmd("effect_squatf", ssbexo_mario_crawl_effect, Low)
    .sound_acmd("sound_squatf", ssbexo_mario_crawl_sound, Low)
    .expression_acmd("expression_squatf", ssbexo_mario_crawl_expression, Low)
    .game_acmd("game_squatb", ssbexo_mario_crawl_acmd, Low)
    .effect_acmd("effect_squatb", ssbexo_mario_crawl_effect, Low)
    .sound_acmd("sound_squatb", ssbexo_mario_crawl_sound, Low)
    .expression_acmd("expression_squatb", ssbexo_mario_crawl_expression, Low)
    .install()
    ;
}