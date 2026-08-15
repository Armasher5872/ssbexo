use super::*;

//Retaliation Stance Parry ACMD
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_EDGE_SHIELD_GROUP_KIND_SPECIAL_LW);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.1, 361, 0, 0, 0, 2.7, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(12.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.1, 361, 0, 0, 0, 2.7, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.1, 361, 0, 0, 0, 2.7, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(31.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, *FIGHTER_EDGE_SHIELD_GROUP_KIND_SPECIAL_LW);
    }
}

//Retaliation Stance Parry Effect
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4, 0, -0.7, Hash40::new("swordl2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("rot"), 0, 4, 2, 0, -3.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
    }
}

//Retaliation Stance Parry Sound
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_sound(agent: &mut L2CAgentBase) {
    let rand = sv_math::randf(hash40("fighter"), 1.0);
    let boma = agent.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_l02"));
            }
        }
        else if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_s01"));
            }
        }
        else if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_s02"));
            }
        }
        else if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_s03"));
            }
        }
        else if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_rand01"));
            }
        }
        else if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack03"));
            }
        }
        else if rand > 0.5 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_attack04"));
            }
        }
    }
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

//Retaliation Stance Parry Expression
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_slash"), 4);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl_s"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwparry", ssbexo_edge_retaliation_stance_parry_acmd, Low)
    .acmd("game_specialairlwparry", ssbexo_edge_retaliation_stance_parry_acmd, Low)
    .acmd("effect_speciallwparry", ssbexo_edge_retaliation_stance_parry_effect, Low)
    .acmd("effect_specialairlwparry", ssbexo_edge_retaliation_stance_parry_effect, Low)
    .acmd("sound_speciallwparry", ssbexo_edge_retaliation_stance_parry_sound, Low)
    .acmd("sound_specialairlwparry", ssbexo_edge_retaliation_stance_parry_sound, Low)
    .acmd("expression_speciallwparry", ssbexo_edge_retaliation_stance_parry_expression, Low)
    .acmd("expression_specialairlwparry", ssbexo_edge_retaliation_stance_parry_expression, Low)
    .install()
    ;
}