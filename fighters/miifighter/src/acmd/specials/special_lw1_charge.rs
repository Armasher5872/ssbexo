use super::*;

//Armor Crushing Thunder Kick Charge ACMD
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_charge_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_DASH_CANCEL);
    }
}

//Armor Crushing Thunder Kick Charge Effect
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..10 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("miifighter_sidekick_hold"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
            EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
            EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
            EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_sign"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_sign"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_flash"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
    }
}

//Armor Crushing Thunder Kick Charge Sound
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_charge_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_special_c3_n01"));
    }
}

//Grounded Armor Crushing Thunder Kick Charge Expression
unsafe extern "C" fn ssbexo_miifighter_grounded_armor_crushing_thunder_kick_charge_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Armor Crushing Thunder Kick Charge Expression
unsafe extern "C" fn ssbexo_miifighter_aerial_armor_crushing_thunder_kick_charge_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_acmd, Low)
    .game_acmd("game_specialairlw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_acmd, Low)
    .effect_acmd("effect_speciallw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_effect, Low)
    .effect_acmd("effect_specialairlw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_effect, Low)
    .sound_acmd("sound_speciallw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_sound, Low)
    .sound_acmd("sound_specialairlw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_sound, Low)
    .expression_acmd("expression_speciallw1charge", ssbexo_miifighter_grounded_armor_crushing_thunder_kick_charge_expression, Low)
    .expression_acmd("expression_specialairlw1charge", ssbexo_miifighter_aerial_armor_crushing_thunder_kick_charge_expression, Low)
    .install()
    ;
}