use super::*;

//Grounded Down Special ACMD
unsafe extern "C" fn ssbexo_mariod_grounded_down_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    for _ in 0..6 {
        if is_excute(agent) {
            ATTACK(agent, 4, 0, Hash40::new("top"), 1.6, 90, 100, 80, 0, 4.0, 0.0, 3.2, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 3, 0, Hash40::new("top"), 1.6, 367, 100, 15, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.6, 367, 100, 15, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("sys_hit_elec"));
            AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("sys_hit_elec"));
            AttackModule::set_optional_hit_effect(boma, 2, Hash40::new("sys_hit_elec"));
            AttackModule::set_optional_hit_effect(boma, 3, Hash40::new("sys_hit_elec"));
            AttackModule::set_optional_hit_effect(boma, 4, Hash40::new("sys_hit_elec"));
            AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_elec_m"));
            AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_elec_m"));
            AttackModule::set_optional_hit_sound(boma, 2, Hash40::new("se_common_elec_m"));
            AttackModule::set_optional_hit_sound(boma, 3, Hash40::new("se_common_elec_m"));
            AttackModule::set_optional_hit_sound(boma, 4, Hash40::new("se_common_elec_m"));
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 3.0);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 45, 154, 0, 80, 6.5, 0.0, 12.0, 6.0, Some(0.0), Some(12.0), Some(-6.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 45, 154, 0, 80, 6.0, 0.0, 4.0, 2.5, Some(0.0), Some(4.0), Some(-2.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("sys_hit_elec"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("sys_hit_elec"));
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_elec_ll"));
        AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_elec_ll"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Aerial Down Special ACMD
unsafe extern "C" fn ssbexo_mariod_aerial_down_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    for _ in 0..6 {
        if is_excute(agent) {
            ATTACK(agent, 4, 0, Hash40::new("top"), 1.6, 90, 100, 80, 0, 4.0, 0.0, 3.2, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 3, 0, Hash40::new("top"), 1.6, 367, 100, 15, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.6, 367, 100, 15, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("sys_hit_elec"));
            AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("sys_hit_elec"));
            AttackModule::set_optional_hit_effect(boma, 2, Hash40::new("sys_hit_elec"));
            AttackModule::set_optional_hit_effect(boma, 3, Hash40::new("sys_hit_elec"));
            AttackModule::set_optional_hit_effect(boma, 4, Hash40::new("sys_hit_elec"));
            AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_elec_m"));
            AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_elec_m"));
            AttackModule::set_optional_hit_sound(boma, 2, Hash40::new("se_common_elec_m"));
            AttackModule::set_optional_hit_sound(boma, 3, Hash40::new("se_common_elec_m"));
            AttackModule::set_optional_hit_sound(boma, 4, Hash40::new("se_common_elec_m"));
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 3.0);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 45, 154, 0, 80, 6.5, 0.0, 12.0, 6.0, Some(0.0), Some(12.0), Some(-6.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 45, 154, 0, 80, 6.0, 0.0, 4.0, 2.5, Some(0.0), Some(4.0), Some(-2.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("sys_hit_elec"));
        AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("sys_hit_elec"));
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_elec_ll"));
        AttackModule::set_optional_hit_sound(boma, 1, Hash40::new("se_common_elec_ll"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Grounded Down Special Effect
unsafe extern "C" fn ssbexo_mariod_grounded_down_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    wait(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_r"), Hash40::new("mariod_cyclone_l"), Hash40::new("top"), 0, 1, 0, 0, 180, 0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handr"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handr"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handr"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handr"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_l"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 3, 9.5, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_impact"), Hash40::new("mariod_cyclone_impact"), Hash40::new("top"), 3, 9.5, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_cyclone_impact"), -1);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

//Aerial Down Special Effect
unsafe extern "C" fn ssbexo_mariod_aerial_down_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    wait(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_r"), Hash40::new("mariod_cyclone_l"), Hash40::new("top"), 0, -1.5, 0, 0, 180, 0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handr"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handr"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handr"), 2.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_r"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_cyclone_l"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 3, 9.5, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_cyclone_impact"), Hash40::new("mariod_cyclone_impact"), Hash40::new("top"), 3, 9.5, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_cyclone_impact"), -1);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_mariod_down_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_l01"));
        PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
        PLAY_SE(agent, Hash40::new("se_mariod_special_l02"));
        PLAY_SE(agent, Hash40::new("vc_mariod_009"));
    }
}

pub fn install() {
    Agent::new("mariod")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallw", ssbexo_mariod_grounded_down_special_acmd, Low)
    .acmd("game_specialairlw", ssbexo_mariod_aerial_down_special_acmd, Low)
    .acmd("effect_speciallw", ssbexo_mariod_grounded_down_special_effect, Low)
    .acmd("effect_specialairlw", ssbexo_mariod_aerial_down_special_effect, Low)
    .acmd("sound_speciallw", ssbexo_mariod_down_special_sound, Low)
    .acmd("sound_specialairlw", ssbexo_mariod_down_special_sound, Low)
    .install()
    ;
}