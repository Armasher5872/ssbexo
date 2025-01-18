use super::*;

//Turn Dash ACMD
unsafe extern "C" fn ssbexo_robot_turn_dash_acmd(agent: &mut L2CAgentBase) {
    let snake_speed_value = WorkModule::get_float(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
    let snake_speed = Vector3f{x: snake_speed_value*PostureModule::lr(agent.module_accessor), y: 0.0, z: 0.0};
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE) {
            KineticModule::add_speed(agent.module_accessor, &snake_speed);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

//Turn Dash Effect
unsafe extern "C" fn ssbexo_robot_turn_dash_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("robot_nozzle"), Hash40::new("knee1"), 1.2, 0, 0, 90, -90, 0, 1, true);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("knee1"), 1.2, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_spark"), Hash40::new("knee1"), 1.2, 0, 0, 90, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Turn Run ACMD
unsafe extern "C" fn ssbexo_robot_turn_run_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::unable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_STOP);
    }
}

pub fn install() {
    Agent::new("robot")
    .game_acmd("game_turndash", ssbexo_robot_turn_dash_acmd, Priority::Low)
    .effect_acmd("effect_turndash", ssbexo_robot_turn_dash_effect, Priority::Low)
    .game_acmd("game_turnrun", ssbexo_robot_turn_run_acmd, Priority::Low)
    .install()
    ;
}