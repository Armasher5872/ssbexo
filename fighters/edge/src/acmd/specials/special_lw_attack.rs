use super::*;

//Retaliation Stance Attack ACMD
unsafe extern "C" fn ssbexo_edge_retaliation_stance_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let charge = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE);
    let damage = if charge >= 2 {16.5} else if charge == 1 {11.0} else {5.5};
    let hitstop = if charge >= 2 {2.0} else if charge == 1 {1.5} else {1.0};
    let sound_level = if charge >= 1 {*ATTACK_SOUND_LEVEL_L} else {*ATTACK_SOUND_LEVEL_M};
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, 23, 63, 0, 85, 2.7, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(12.5), hitstop, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), sound_level, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), damage, 23, 63, 0, 85, 2.7, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(23.0), hitstop, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), sound_level, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), damage, 23, 63, 0, 85, 2.7, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(31.0), hitstop, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), sound_level, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Retaliation Stance Attack Effect
unsafe extern "C" fn ssbexo_edge_retaliation_stance_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let charge = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE);
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if charge >= 2 {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("edge_retaliation_sword_fire_s"), Hash40::new("swordl1"), 5, -4, 0, 0, 0, 0, 0.7, true);
                EFFECT_FOLLOW(agent, Hash40::new("edge_retaliation_sword_fire_s"), Hash40::new("swordl1"), 10, -4, 0, 0, 0, 0, 0.7, true);
                EFFECT_FOLLOW(agent, Hash40::new("edge_retaliation_sword_fire_s"), Hash40::new("swordl1"), 15, -4, 0, 0, 0, 0, 0.7, true);
                EFFECT_FOLLOW(agent, Hash40::new("edge_retaliation_sword_fire_s"), Hash40::new("swordl1"), 20, -4, 0, 0, 0, 0, 0.7, true);
                EFFECT_FOLLOW(agent, Hash40::new("edge_retaliation_sword_fire_s"), Hash40::new("swordl1"), 25, -4, 0, 0, 0, 0, 0.7, true);
                EFFECT_FOLLOW(agent, Hash40::new("edge_retaliation_sword_fire_s"), Hash40::new("swordl1"), 30, -4, 0, 0, 0, 0, 0.7, true);
            }
        }
        if charge >= 1 {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword5"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4, 0, -0.7, Hash40::new("swordl2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4, 0, -0.7, Hash40::new("swordl2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
    }
}

//Grounded Retaliation Stance Attack Sound
unsafe extern "C" fn ssbexo_edge_grounded_retaliation_stance_attack_sound(agent: &mut L2CAgentBase) {
    let rand = sv_math::randf(hash40("fighter"), 1.0);
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let charge = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_edge_step_left_s"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_edge_step_left_m"));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_attackhard_s01"));
    }
    frame(lua_state, 16.0);
    if charge > 0 {
        if rand > 0.9 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_rand04"));
            }
        }
        else if rand > 0.8 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_rand05"));
            }
        }
        else if rand > 0.7 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_l03"));
            }
        }
        else if rand > 0.6 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_s03"));
            }
        }
        else if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack06"));
            }
        }
        else if rand > 0.4 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack08"));
            }
        }
        else if rand > 0.3 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack07"));
            }
        }
        else if rand > 0.2 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack09"));
            }
        }
        else {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_n05"));
            }
        }
    }
    else {
        if rand > 0.9 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack01"));
            }
        }
        else if rand > 0.8 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack02"));
            }
        }
        else if rand > 0.7 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack03"));
            }
        }
        else if rand > 0.6 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack04"));
            }
        }
        else if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_rand04"));
            }
        }
        else if rand > 0.4 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_n03"));
            }
        }
        else if rand > 0.2 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_n02"));
            }
        }
    }
    frame(lua_state, 20.0);
    if charge >= 2 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_edge_final01_02"));
        }
    }
    else {
        if rand > 0.857 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_07"));
            }
        }
        else if rand > 0.714 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_06"));
            }
        }
        else if rand > 0.571 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_05"));
            }
        }
        else if rand > 0.428 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_04"));
            }
        }
        else if rand > 0.285 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_03"));
            }
        }
        else if rand > 0.142 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_02"));
            }
        }
        else {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_01"));
            }
        }
    }
}

//Aerial Retaliation Stance Attack Sound
unsafe extern "C" fn ssbexo_edge_aerial_retaliation_stance_attack_sound(agent: &mut L2CAgentBase) {
    let rand = sv_math::randf(hash40("fighter"), 1.0);
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let charge = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE);
    frame(lua_state, 16.0);
    if charge > 0 {
        if rand > 0.9 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_rand04"));
            }
        }
        else if rand > 0.8 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_rand05"));
            }
        }
        else if rand > 0.7 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_l03"));
            }
        }
        else if rand > 0.6 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_s03"));
            }
        }
        else if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack06"));
            }
        }
        else if rand > 0.4 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack08"));
            }
        }
        else if rand > 0.3 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack07"));
            }
        }
        else if rand > 0.2 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack09"));
            }
        }
        else {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_n05"));
            }
        }
    }
    else {
        if rand > 0.9 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack01"));
            }
        }
        else if rand > 0.8 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack02"));
            }
        }
        else if rand > 0.7 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack03"));
            }
        }
        else if rand > 0.6 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack04"));
            }
        }
        else if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_rand04"));
            }
        }
        else if rand > 0.4 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_n03"));
            }
        }
        else if rand > 0.2 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_n02"));
            }
        }
    }
    frame(lua_state, 20.0);
    if charge >= 2 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_edge_final01_02"));
        }
    }
    else {
        if rand > 0.857 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_07"));
            }
        }
        else if rand > 0.714 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_06"));
            }
        }
        else if rand > 0.571 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_05"));
            }
        }
        else if rand > 0.428 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_04"));
            }
        }
        else if rand > 0.285 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_03"));
            }
        }
        else if rand > 0.142 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_02"));
            }
        }
        else {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_edge_special_h03_01"));
            }
        }
    }
}

//Retaliation Stance Attack Expression
unsafe extern "C" fn ssbexo_edge_retaliation_stance_attack_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_slash"), 4);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl_m"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwattack", ssbexo_edge_retaliation_stance_attack_acmd, Low)
    .acmd("game_specialairlwattack", ssbexo_edge_retaliation_stance_attack_acmd, Low)
    .acmd("effect_speciallwattack", ssbexo_edge_retaliation_stance_attack_effect, Low)
    .acmd("effect_specialairlwattack", ssbexo_edge_retaliation_stance_attack_effect, Low)
    .acmd("sound_speciallwattack", ssbexo_edge_grounded_retaliation_stance_attack_sound, Low)
    .acmd("sound_specialairlwattack", ssbexo_edge_aerial_retaliation_stance_attack_sound, Low)
    .acmd("expression_speciallwattack", ssbexo_edge_retaliation_stance_attack_expression, Low)
    .acmd("expression_specialairlwattack", ssbexo_edge_retaliation_stance_attack_expression, Low)
    .install()
    ;
}