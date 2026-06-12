use super::*;

//Down Special Catch Jump ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_catch_jump_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
        let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);
        let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
        ADD_SPEED_NO_LIMIT(agent, 0, 2.0);
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y);
        sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable*0.7);
        sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable*0.7);
        sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, air_speed_x_stable*0.7);
        sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, air_speed_x_stable*0.7);
        sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*0.7);
        sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*0.7);
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
    .acmd("game_speciallwcatchjump", ssbexo_luigi_down_special_catch_jump_acmd, Low)
    .acmd("effect_speciallwcatchjump", ssbexo_luigi_down_special_catch_jump_effect, Low)
    .acmd("sound_speciallwcatchjump", ssbexo_luigi_down_special_catch_jump_sound, Low)
    .acmd("expression_speciallwcatchjump", ssbexo_luigi_down_special_catch_jump_expression, Low)
    .install()
    ;
}