use super::*;

unsafe extern "C" fn ssbexo_wario_grounded_down_special_launch_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    }
}

unsafe extern "C" fn ssbexo_wario_grounded_down_special_launch_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

unsafe extern "C" fn ssbexo_wario_grounded_down_special_launch_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_l04"));
    }
}

unsafe extern "C" fn ssbexo_wario_grounded_down_special_launch_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwlaunch", ssbexo_wario_grounded_down_special_launch_acmd, Low)
    .effect_acmd("effect_speciallwlaunch", ssbexo_wario_grounded_down_special_launch_effect, Low)
    .sound_acmd("sound_speciallwlaunch", ssbexo_wario_grounded_down_special_launch_sound, Low)
    .expression_acmd("expression_speciallwlaunch", ssbexo_wario_grounded_down_special_launch_expression, Low)
    .install()
    ;
}