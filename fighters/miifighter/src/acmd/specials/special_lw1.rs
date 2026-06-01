use super::*;

//Armor Crushing Thunder Kick Start ACMD
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_start_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_ACTIVE_ARMOR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_DASH_CANCEL);
    }
}

//Armor Crushing Thunder Kick Start Effect
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
    }
}

//Armor Crushing Thunder Kick Start Sound
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_start_sound(_agent: &mut L2CAgentBase) {}

//Grounded Armor Crushing Thunder Kick Start Expression
unsafe extern "C" fn ssbexo_miifighter_grounded_armor_crushing_thunder_kick_start_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Armor Crushing Thunder Kick Start Expression
unsafe extern "C" fn ssbexo_miifighter_aerial_armor_crushing_thunder_kick_start_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_acmd, Low)
    .game_acmd("game_specialairlw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_acmd, Low)
    .effect_acmd("effect_speciallw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_effect, Low)
    .effect_acmd("effect_specialairlw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_effect, Low)
    .sound_acmd("sound_speciallw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_sound, Low)
    .sound_acmd("sound_specialairlw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_sound, Low)
    .expression_acmd("expression_speciallw1", ssbexo_miifighter_grounded_armor_crushing_thunder_kick_start_expression, Low)
    .expression_acmd("expression_specialairlw1", ssbexo_miifighter_aerial_armor_crushing_thunder_kick_start_expression, Low)
    .install()
    ;
}