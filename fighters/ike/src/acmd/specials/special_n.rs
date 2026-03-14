use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_ike_neutral_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword"), 6.0, 361, 120, 0, 60, 4.0, 0.0, 10.2, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword"), 6.0, 361, 120, 0, 60, 4.0, 0.0, 7.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword"), 6.0, 361, 120, 0, 60, 4.0, 0.0, 5.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SLASH, false, -1);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Neutral Special Effect
unsafe extern "C" fn ssbexo_ike_grounded_neutral_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 18, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 16, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("ike_sword2"), false, false);
    }
}

//Aerial Neutral Special Effect
unsafe extern "C" fn ssbexo_ike_aerial_neutral_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("ike_sword2"), false, false);
    }
}

//Grounded Neutral Special Sound
unsafe extern "C" fn ssbexo_ike_grounded_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, pull as i32, 1.0, 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        let flame = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_special_n07"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swing as i32, 1.7, 0);
        SoundModule::set_se_vol(agent.module_accessor, flame as i32, 2.0, 0);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ike_special_n01"));
        PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ike_swordgroundhit"));
    }
}

//Aerial Neutral Special Sound
unsafe extern "C" fn ssbexo_ike_aerial_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, pull as i32, 1.0, 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        let flame = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_special_n07"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swing as i32, 1.7, 0);
        SoundModule::set_se_vol(agent.module_accessor, flame as i32, 2.0, 0);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ike_special_n01"));
        PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
    }
}

//Grounded Neutral Special Expression
unsafe extern "C" fn ssbexo_ike_grounded_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        agent.clear_lua_stack();
        lua_args!(agent, 0, 1, 80, 300, 0.8, 0, 12, 24, 24, 40);
        sv_animcmd::AREA_WIND_2ND_arg10(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

//Aerial Neutral Special Expression
unsafe extern "C" fn ssbexo_ike_aerial_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 1, 80, 300, 0.8, 0, 12, 24, 24, 40);
        sv_animcmd::AREA_WIND_2ND_arg10(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

pub fn install() {
    Agent::new("ike")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnstart", ssbexo_ike_neutral_special_acmd, Low)
    .game_acmd("game_specialairnstart", ssbexo_ike_neutral_special_acmd, Low)
    .effect_acmd("effect_specialnstart", ssbexo_ike_grounded_neutral_special_effect, Low)
    .effect_acmd("effect_specialairnstart", ssbexo_ike_aerial_neutral_special_effect, Low)
    .sound_acmd("sound_specialnstart", ssbexo_ike_grounded_neutral_special_sound, Low)
    .sound_acmd("sound_specialairnstart", ssbexo_ike_aerial_neutral_special_sound, Low)
    .expression_acmd("expression_specialnstart", ssbexo_ike_grounded_neutral_special_expression, Low)
    .expression_acmd("expression_specialairnstart", ssbexo_ike_aerial_neutral_special_expression, Low)
    .install()
    ;
}