use super::*;

//Up Taunt ACMD
unsafe extern "C" fn ssbexo_link_up_taunt_acmd(_agent: &mut L2CAgentBase) {}

//Up Taunt Effect
unsafe extern "C" fn ssbexo_link_up_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_link_up_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_appeal_h01"));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_appeal_h02"));
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_swing_s"));
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_swing_s"));
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_swing_s"));
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_appeal_h03"));
    }
}

//Up Taunt Expression
unsafe extern "C" fn ssbexo_link_up_taunt_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        VisibilityModule::set_int64(boma, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beams"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 105.0);
    if is_excute(agent) {
        if ItemModule::is_have_item(boma, 0) {
            ItemModule::set_have_item_visibility(boma, true, 0);
        }
        else {
            VisibilityModule::set_int64(boma, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_appealhir", ssbexo_link_up_taunt_acmd, Low)
    .acmd("effect_appealhir", ssbexo_link_up_taunt_effect, Low)
    .acmd("sound_appealhir", ssbexo_link_up_taunt_sound, Low)
    .acmd("expression_appealhir", ssbexo_link_up_taunt_expression, Low)
    .acmd("game_appealhil", ssbexo_link_up_taunt_acmd, Low)
    .acmd("effect_appealhil", ssbexo_link_up_taunt_effect, Low)
    .acmd("sound_appealhil", ssbexo_link_up_taunt_sound, Low)
    .acmd("expression_appealhil", ssbexo_link_up_taunt_expression, Low)
    .install()
    ;
}