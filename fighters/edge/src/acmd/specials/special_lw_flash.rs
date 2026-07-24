use super::*;

//Retaliation Stance Scintilla ACMD
unsafe extern "C" fn ssbexo_edge_retaliation_stance_flash_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, *FIGHTER_EDGE_SHIELD_GROUP_KIND_SPECIAL_LW_FLASH);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.04);
        sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.08);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("rot"), 3.1, 0, 50, 50, 0, 10.0, 0.0, 0.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("rot"), 3.1, 0, 50, 50, 0, 10.0, 0.0, 0.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("rot"), 3.1, 0, 50, 50, 0, 8.5, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 12.0, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FLASH, false, -1);
    }
}

//Retaliation Stance Scintilla Effect
unsafe extern "C" fn ssbexo_edge_retaliation_stance_flash_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4, 0, -0.7, Hash40::new("swordl2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

//Retaliation Stance Scintilla Sound
unsafe extern "C" fn ssbexo_edge_retaliation_stance_flash_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let rand = sv_math::randf(hash40("fighter"), 1.0);
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_l02_01"));
        PLAY_SE(agent, Hash40::new("se_edge_attackair_b02"));
    }
    if rand > 0.9 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_s01"));
        }
    }
    else if rand > 0.8 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_s03"));
        }
    }
    else if rand > 0.7 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_rand05"));
        }
    }
    else if rand > 0.6 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_rand03"));
        }
    }
    else if rand > 0.5 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_rand04"));
        }
    }
    else if rand > 0.4 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_l02"));
        }
    }
    else if rand > 0.2 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_l03"));
        }
    }
}

//Retaliation Stance Scintilla Expression
unsafe extern "C" fn ssbexo_edge_retaliation_stance_flash_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_slash"), 4);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl_s"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwflash", ssbexo_edge_retaliation_stance_flash_acmd, Low)
    .acmd("game_specialairlwflash", ssbexo_edge_retaliation_stance_flash_acmd, Low)
    .acmd("effect_speciallwflash", ssbexo_edge_retaliation_stance_flash_effect, Low)
    .acmd("effect_specialairlwflash", ssbexo_edge_retaliation_stance_flash_effect, Low)
    .acmd("sound_speciallwflash", ssbexo_edge_retaliation_stance_flash_sound, Low)
    .acmd("sound_specialairlwflash", ssbexo_edge_retaliation_stance_flash_sound, Low)
    .acmd("expression_speciallwflash", ssbexo_edge_retaliation_stance_flash_expression, Low)
    .acmd("expression_specialairlwflash", ssbexo_edge_retaliation_stance_flash_expression, Low)
    .install()
    ;
}