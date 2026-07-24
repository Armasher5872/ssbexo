use super::*;

//Grounded Retaliation Stance Parry Hit ACMD
unsafe extern "C" fn ssbexo_edge_grounded_retaliation_stance_parry_hit_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_XLU);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.1, 65, 22, 0, 45, 10.0, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(12.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.1, 65, 22, 0, 45, 10.0, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(23.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.1, 65, 22, 0, 45, 10.0, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(31.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

//Aerial Retaliation Stance Parry Hit ACMD
unsafe extern "C" fn ssbexo_edge_aerial_retaliation_stance_parry_hit_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_XLU);
        AttackModule::clear_all(boma);
        sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, WorkModule::get_param_float(boma, hash40("common"), hash40("air_speed_y_stable"))*0.15);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.1, 65, 22, 0, 45, 10.0, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(12.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.1, 65, 22, 0, 45, 10.0, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(23.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.1, 65, 22, 0, 45, 10.0, 0.0, 9.5, 6.0, Some(0.0), Some(9.5), Some(31.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

//Retaliation Stance Parry Hit Effect
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_hit_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4, 0, -0.7, Hash40::new("swordl2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

//Retaliation Stance Parry Hit Sound
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_hit_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_l02_01"));
        PLAY_SE(agent, Hash40::new("se_edge_smash_s01"));
    }
}

//Retaliation Stance Parry Hit Expression
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_hit_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_slash"), 4);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl_s"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwparryhit", ssbexo_edge_grounded_retaliation_stance_parry_hit_acmd, Low)
    .acmd("game_specialairlwparryhit", ssbexo_edge_aerial_retaliation_stance_parry_hit_acmd, Low)
    .acmd("effect_speciallwparryhit", ssbexo_edge_retaliation_stance_parry_hit_effect, Low)
    .acmd("effect_specialairlwparryhit", ssbexo_edge_retaliation_stance_parry_hit_effect, Low)
    .acmd("sound_speciallwparryhit", ssbexo_edge_retaliation_stance_parry_hit_sound, Low)
    .acmd("sound_specialairlwparryhit", ssbexo_edge_retaliation_stance_parry_hit_sound, Low)
    .acmd("expression_speciallwparryhit", ssbexo_edge_retaliation_stance_parry_hit_expression, Low)
    .acmd("expression_specialairlwparryhit", ssbexo_edge_retaliation_stance_parry_hit_expression, Low)
    .install()
    ;
}