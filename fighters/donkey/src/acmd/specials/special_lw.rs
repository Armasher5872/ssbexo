use super::*;

//Down Special ACMD
unsafe extern "C" fn ssbexo_donkey_down_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    FT_MOTION_RATE(agent, 33.0/25.0);
    frame(lua_state, 25.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_UNLINK);
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_donkey_down_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_item_get"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_donkey_down_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_item_get"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_item_throw"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_donkey_rnd_attack"));
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_donkey_down_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 2);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwstart", ssbexo_donkey_down_special_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_donkey_down_special_acmd, Low)
    .effect_acmd("effect_speciallwstart", ssbexo_donkey_down_special_effect, Low)
    .effect_acmd("effect_specialairlw", ssbexo_donkey_down_special_effect, Low)
    .sound_acmd("sound_speciallwstart", ssbexo_donkey_down_special_sound, Low)
    .sound_acmd("sound_specialairlw", ssbexo_donkey_down_special_sound, Low)
    .expression_acmd("expression_speciallwstart", ssbexo_donkey_down_special_expression, Low)
    .expression_acmd("expression_specialairlw", ssbexo_donkey_down_special_expression, Low)
    .install()
    ;
}