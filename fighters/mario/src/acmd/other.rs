use super::*;

//Down Taunt Effect
unsafe extern "C" fn ssbexo_mario_down_taunt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_mario_down_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_landing03"));
    }
}

//Down Taunt Expression
unsafe extern "C" fn ssbexo_mario_down_taunt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
}

//Crawl ACMD
unsafe extern "C" fn ssbexo_mario_crawl_acmd(_agent: &mut L2CAgentBase) {}

//Crawl Effect
unsafe extern "C" fn ssbexo_mario_crawl_effect(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 25.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 3, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 41.0);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 3, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Crawl Sound
unsafe extern "C" fn ssbexo_mario_crawl_sound(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 24.0);
        if is_excute(agent) {
            PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_mario_step_right_s"), Hash40::new("se_mario_step_left_s"));
        }
        frame(agent.lua_state_agent, 40.0);
        PLAY_STEP_FLIPPABLE(agent, Hash40::new("se_mario_step_left_s"), Hash40::new("se_mario_step_right_s"));
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Crawl Expression
unsafe extern "C" fn ssbexo_mario_crawl_expression(agent: &mut L2CAgentBase) {
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6);
        }
        frame(agent.lua_state_agent, 6.0);
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 14, true);
        }
        frame(agent.lua_state_agent, 20.0);
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 13, true);
        }
        frame(agent.lua_state_agent, 24.0);
        if is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 33.0);
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 12, true);
        }
        frame(agent.lua_state_agent, 40.0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
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