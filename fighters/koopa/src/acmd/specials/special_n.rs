use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_koopa_neutral_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, false, -1);
    }
}

//Grounded Neutral Special Effect
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Neutral Special Effect
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Grounded Neutral Special Sound
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_koopa_step_left_m"));
    }
}

//Aerial Neutral Special Sound
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
}

//Grounded Neutral Special Expression
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Neutral Special Expression
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialn", ssbexo_koopa_neutral_special_acmd, Low)
    .acmd("game_specialairn", ssbexo_koopa_neutral_special_acmd, Low)
    .acmd("effect_specialn", ssbexo_koopa_grounded_neutral_special_effect, Low)
    .acmd("effect_specialairn", ssbexo_koopa_aerial_neutral_special_effect, Low)
    .acmd("sound_specialn", ssbexo_koopa_grounded_neutral_special_sound, Low)
    .acmd("sound_specialairn", ssbexo_koopa_aerial_neutral_special_sound, Low)
    .acmd("expression_specialn", ssbexo_koopa_grounded_neutral_special_expression, Low)
    .acmd("expression_specialairn", ssbexo_koopa_aerial_neutral_special_expression, Low)
    .install()
    ;
}