use super::*;

//Up Special 3 ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_3_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi3"), false, -1.0);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 275, 100, 96, 0, 6.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

//Up Special 3 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_3_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 2.5, 0.5, Hash40::new("haver"), 0.0, 12.0, 0.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Special 3 Sound
unsafe extern "C" fn ssbexo_kirby_up_special_3_sound(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_special_h04"));
    }
}

//Up Special 3 Expression
unsafe extern "C" fn ssbexo_kirby_up_special_3_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhi3", ssbexo_kirby_up_special_3_acmd, Low)
    .effect_acmd("effect_specialhi3", ssbexo_kirby_up_special_3_effect, Low)
    .sound_acmd("sound_specialhi3", ssbexo_kirby_up_special_3_sound, Low)
    .expression_acmd("expression_specialhi3", ssbexo_kirby_up_special_3_expression, Low)
    .install()
    ;
}