use super::*;

//Up Special 1 ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_1_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi"), false, -1.0);
    }
}

//Up Special 1 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_1_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 12, 0, 0, 0, -149, 1, false, *EF_FLIP_YZ);
    }
}

//Up Special 1 Sound
unsafe extern "C" fn ssbexo_kirby_up_special_1_sound(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_006"));
        PLAY_SE(agent, Hash40::new("se_kirby_special_h01"));
    }
}

//Up Special 1 Expression
unsafe extern "C" fn ssbexo_kirby_up_special_1_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhi1", ssbexo_kirby_up_special_1_acmd, Low)
    .game_acmd("game_specialairhi1", ssbexo_kirby_up_special_1_acmd, Low)
    .effect_acmd("effect_specialhi1", ssbexo_kirby_up_special_1_effect, Low)
    .effect_acmd("effect_specialairhi1", ssbexo_kirby_up_special_1_effect, Low)
    .sound_acmd("sound_specialhi1", ssbexo_kirby_up_special_1_sound, Low)
    .sound_acmd("sound_specialairhi1", ssbexo_kirby_up_special_1_sound, Low)
    .expression_acmd("expression_specialhi1", ssbexo_kirby_up_special_1_expression, Low)
    .expression_acmd("expression_specialairhi1", ssbexo_kirby_up_special_1_expression, Low)
    .install()
    ;
}