use super::*;

//Side Special Bonk ACMD
unsafe extern "C" fn ssbexo_mario_side_special_bonk_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, -1.8*lr, 0.0);
        sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.05);
    }
}

//Side Special Bonk Effect
unsafe extern "C" fn ssbexo_mario_side_special_bonk_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_step_jump"), Hash40::new("top"), 0, 0, -7, 0, 0, 180, 1.5, false);
    }
}

//Side Special Bonk Sound
unsafe extern "C" fn ssbexo_mario_side_special_bonk_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_camerahit"));
        PLAY_SE(agent, Hash40::new("vc_mario_missfoot01"));
    }
}

//Side Special Bonk Expression
unsafe extern "C" fn ssbexo_mario_side_special_bonk_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialsbonk", ssbexo_mario_side_special_bonk_acmd, Low)
    .effect_acmd("effect_specialsbonk", ssbexo_mario_side_special_bonk_effect, Low)
    .sound_acmd("sound_specialsbonk", ssbexo_mario_side_special_bonk_sound, Low)
    .expression_acmd("expression_specialsbonk", ssbexo_mario_side_special_bonk_expression, Low)
    .install()
    ;
}