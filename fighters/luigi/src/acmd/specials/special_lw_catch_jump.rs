use super::*;

//Down Special Catch Jump ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_catch_jump_acmd(agent: &mut L2CAgentBase) {
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

//Down Special Catch Jump Effect
unsafe extern "C" fn ssbexo_luigi_down_special_catch_jump_effect(_agent: &mut L2CAgentBase) {}

//Down Special Catch Jump Sound
unsafe extern "C" fn ssbexo_luigi_down_special_catch_jump_sound(_agent: &mut L2CAgentBase) {}

//Down Special Catch Jump Expression
unsafe extern "C" fn ssbexo_luigi_down_special_catch_jump_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwcatchjump", ssbexo_luigi_down_special_catch_jump_acmd, Low)
    .effect_acmd("effect_speciallwcatchjump", ssbexo_luigi_down_special_catch_jump_effect, Low)
    .sound_acmd("sound_speciallwcatchjump", ssbexo_luigi_down_special_catch_jump_sound, Low)
    .expression_acmd("expression_speciallwcatchjump", ssbexo_luigi_down_special_catch_jump_expression, Low)
    .install()
    ;
}