use super::*;

//Up Special 2 ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_2_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi2"), false, -1.0);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 85, 85, 0, 117, 3.5, 0.0, 3.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 85, 85, 0, 117, 3.5, 0.0, 3.5, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 85, 85, 0, 117, 3.5, 0.0, 13.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 85, 85, 0, 117, 3.5, 0.0, 13.5, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(0));
    }
}

//Up Special 2 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_2_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 3.0, 0.25, Hash40::new("haver"), 0.0, 15.0, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        EFFECT_FOLLOW(agent, Hash40::new("kirby_fcut_rise"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

//Up Special 2 Sound
unsafe extern "C" fn ssbexo_kirby_up_special_2_sound(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_007"));
        PLAY_SE(agent, Hash40::new("se_kirby_special_h02"));
    }
}

//Up Special 2 Expression
unsafe extern "C" fn ssbexo_kirby_up_special_2_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhi2", ssbexo_kirby_up_special_2_acmd, Low)
    .game_acmd("game_specialairhi2", ssbexo_kirby_up_special_2_acmd, Low)
    .effect_acmd("effect_specialhi2", ssbexo_kirby_up_special_2_effect, Low)
    .effect_acmd("effect_specialairhi2", ssbexo_kirby_up_special_2_effect, Low)
    .sound_acmd("sound_specialhi2", ssbexo_kirby_up_special_2_sound, Low)
    .sound_acmd("sound_specialairhi2", ssbexo_kirby_up_special_2_sound, Low)
    .expression_acmd("expression_specialhi2", ssbexo_kirby_up_special_2_expression, Low)
    .expression_acmd("expression_specialairhi2", ssbexo_kirby_up_special_2_expression, Low)
    .install()
    ;
}