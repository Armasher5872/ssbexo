use super::*;

//Punisher Squat Effect
unsafe extern "C" fn ssbexo_cloud_punisher_squat_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
    }
}

//Punisher Squat Sound
unsafe extern "C" fn ssbexo_cloud_punisher_squat_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_squat"));
    }
}

//Punisher Squat Expression
unsafe extern "C" fn ssbexo_cloud_punisher_squat_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_r") as i64);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_punishsquat", ssbexo_cloud_punisher_squat_effect, Low)
    .acmd("sound_punishsquat", ssbexo_cloud_punisher_squat_sound, Low)
    .acmd("expression_punishsquat", ssbexo_cloud_punisher_squat_expression, Low)
    .install()
    ;
}