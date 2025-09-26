use super::*;

//Jab 1 ACMD
unsafe extern "C" fn ssbexo_ganon_jab_1_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 80, 25, 0, 35, 5.0, 0.0, 7.0, 14.0, Some(0.0), Some(7.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 8.0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

//Jab 1 Effect
unsafe extern "C" fn ssbexo_ganon_jab_1_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 8.0, 8.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        if has_totk_skin(agent.module_accessor) {
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 1.5);
    }
}

//Jab 1 Sound
unsafe extern "C" fn ssbexo_ganon_jab_1_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_swing_s"));
    }
}

//Jab 1 Expression
unsafe extern "C" fn ssbexo_ganon_jab_1_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Jab 2 ACMD
unsafe extern "C" fn ssbexo_ganon_jab_2_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 361, 74, 0, 55, 5.0, 0.0, 10.0, 17.0, Some(0.0), Some(10.0), Some(32.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Jab 2 Effect
unsafe extern "C" fn ssbexo_ganon_jab_2_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1000, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 13.33, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 3.846, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.349, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.079, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_entry_aura"), false, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ganon_sword1"), Hash40::new("tex_ganon_sword2"), 7, Hash40::new("haver"), 0, 2.3, 0, Hash40::new("haver"), 0, 18.5, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);
        AFTER_IMAGE_OFF(agent, 4);
    }
}

//Jab 2 Sound
unsafe extern "C" fn ssbexo_ganon_jab_2_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        let rand = sv_math::rand(hash40("fighter"), 100);
        if (67..=100).contains(&rand) {
            PLAY_SE(agent, Hash40::new("vc_ganon_attack05"));
        } 
        else if (34..=66).contains(&rand) {
            PLAY_SE(agent, Hash40::new("vc_ganon_attack03"));
        } 
        else {
            PLAY_SE(agent, Hash40::new("vc_ganon_attack04"));
        }
        PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
}

//Jab 2 Expression
unsafe extern "C" fn ssbexo_ganon_jab_2_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_ganon_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 70, 85, 0, 50, 7.0, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 80, 60, 0, 45, 4.0, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.3);
    }
    wait(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_ganon_dash_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 9, 6.5, 0, 0, 0, 1.6, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, 2, 0, 180, 0, 1.5, true);
        if has_totk_skin(agent.module_accessor) {
            LAST_PARTICLE_SET_COLOR(agent, 1, 0, 0);
        }
        else {
            LAST_PARTICLE_SET_COLOR(agent, 0.8, 0.6, 3);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, 2, 0, 180, 0, 1.5, true);
        if has_totk_skin(agent.module_accessor) {
            LAST_PARTICLE_SET_COLOR(agent, 1, 0, 0);
        }
        else {
            LAST_PARTICLE_SET_COLOR(agent, 0.8, 0.6, 3);
        }
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2.8, 0, 7.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack11", ssbexo_ganon_jab_1_acmd, Low)
    .effect_acmd("effect_attack11", ssbexo_ganon_jab_1_effect, Low)
    .sound_acmd("sound_attack11", ssbexo_ganon_jab_1_sound, Low)
    .expression_acmd("expression_attack11", ssbexo_ganon_jab_1_expression, Low)
    .game_acmd("game_attack12", ssbexo_ganon_jab_2_acmd, Low)
    .effect_acmd("effect_attack12", ssbexo_ganon_jab_2_effect, Low)
    .sound_acmd("sound_attack12", ssbexo_ganon_jab_2_sound, Low)
    .expression_acmd("expression_attack12", ssbexo_ganon_jab_2_expression, Low)
    .game_acmd("game_attackdash", ssbexo_ganon_dash_attack_acmd, Low)
    .effect_acmd("effect_attackdash", ssbexo_ganon_dash_attack_effect, Low)
    .install()
    ;
}