use super::*;

//Grounded Up Special End ACMD
unsafe extern "C" fn ssbexo_ganon_grounded_up_special_end_acmd(_agent: &mut L2CAgentBase) {}

//Aerial Up Special End ACMD
unsafe extern "C" fn ssbexo_ganon_aerial_up_special_end_acmd(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
        if is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    frame(agent.lua_state_agent, 38.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
        if is_excute(agent) {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    frame(agent.lua_state_agent, 45.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
        if is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    frame(agent.lua_state_agent, 48.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
        if is_excute(agent) {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
}

//Grounded Up Special End Effect
unsafe extern "C" fn ssbexo_ganon_grounded_up_special_end_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.8);
        BURN_COLOR(agent, 0.2, 0, 1.7, 0.4);
        ColorBlendModule::set_disable_camera_depth_influence(agent.module_accessor, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        BURN_COLOR_FRAME(agent, 15, 0.2, 0, 1.7, 0);
        FLASH_FRM(agent, 15, 0, 0, 0, 0);
    }
    wait(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
        ColorBlendModule::set_disable_camera_depth_influence(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Up Special End Effect
unsafe extern "C" fn ssbexo_ganon_aerial_up_special_end_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.8);
        BURN_COLOR(agent, 0.2, 0, 1.7, 0.4);
        ColorBlendModule::set_disable_camera_depth_influence(agent.module_accessor, true);
        EFFECT(agent, Hash40::new("ganon_entry"), Hash40::new("top"), 6, 15, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        BURN_COLOR_FRAME(agent, 15, 0.2, 0, 1.7, 0);
        FLASH_FRM(agent, 15, 0, 0, 0, 0);
    }
    wait(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
        ColorBlendModule::set_disable_camera_depth_influence(agent.module_accessor, false);
    }
}

//Grounded Up Special End Sound
unsafe extern "C" fn ssbexo_ganon_grounded_up_special_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_ganon_landing02"));
    }
}

//Aerial Up Special End Sound
unsafe extern "C" fn ssbexo_ganon_aerial_up_special_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
}

//Up Special End Expression
unsafe extern "C" fn ssbexo_ganon_up_special_end_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhiend", ssbexo_ganon_grounded_up_special_end_acmd, Low)
    .effect_acmd("effect_specialhiend", ssbexo_ganon_grounded_up_special_end_effect, Low)
    .sound_acmd("sound_specialhiend", ssbexo_ganon_grounded_up_special_end_sound, Low)
    .expression_acmd("expression_specialhiend", ssbexo_ganon_up_special_end_expression, Low)
    .game_acmd("game_specialairhiend", ssbexo_ganon_aerial_up_special_end_acmd, Low)
    .effect_acmd("effect_specialairhiend", ssbexo_ganon_aerial_up_special_end_effect, Low)
    .sound_acmd("sound_specialairhiend", ssbexo_ganon_aerial_up_special_end_sound, Low)
    .expression_acmd("expression_specialairhiend", ssbexo_ganon_up_special_end_expression, Low)
    .game_acmd("game_specialairhiendcharged", ssbexo_ganon_aerial_up_special_end_acmd, Low)
    .effect_acmd("effect_specialairhiendcharged", ssbexo_ganon_aerial_up_special_end_effect, Low)
    .sound_acmd("sound_specialairhiendcharge", ssbexo_ganon_aerial_up_special_end_sound, Low)
    .expression_acmd("expression_specialairhiendcharged", ssbexo_ganon_up_special_end_expression, Low)
    .install()
    ;
}