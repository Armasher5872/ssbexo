use super::*;

//Dash Attack Throw ACMD
unsafe extern "C" fn ssbexo_snake_dash_attack_throw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ITEM_THROW_DASH, false);
    }
}

//Dash Attack Throw Effect
unsafe extern "C" fn ssbexo_snake_dash_attack_throw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Dash Attack Throw Sound
unsafe extern "C" fn ssbexo_snake_dash_attack_throw_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_attackdash_gear"));
    }
}

//Dash Attack Throw Expression
unsafe extern "C" fn ssbexo_snake_dash_attack_throw_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("snake")
    .game_acmd("game_attackdashthrow", ssbexo_snake_dash_attack_throw_acmd, Priority::Low)
    .effect_acmd("effect_attackdashthrow", ssbexo_snake_dash_attack_throw_effect, Priority::Low)
    .sound_acmd("sound_attackdashthrow", ssbexo_snake_dash_attack_throw_sound, Priority::Low)
    .expression_acmd("expression_attackdashthrow", ssbexo_snake_dash_attack_throw_expression, Priority::Low)
    .install()
    ;
}