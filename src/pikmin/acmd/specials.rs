use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_pikmin_neutral_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        let mot_rate = WorkModule::get_float(agent.module_accessor, *FIGHTER_PIKMIN_STATUS_PULL_OUT_WORK_FLOAT_MOT_RATE);
        MotionModule::set_rate(agent.module_accessor, mot_rate);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
    }
}

//Neutral Special Effect
unsafe extern "C" fn ssbexo_pikmin_neutral_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("pikmin_hikkonuki"), Hash40::new("top"), 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Neutral Special Sound
unsafe extern "C" fn ssbexo_pikmin_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_special_n01"));
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_special_n03"));
    }
}

//Neutral Special Expression
unsafe extern "C" fn ssbexo_pikmin_neutral_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Grounded Neutral Special Failure Sound
unsafe extern "C" fn ssbexo_pikmin_neutral_special_failure_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_special_n02"));
    }
}

//Grounded Neutral Special Failure Expression
unsafe extern "C" fn ssbexo_pikmin_neutral_special_failure_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 4);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

pub fn install() {
    Agent::new("pikmin")
    .game_acmd("game_specialnstart", ssbexo_pikmin_neutral_special_acmd)
    .effect_acmd("effect_specialnstart", ssbexo_pikmin_neutral_special_effect)
    .sound_acmd("sound_specialnstart", ssbexo_pikmin_neutral_special_sound)
    .expression_acmd("expression_specialnstart", ssbexo_pikmin_neutral_special_expression)
    .sound_acmd("sound_specialnfailure", ssbexo_pikmin_neutral_special_failure_sound)
    .expression_acmd("expression_specialnfailure", ssbexo_pikmin_neutral_special_failure_expression)
    .install()
    ;
}