use super::*;

//Grounded Slip Counter Hit ACMD
unsafe extern "C" fn ssbexo_littlemac_grounded_slip_counter_hit_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_START);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Aerial Slip Counter Hit ACMD
unsafe extern "C" fn ssbexo_littlemac_aerial_slip_counter_hit_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_START);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Slip Counter Hit Effect
unsafe extern "C" fn ssbexo_littlemac_slip_counter_hit_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("littlemac_counter_success"), Hash40::new("top"), -1, 11, -5.5, 0, 90, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

//Slip Counter Hit Sound
unsafe extern "C" fn ssbexo_littlemac_slip_counter_hit_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_escape"));
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_special_l02"));
        PLAY_SE(agent, Hash40::new("vc_littlemac_special_l02"));
    }
    wait(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_special_l03"));
    }
}

//Slip Counter Hit Expression
unsafe extern "C" fn ssbexo_littlemac_slip_counter_hit_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwhit", ssbexo_littlemac_grounded_slip_counter_hit_acmd, Low)
    .game_acmd("game_specialairlwhit", ssbexo_littlemac_aerial_slip_counter_hit_acmd, Low)
    .game_acmd("game_speciallwhitf", ssbexo_littlemac_grounded_slip_counter_hit_acmd, Low)
    .game_acmd("game_specialairlwhitf", ssbexo_littlemac_aerial_slip_counter_hit_acmd, Low)
    .effect_acmd("effect_speciallwhit", ssbexo_littlemac_slip_counter_hit_effect, Low)
    .effect_acmd("effect_specialairlwhit", ssbexo_littlemac_slip_counter_hit_effect, Low)
    .effect_acmd("effect_speciallwhitf", ssbexo_littlemac_slip_counter_hit_effect, Low)
    .effect_acmd("effect_specialairlwhitf", ssbexo_littlemac_slip_counter_hit_effect, Low)
    .sound_acmd("sound_speciallwhit", ssbexo_littlemac_slip_counter_hit_sound, Low)
    .sound_acmd("sound_specialairlwhit", ssbexo_littlemac_slip_counter_hit_sound, Low)
    .sound_acmd("sound_speciallwhitf", ssbexo_littlemac_slip_counter_hit_sound, Low)
    .sound_acmd("sound_specialairlwhitf", ssbexo_littlemac_slip_counter_hit_sound, Low)
    .expression_acmd("expression_speciallwhit", ssbexo_littlemac_slip_counter_hit_expression, Low)
    .expression_acmd("expression_specialairlwhit", ssbexo_littlemac_slip_counter_hit_expression, Low)
    .expression_acmd("expression_speciallwhitf", ssbexo_littlemac_slip_counter_hit_expression, Low)
    .expression_acmd("expression_specialairlwhitf", ssbexo_littlemac_slip_counter_hit_expression, Low)
    .install()
    ;
}