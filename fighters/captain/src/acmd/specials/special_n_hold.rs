use super::*;

unsafe extern "C" fn ssbexo_captain_neutral_special_hold_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 195.0/180.0);
    }  
}

unsafe extern "C" fn ssbexo_captain_neutral_special_hold_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    for _ in 0..10 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 1, true);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 89.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_hold"), Hash40::new("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 1, true);
        FLASH(agent, 1, 1, 0.392, 0.392);
    }
    frame(lua_state, 90.0);
    for _ in 0..10 {
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.392);
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.0);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.392);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.392);
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0, 0, 0.353);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn ssbexo_captain_neutral_special_hold_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_boost_charge"));
    }
}

unsafe extern "C" fn ssbexo_captain_neutral_special_hold_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        physics!(agent, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.9, 0.9, -1, 0.6, 0.5, -1, Hash40::new("invalid"));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 180, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 175.0);
    if is_excute(agent) {
        physics!(agent, *MA_MSC_CMD_PHYSICS_STOP_CHARGE);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 180, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnhold", ssbexo_captain_neutral_special_hold_acmd, Low)
    .acmd("game_specialairnhold", ssbexo_captain_neutral_special_hold_acmd, Low)
    .acmd("effect_specialnhold", ssbexo_captain_neutral_special_hold_effect, Low)
    .acmd("effect_specialairnhold", ssbexo_captain_neutral_special_hold_effect, Low)
    .acmd("sound_specialnhold", ssbexo_captain_neutral_special_hold_sound, Low)
    .acmd("sound_specialairnhold", ssbexo_captain_neutral_special_hold_sound, Low)
    .acmd("expression_specialnhold", ssbexo_captain_neutral_special_hold_expression, Low)
    .acmd("expression_specialairnhold", ssbexo_captain_neutral_special_hold_expression, Low)
    .install()
    ;
}