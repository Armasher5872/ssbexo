use super::*;

//Up Special Launch ACMD
unsafe extern "C" fn ssbexo_link_special_hi_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("special_hi"), false, -1.0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_JUMP);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 6.0, 0.0, 4.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 60, 145, 0, 50, 6.0, 0.0, 4.0, 5.0, None, None, None, 1.0, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Special Launch Effect
unsafe extern "C" fn ssbexo_link_special_hi_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 13, 0, -90, 0, 0, 0.625, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(agent, 0.25, 1.00, 0.5);
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

//Up Special Launch Sound
unsafe extern "C" fn ssbexo_link_special_hi_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_jump02"));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_link_ottotto"));
    }
}

//Up Special Launch Expression
unsafe extern "C" fn ssbexo_link_special_hi_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhi", ssbexo_link_special_hi_acmd, Low)
    .game_acmd("game_specialairhi", ssbexo_link_special_hi_acmd, Low)
    .effect_acmd("effect_specialhi", ssbexo_link_special_hi_effect, Low)
    .effect_acmd("effect_specialairhi", ssbexo_link_special_hi_effect, Low)
    .sound_acmd("sound_specialhi", ssbexo_link_special_hi_sound, Low)
    .sound_acmd("sound_specialairhi", ssbexo_link_special_hi_sound, Low)
    .expression_acmd("expression_specialhi", ssbexo_link_special_hi_expression, Low)
    .expression_acmd("expression_specialairhi", ssbexo_link_special_hi_expression, Low)
    .install()
    ;
}