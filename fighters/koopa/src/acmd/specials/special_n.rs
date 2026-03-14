use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_koopa_neutral_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, false, -1);
    }
}

//Grounded Neutral Special Effect
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Neutral Special Effect
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Grounded Neutral Special Sound
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_koopa_step_left_m"));
    }
}

//Aerial Neutral Special Sound
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
}

//Grounded Neutral Special Expression
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Neutral Special Expression
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn", ssbexo_koopa_neutral_special_acmd, Low)
    .game_acmd("game_specialairn", ssbexo_koopa_neutral_special_acmd, Low)
    .effect_acmd("effect_specialn", ssbexo_koopa_grounded_neutral_special_effect, Low)
    .effect_acmd("effect_specialairn", ssbexo_koopa_aerial_neutral_special_effect, Low)
    .sound_acmd("sound_specialn", ssbexo_koopa_grounded_neutral_special_sound, Low)
    .sound_acmd("sound_specialairn", ssbexo_koopa_aerial_neutral_special_sound, Low)
    .expression_acmd("expression_specialn", ssbexo_koopa_grounded_neutral_special_expression, Low)
    .expression_acmd("expression_specialairn", ssbexo_koopa_aerial_neutral_special_expression, Low)
    .install()
    ;
}