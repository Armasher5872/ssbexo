use super::*;

//Neutral Special Catch Jump ACMD
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_jump_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        let air_accel_y = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("air_accel_y"));
        ADD_SPEED_NO_LIMIT(agent, 0, 2.6);
        sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y*0.55);
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
}

//Neutral Special Catch Jump Effect
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_jump_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Neutral Special Catch Jump Sound
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_jump_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_wario_rnd_jump"));
        PLAY_SE(agent, Hash40::new("se_wario_jump01"));
    }
}

//Neutral Special Catch Jump Expression
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_jump_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialncatchjump", ssbexo_wario_neutral_special_catch_jump_acmd, Low)
    .effect_acmd("effect_specialncatchjump", ssbexo_wario_neutral_special_catch_jump_effect, Low)
    .sound_acmd("sound_specialncatchjump", ssbexo_wario_neutral_special_catch_jump_sound, Low)
    .expression_acmd("expression_specialncatchjump", ssbexo_wario_neutral_special_catch_jump_expression, Low)
    .install()
    ;
}