use super::*;

//Up Special 5 ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_5_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi4"), false, -1.0);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTERSHOT, false, -1);
    }
}

//Up Special 5 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_5_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 0);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("kirby_star"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Special 5 Sound
unsafe extern "C" fn ssbexo_kirby_up_special_5_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_008"));
        PLAY_SE(agent, Hash40::new("se_kirby_special_h03"));
    }
}

//Up Special 5 Expression
unsafe extern "C" fn ssbexo_kirby_up_special_5_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 7);
    }
}

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhi5", ssbexo_kirby_up_special_5_acmd, Low)
    .effect_acmd("effect_specialhi5", ssbexo_kirby_up_special_5_effect, Low)
    .sound_acmd("sound_specialhi5", ssbexo_kirby_up_special_5_sound, Low)
    .expression_acmd("expression_specialhi5", ssbexo_kirby_up_special_5_expression, Low)
    .install()
    ;
}