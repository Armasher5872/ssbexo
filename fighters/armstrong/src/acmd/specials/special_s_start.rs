use super::*;

//Grounded Side Special Start ACMD
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_start_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUN);
    }
}

//Grounded Side Special Start Effect
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Grounded Side Special Start Sound
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_start_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_s01"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_s02"));
    }
}

//Grounded Side Special Start Expression
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_start_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Side Special Start ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_start_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUN);
    }
}

//Aerial Side Special Start Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Side Special Start Sound
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_start_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_s01"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_s02"));
    }
}

//Aerial Side Special Start Expression
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_start_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_armstrong_costumes_acmd())
    .game_acmd("game_specialsstart", ssbexo_armstrong_grounded_side_special_start_acmd, Low)
    .effect_acmd("effect_specialsstart", ssbexo_armstrong_grounded_side_special_start_effect, Low)
    .sound_acmd("sound_specialsstart", ssbexo_armstrong_grounded_side_special_start_sound, Low)
    .expression_acmd("expression_specialsstart", ssbexo_armstrong_grounded_side_special_start_expression, Low)
    .game_acmd("game_specialairsstart", ssbexo_armstrong_aerial_side_special_start_acmd, Low)
    .effect_acmd("effect_specialairsstart", ssbexo_armstrong_aerial_side_special_start_effect, Low)
    .sound_acmd("sound_specialairsstart", ssbexo_armstrong_aerial_side_special_start_sound, Low)
    .expression_acmd("expression_specialairsstart", ssbexo_armstrong_aerial_side_special_start_expression, Low)
    .install()
    ;
}