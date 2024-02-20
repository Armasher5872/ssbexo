use super::*;

//Grounded Side Special ACMD
unsafe extern "C" fn ssbexo_luigi_grounded_side_special_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE) {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 30, 0, 85, 5.5, 0.0, 10.0, -5.5, None, None, None, 1.25, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 30, 0, 85, 5.5, 0.0, 10.0, 5.5, None, None, None, 1.25, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 60, 0, 25, 6.0, 0.0, 5.5, 0.0, None, None, None, 1.25, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 180, 100, 50, 0, 19.5, 0.0, 8.5, 0.0, None, None, None, 1.25, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 9, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 9, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 9, false);
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        macros::FT_MOTION_RATE(agent, 0.6);
        frame(agent.lua_state_agent, 44.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 90, 0, 85, 7.5, 0.0, 11.0, -9.5, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 90, 0, 85, 7.5, 0.0, 11.0, 9.5, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 90, 90, 0, 85, 6.5, 0.0, 4.0, 0.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 30, 0, 85, 5.5, 0.0, 10.0, -5.5, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 30, 0, 85, 5.5, 0.0, 10.0, 5.5, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 60, 0, 25, 6.0, 0.0, 5.5, 0.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 180, 100, 50, 0, 19.5, 0.0, 8.5, 0.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 9, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 9, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 9, false);
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        macros::FT_MOTION_RATE(agent, 0.6);
        frame(agent.lua_state_agent, 44.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 70, 90, 0, 85, 7.5, 0.0, 11.0, -9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 70, 90, 0, 85, 7.5, 0.0, 11.0, 9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 90, 90, 0, 85, 6.5, 0.0, 4.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Aerial Side Special ACMD
unsafe extern "C" fn ssbexo_luigi_aerial_side_special_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE) {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, 5.5, None, None, None, 1.25, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, -5.5, None, None, None, 1.25, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 2.5, 0.0, None, None, None, 1.25, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 180, 100, 45, 0, 15.5, 0.0, 8.5, 0.0, None, None, None, 1.25, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 8, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 8, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 8, false);
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        macros::FT_MOTION_RATE(agent, 0.6);
        frame(agent.lua_state_agent, 44.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 90, 0, 85, 7.5, 0.0, 11.0, -9.5, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 90, 0, 85, 7.5, 0.0, 11.0, 9.5, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 89, 90, 0, 85, 6.5, 0.0, 2.0, 0.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, 5.5, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 9.5, -5.5, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 30, 0, 80, 6.0, 0.0, 2.5, 0.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 180, 100, 45, 0, 15.5, 0.0, 8.5, 0.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 8, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 8, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 0.0, y: 8.0}, 8, false);
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        macros::FT_MOTION_RATE(agent, 0.6);
        frame(agent.lua_state_agent, 44.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 70, 90, 0, 85, 7.5, 0.0, 11.0, -9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 70, 90, 0, 85, 7.5, 0.0, 11.0, 9.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 89, 90, 0, 85, 6.5, 0.0, 2.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Grounded Side Special Effect
unsafe extern "C" fn ssbexo_luigi_grounded_side_special_effect(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE) {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_cyclone_tornado_r"), Hash40::new("luigi_cyclone_tornado_l"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, true, *EF_FLIP_NONE);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_r"), false, true);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_l"), false, true);
        }
        frame(agent.lua_state_agent, 43.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("luigi_cyclone_finish"), Hash40::new("top"), 0, 7, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(agent.lua_state_agent, 44.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_cyclone_tornado_r"), Hash40::new("luigi_cyclone_tornado_l"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, true, *EF_FLIP_NONE);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_r"), false, true);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_l"), false, true);
        }
        frame(agent.lua_state_agent, 43.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("luigi_cyclone_finish"), Hash40::new("top"), 0, 7, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 44.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//Aerial Side Special Effect
unsafe extern "C" fn ssbexo_luigi_aerial_side_special_effect(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE) {
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_cyclone_tornado_r"), Hash40::new("luigi_cyclone_tornado_l"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, true, *EF_FLIP_NONE);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_r"), false, true);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_l"), false, true);
        }
        frame(agent.lua_state_agent, 43.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("luigi_cyclone_finish"), Hash40::new("top"), 0, 7, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
    }
    else {
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_cyclone_tornado_r"), Hash40::new("luigi_cyclone_tornado_l"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, true, *EF_FLIP_NONE);
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_r"), false, true);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_cyclone_tornado_l"), false, true);
        }
        frame(agent.lua_state_agent, 43.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("luigi_cyclone_finish"), Hash40::new("top"), 0, 7, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }   
    }
}

//Grounded Side Special Sound
unsafe extern "C" fn ssbexo_luigi_grounded_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_luigi_special_l01"));
    }
    wait(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_luigi_special_l02"));
        macros::PLAY_SE(agent, Hash40::new("vc_luigi_006"));
    }
}

//Aerial Side Special Sound
unsafe extern "C" fn ssbexo_luigi_aerial_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_luigi_special_l01"));
    }
    wait(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_luigi_special_l02"));
        macros::PLAY_SE(agent, Hash40::new("vc_luigi_006"));
    }
}

//Grounded Side Special Expression
unsafe extern "C" fn ssbexo_luigi_grounded_side_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 9);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Aerial Side Special Expression
unsafe extern "C" fn ssbexo_luigi_aerial_side_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 9);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Grounded Up Special ACMD
unsafe extern "C" fn ssbexo_luigi_grounded_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 88, 88, 0, 50, 2.2, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Up Special ACMD
unsafe extern "C" fn ssbexo_luigi_aerial_up_special_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHECK_DEAD_AREA_FORCE);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHECK_DEAD_AREA_FORCE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 90, 80, 0, 40, 2.7, 0.0, 6.0, 5.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_acmd(agent: &mut L2CAgentBase) {
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_luigi_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_luigi_down_special_sound(agent: &mut L2CAgentBase) {}

//Down Special Expression
unsafe extern "C" fn ssbexo_luigi_down_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special Loop ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_loop_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.2, 180, 100, 30, 0, 6.0, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(27.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
}

//Down Special Loop Effect
unsafe extern "C" fn ssbexo_luigi_down_special_loop_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("luigi_final_vacuum"), Hash40::new("top"), 0, 6, 14, 0, 0, 0, 1, true);
    }
}

//Down Special Loop Sound
unsafe extern "C" fn ssbexo_luigi_down_special_loop_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_luigi_final02"));
    }
}

//Down Special Loop Expression
unsafe extern "C" fn ssbexo_luigi_down_special_loop_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_none"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special Catch Pull ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_catch_pull_acmd(agent: &mut L2CAgentBase) {}

//Down Special Catch Pull Effect
unsafe extern "C" fn ssbexo_luigi_down_special_catch_pull_effect(agent: &mut L2CAgentBase) {}

//Down Special Catch Pull Sound
unsafe extern "C" fn ssbexo_luigi_down_special_catch_pull_sound(agent: &mut L2CAgentBase) {}

//Down Special Catch Pull Expression
unsafe extern "C" fn ssbexo_luigi_down_special_catch_pull_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Down Special Throw ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_throw_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 65, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 18, 4);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 12.0, y: -1.0, z: 0.0});
        macros::FT_CATCH_STOP(agent, 5, 1);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

//Down Special Throw Effect
unsafe extern "C" fn ssbexo_luigi_down_special_throw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -3, 3, 0, 20, -90, 1.7, true, *EF_FLIP_YZ, 0.4);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind"), false, false);
    }
}

//Down Special Throw Sound
unsafe extern "C" fn ssbexo_luigi_down_special_throw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_luigi_rnd_attack"));
    }
}

//Down Special Throw Expression
unsafe extern "C" fn ssbexo_luigi_down_special_throw_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special End ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_end_acmd(agent: &mut L2CAgentBase) {
    let status_kind = StatusModule::status_kind(agent.module_accessor);
    if [*FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING].contains(&status_kind) {
        if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
            if macros::is_excute(agent) {
                ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
            if macros::is_excute(agent) {
                ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, -1);
            ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_AIR_LASSO_FLAG_SHOOT_PLUNGER);
        }
        frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
        }
    }
    else {
        if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
            if macros::is_excute(agent) {
                ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        if macros::is_excute(agent) {
            ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Down Special End Effect
unsafe extern "C" fn ssbexo_luigi_down_special_end_effect(_agent: &mut L2CAgentBase) {}

//Down Special End Sound
unsafe extern "C" fn ssbexo_luigi_down_special_end_sound(_agent: &mut L2CAgentBase) {}

//Down Special End Expression
unsafe extern "C" fn ssbexo_luigi_down_special_end_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .game_acmd("game_specialsstart", ssbexo_luigi_grounded_side_special_acmd)
    .game_acmd("game_specialairsstart", ssbexo_luigi_aerial_side_special_acmd)
    .effect_acmd("effect_specialsstart", ssbexo_luigi_grounded_side_special_effect)
    .effect_acmd("effect_specialairsstart", ssbexo_luigi_aerial_side_special_effect)
    .sound_acmd("sound_specialsstart", ssbexo_luigi_grounded_side_special_sound)
    .sound_acmd("sound_specialairsstart", ssbexo_luigi_aerial_side_special_sound)
    .expression_acmd("expression_specialsstart", ssbexo_luigi_grounded_side_special_expression)
    .expression_acmd("expression_specialairsstart", ssbexo_luigi_aerial_side_special_expression)
    .game_acmd("game_specialhi", ssbexo_luigi_grounded_up_special_acmd)
    .game_acmd("game_specialairhi", ssbexo_luigi_aerial_up_special_acmd)
    .game_acmd("game_speciallw", ssbexo_luigi_down_special_acmd)
    .game_acmd("game_specialairlw", ssbexo_luigi_down_special_acmd)
    .effect_acmd("effect_speciallw", ssbexo_luigi_down_special_effect)
    .effect_acmd("effect_specialairlw", ssbexo_luigi_down_special_effect)
    .sound_acmd("sound_speciallw", ssbexo_luigi_down_special_sound)
    .sound_acmd("sound_specialairlw", ssbexo_luigi_down_special_sound)
    .expression_acmd("expression_speciallw", ssbexo_luigi_down_special_expression)
    .expression_acmd("expression_specialairlw", ssbexo_luigi_down_special_expression)
    .game_acmd("game_speciallwloop", ssbexo_luigi_down_special_loop_acmd)
    .effect_acmd("effect_speciallwloop", ssbexo_luigi_down_special_loop_effect)
    .sound_acmd("sound_speciallwloop", ssbexo_luigi_down_special_loop_sound)
    .expression_acmd("expression_speciallwloop", ssbexo_luigi_down_special_loop_expression)
    .game_acmd("game_speciallwcatchpull", ssbexo_luigi_down_special_catch_pull_acmd)
    .effect_acmd("effect_speciallwcatchpull", ssbexo_luigi_down_special_catch_pull_effect)
    .sound_acmd("sound_speciallwcatchpull", ssbexo_luigi_down_special_catch_pull_sound)
    .expression_acmd("expression_speciallwcatchpull", ssbexo_luigi_down_special_catch_pull_expression)
    .game_acmd("game_speciallwthrow", ssbexo_luigi_down_special_throw_acmd)
    .game_acmd("game_specialairlwthrow", ssbexo_luigi_down_special_throw_acmd)
    .effect_acmd("effect_speciallwthrow", ssbexo_luigi_down_special_throw_effect)
    .effect_acmd("effect_specialairlwthrow", ssbexo_luigi_down_special_throw_effect)
    .sound_acmd("sound_speciallwthrow", ssbexo_luigi_down_special_throw_sound)
    .sound_acmd("sound_specialairlwthrow", ssbexo_luigi_down_special_throw_sound)
    .expression_acmd("expression_speciallwthrow", ssbexo_luigi_down_special_throw_expression)
    .expression_acmd("expression_specialairlwthrow", ssbexo_luigi_down_special_throw_expression)
    .game_acmd("game_speciallwend", ssbexo_luigi_down_special_end_acmd)
    .game_acmd("game_specialairlwend", ssbexo_luigi_down_special_end_acmd)
    .effect_acmd("effect_speciallwend", ssbexo_luigi_down_special_end_effect)
    .effect_acmd("effect_specialairlwend", ssbexo_luigi_down_special_end_effect)
    .sound_acmd("sound_speciallwend", ssbexo_luigi_down_special_end_sound)
    .sound_acmd("sound_specialairlwend", ssbexo_luigi_down_special_end_sound)
    .expression_acmd("expression_speciallwend", ssbexo_luigi_down_special_end_expression)
    .expression_acmd("expression_specialairlwend", ssbexo_luigi_down_special_end_expression)
    .install()
    ;
}