use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_captain_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 20.0, 43, 94, 0, 36, 3.8, 2.2, 0.7, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 20.0, 43, 94, 0, 36, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Hi ACMD
unsafe extern "C" fn ssbexo_captain_forward_smash_hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 20.0, 70, 94, 0, 36, 3.8, 2.2, 0.7, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 20.0, 70, 94, 0, 36, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Lw ACMD
unsafe extern "C" fn ssbexo_captain_forward_smash_lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 20.0, 40, 94, 0, 36, 3.8, 2.2, 0.7, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 20.0, 40, 94, 0, 36, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_captain_forward_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("captain_smash_line"), Hash40::new("top"), 0, 10, -16, 0, 0, 0, 1.2, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Smash Hi Effect
unsafe extern "C" fn ssbexo_captain_forward_smash_hi_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("captain_smash_line"), Hash40::new("top"), 0, 2, -16, -25, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Smash Lw Effect
unsafe extern "C" fn ssbexo_captain_forward_smash_lw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("captain_smash_line"), Hash40::new("top"), 0, 16, -12, 25, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("captain_smash_line"), true, true);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_captain_up_smash_acmd(agent: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let vector = smash::phx::Vector2f{x: 5.0, y: 29.0};
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        SIZE0[get_player_number(module_accessor)] = 6.0;
        SIZE1[get_player_number(module_accessor)] = 5.5;
        SIZE2[get_player_number(module_accessor)] = 5.0;
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            SIZE0[get_player_number(module_accessor)] += 4.0;
            SIZE1[get_player_number(module_accessor)] += 4.0;
            SIZE2[get_player_number(module_accessor)] += 4.0;
        }
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 90, 90, 150, 0, 6.0, 0.0, 7.5, 7.5, Some(0.0), Some(3.5), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 3, 0, Hash40::new("footr"), 9.0, 80, 90, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 9.0, 80, 90, 10, 5, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legr"), 9.0, 80, 90, 20, 10, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &vector, 7.5 as u32, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 83, 80, 0, 70, SIZE0[get_player_number(module_accessor)], 0.0, 29.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 85, 81, 0, 70, SIZE1[get_player_number(module_accessor)], 0.0, 21.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 70, 88, 0, 70, SIZE2[get_player_number(module_accessor)], 0.0, 16.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_captain_up_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("legr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("legr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("captain_smash_arc"), Hash40::new("top"), 0, 18, 2, 68, 0, 151, 1, true);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("legr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("captain_smash_arc"), Hash40::new("top"), 0, 17, 1.5, 73, -15, 165, 1.05, true);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_captain_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 14.0, 28, 90, 0, 30, 4.5, 4.9, -0.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 14.0, 60, 90, 0, 30, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("legr"), 14.0, 120, 90, 0, 30, 3.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("kneer"), 18.0, 28, 84, 0, 20, 4.5, 4.9, -0.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 1, Hash40::new("kneer"), 18.0, 60, 84, 0, 20, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 1, Hash40::new("legr"), 18.0, 120, 84, 0, 20, 3.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_captain_down_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("legr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("captain_smash_lw"), Hash40::new("top"), -2, 11, 6, 44, 80, 115, 0.95, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.8);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("legr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("captain_smash_arc"), true, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("legr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("captain_smash_line"), Hash40::new("top"), 1, 11, 2, 15, 180, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_damage_fire"), Hash40::new("legr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 7, -15.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true);
    }
}

pub fn install() {
    Agent::new("captain")
    .game_acmd("game_attacks4", ssbexo_captain_forward_smash_acmd)
    .game_acmd("game_attacks4hi", ssbexo_captain_forward_smash_hi_acmd)
    .game_acmd("game_attacks4lw", ssbexo_captain_forward_smash_lw_acmd)
    .effect_acmd("effect_attacks4", ssbexo_captain_forward_smash_effect)
    .effect_acmd("effect_attacks4hi", ssbexo_captain_forward_smash_hi_effect)
    .effect_acmd("effect_attacks4lw", ssbexo_captain_forward_smash_lw_effect)
    .game_acmd("game_attackhi4", ssbexo_captain_up_smash_acmd)
    .effect_acmd("effect_attackhi4", ssbexo_captain_up_smash_effect)
    .game_acmd("game_attacklw4", ssbexo_captain_down_smash_acmd)
    .effect_acmd("effect_attacklw4", ssbexo_captain_down_smash_effect)
    .install()
    ;
}