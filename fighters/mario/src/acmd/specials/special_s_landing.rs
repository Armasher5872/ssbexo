use super::*;

//Side Special Landing ACMD
unsafe extern "C" fn ssbexo_mario_side_special_landing_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        let sum_speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        SA_SET(agent, *SITUATION_KIND_AIR);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x/1.5, 1.3);
    }
}

//Side Special Landing Effect
unsafe extern "C" fn ssbexo_mario_side_special_landing_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Side Special Landing Sound
unsafe extern "C" fn ssbexo_mario_side_special_landing_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_mario_passive"));
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_mario_landing02"));
    }
}

//Side Special Landing Expression
unsafe extern "C" fn ssbexo_mario_side_special_landing_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialslanding", ssbexo_mario_side_special_landing_acmd, Low)
    .effect_acmd("effect_specialslanding", ssbexo_mario_side_special_landing_effect, Low)
    .sound_acmd("sound_specialslanding", ssbexo_mario_side_special_landing_sound, Low)
    .expression_acmd("expression_specialslanding", ssbexo_mario_side_special_landing_expression, Low)
    .install()
    ;
}