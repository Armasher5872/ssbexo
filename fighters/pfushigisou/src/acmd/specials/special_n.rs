use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_pfushigisou_neutral_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(boma);
    frame(lua_state, 20.0);
    if !IS_EXIST_ARTICLE(agent, FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SLUDGE)  {
        if is_excute(agent) {
            ArticleModule::generate_article(boma, FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SLUDGE, false, -1);
        }
    }
    if is_excute(agent) {
        SET_SPEED_EX(agent, speed*lr, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

//Grounded Neutral Special Effect
unsafe extern "C" fn ssbexo_pfushigisou_grounded_neutral_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("pfushigisou_tanemg"), Hash40::new("flower"), 5.7, 0, 0, 0, 0, 0, 1.3, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, -4, 2, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pfushigisou_leaf"), Hash40::new("flower"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 360, 0, true);
    }
}

//Aerial Neutral Special Effect
unsafe extern "C" fn ssbexo_pfushigisou_aerial_neutral_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("pfushigisou_tanemg"), Hash40::new("flower"), 5.7, 0, 0, 0, 0, 0, 1.3, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pfushigisou_leaf"), Hash40::new("flower"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 360, 0, true);
    }
}

//Neutral Special Sound
unsafe extern "C" fn ssbexo_pfushigisou_neutral_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_pfushigisou_special_n01"));
        PLAY_SE(agent, Hash40::new("se_pfushigisou_special_n03"));
    }
}

//Grounded Neutral Special Expression
unsafe extern "C" fn ssbexo_pfushigisou_grounded_neutral_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

//Aerial Neutral Special Expression
unsafe extern "C" fn ssbexo_pfushigisou_aerial_neutral_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("pfushigisou")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnstart", ssbexo_pfushigisou_neutral_special_acmd, Low)
    .acmd("game_specialairnstart", ssbexo_pfushigisou_neutral_special_acmd, Low)
    .acmd("effect_specialnstart", ssbexo_pfushigisou_grounded_neutral_special_effect, Low)
    .acmd("effect_specialairnstart", ssbexo_pfushigisou_aerial_neutral_special_effect, Low)
    .acmd("sound_specialnstart", ssbexo_pfushigisou_neutral_special_sound, Low)
    .acmd("sound_specialairnstart", ssbexo_pfushigisou_neutral_special_sound, Low)
    .acmd("expression_specialnstart", ssbexo_pfushigisou_grounded_neutral_special_expression, Low)
    .acmd("expression_specialairnstart", ssbexo_pfushigisou_aerial_neutral_special_expression, Low)
    .install()
    ;
}