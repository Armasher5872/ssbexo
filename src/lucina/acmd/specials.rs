use super::*;

//Neutral Special Start Effect
unsafe extern "C" fn ssbexo_lucina_neutral_special_start_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 3, 0, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
}

//Neutral Special End Max ACMD
unsafe extern "C" fn ssbexo_lucina_neutral_special_end_max_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 1.5, y: 0.0, z: 0.0});
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Neutral Special End Max Effect
unsafe extern "C" fn ssbexo_lucina_neutral_special_end_max_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_purple"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_breaker_sting"), Hash40::new("top"), -0.0, 9.7, 12, 0, 0, 0, 1.2, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), 0.7, -0.0, 12, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 16, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_purple"), false, true);
    }
}

//Neutral Special End Max Hi ACMD
unsafe extern "C" fn ssbexo_lucina_neutral_special_end_max_hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 1.5, y: 3.5, z: 0.0});
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(12.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 14.0, 25.0, Some(0.0), Some(14.6), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 11.5, 25.0, Some(0.0), Some(12.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Neutral Special End Max Hi Effect
unsafe extern "C" fn ssbexo_lucina_neutral_special_end_max_hi_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_purple"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_breaker_sting"), Hash40::new("top"), -0.0, 10.5, 15, -20, 0, 0, 1.2, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, -20, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_purple"), false, true);
    }
}

//Neutral Special End Max Lw ACMD
unsafe extern "C" fn ssbexo_lucina_neutral_special_end_max_lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 1.5, y: -3.5, z: 0.0});
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, 4.5, 8.0, Some(0.0), Some(1.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 361, 90, 0, 45, 2.5, 0.0, -0.5, 25.0, Some(0.0), Some(-1.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Neutral Special End Max Lw Effect
unsafe extern "C" fn ssbexo_lucina_neutral_special_end_max_lw_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_purple"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_breaker_sting"), Hash40::new("top"), -0.0, 5.5, 15, 12, 0, 0, 1.2, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 12, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_purple"), false, true);
    }
}

//Neutral Special End Max Sound
unsafe extern "C" fn ssbexo_lucina_neutral_special_end_max_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucina_special_n01"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_n04"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Grounded Side Special ACMD
unsafe extern "C" fn ssbexo_lucina_grounded_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_OFF);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 12.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(-40.0), *COLLISION_KIND_MASK_ALL, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_ON);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 140, 70, 0, 20, 8.0, 0.0, 9.0, 11.7, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 140, 70, 0, 20, 7.0, 0.0, 9.0, 15.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 140, 70, 0, 20, 7.0, 0.0, 9.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Side Special Effect
unsafe extern "C" fn ssbexo_lucina_grounded_side_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 0, 0, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_hit_dead"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, -40, 0, 0, 0, 2.5, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_red"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT(agent, Hash40::new("sys_hit_dead"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_4s"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_red"), false, true);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_lucina_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_s04s"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucina_attack02"));
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Grounded Side Special Expression
unsafe extern "C" fn ssbexo_lucina_grounded_side_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y_MINUS));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Aerial Side Special ACMD
unsafe extern "C" fn ssbexo_lucina_aerial_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_OFF);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 12.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(-40.0), *COLLISION_KIND_MASK_ALL, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_ON);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 140, 70, 0, 20, 8.0, 0.0, 9.0, 11.7, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 140, 70, 0, 20, 7.0, 0.0, 9.0, 15.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 140, 70, 0, 20, 7.0, 0.0, 9.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_GRAVITY_ON);
    }
}

//Aerial Side Special Effect
unsafe extern "C" fn ssbexo_lucina_aerial_side_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 0, 0, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_hit_dead"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, -40, 0, 0, 0, 2.5, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_red"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT(agent, Hash40::new("sys_hit_dead"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_4s"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_red"), false, true);
    }
}

//Aerial Side Special Expression
unsafe extern "C" fn ssbexo_lucina_aerial_side_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y_MINUS));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Up Special ACMD
unsafe extern "C" fn ssbexo_lucina_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 2.0, 85, 120, 120, 0, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 2.0, 85, 120, 120, 0, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 85, 120, 120, 0, 4.0, 0.0, 8.0, 8.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 85, 120, 120, 0, 4.0, 0.0, 8.0, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT_ON);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 2.0, 75, 90, 120, 0, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 2.0, 75, 90, 120, 0, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 75, 90, 120, 0, 4.0, 0.0, 8.0, 8.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 75, 90, 120, 0, 4.0, 0.0, 8.0, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 2.0, 65, 60, 120, 0, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 2.0, 65, 60, 120, 0, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 65, 60, 120, 0, 4.0, 0.0, 8.0, 8.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 65, 60, 120, 0, 4.0, 0.0, 8.0, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 2.0, 55, 30, 120, 0, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 2.0, 55, 30, 120, 0, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 55, 30, 120, 0, 4.0, 0.0, 8.0, 8.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 55, 30, 120, 0, 4.0, 0.0, 8.0, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT_ON);
    }
}

//Up Special Effect
unsafe extern "C" fn ssbexo_lucina_up_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0, 0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_jump"), Hash40::new("top"), -0.0, 0, -5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("lucina_dolphin_jump"), -1);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("lucina_dolphin_swing"), -1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_shadow"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_blue"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, -10, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_blue"), false, true);
    }
}

//Up Special Sound
unsafe extern "C" fn ssbexo_lucina_up_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_h01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_special_h"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Up Special 2 ACMD
unsafe extern "C" fn ssbexo_lucina_up_special_2_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 65, 120, 0, 55, 8.0, 0.0, 9.0, 11.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 65, 120, 0, 55, 7.0, 0.0, 9.0, 15.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 65, 120, 0, 55, 7.0, 0.0, 9.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Special 2 Effect
unsafe extern "C" fn ssbexo_lucina_up_special_2_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 0, 0, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0, 0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 5);
    }
}

//Up Special 2 Sound
unsafe extern "C" fn ssbexo_lucina_up_special_2_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_special_h"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_s04s"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Up Special 2 Expression
unsafe extern "C" fn ssbexo_lucina_up_special_2_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Down Special Start ACMD
unsafe extern "C" fn ssbexo_lucina_down_special_start_acmd(_agent: &mut L2CAgentBase) {}

//Down Special Start Effect
unsafe extern "C" fn ssbexo_lucina_down_special_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("lucina_counter_flash"), Hash40::new("top"), 0, 11, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::FLASH(agent, 1, 1, 1, 0.75);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

//Down Special Start Sound
unsafe extern "C" fn ssbexo_lucina_down_special_start_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_l01"));
    }
}

//Down Special Start Expression
unsafe extern "C" fn ssbexo_lucina_down_special_start_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special Loop ACMD
unsafe extern "C" fn ssbexo_lucina_down_special_loop_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 1, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 2, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 3, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 4, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 5, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 6, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
    frame(agent.lua_state_agent, 105.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 7, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
    frame(agent.lua_state_agent, 120.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 8, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
    frame(agent.lua_state_agent, 135.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 9, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
    frame(agent.lua_state_agent, 150.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 10, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
    frame(agent.lua_state_agent, 165.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 11, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
}

//Down Special Loop Effect
unsafe extern "C" fn ssbexo_lucina_down_special_loop_effect(agent: &mut L2CAgentBase) {
    for _ in 0..2 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 15.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
            let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.5, 15, 0, 4, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 30.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
            let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.5, 15, 0, 4, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 45.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
            let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.62, 15, 0, 4, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 60.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
            let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.62, 15, 0, 4, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 75.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
            let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.75, 15, 0, 4, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 90.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
            let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.75, 15, 0, 4, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 105.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
            let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.87, 15, 0, 4, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 120.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
            let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.87, 15, 0, 4, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 135.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
            let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -12, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 150.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
            let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
            EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -12, 0, 0, 0, 0, 0, 1.1, 15, 0, 4, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 165.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::EFFECT(agent, Hash40::new("sys_explosion_flash"), Hash40::new("sword1"), 0, 0, 8, 0, 0, 0, 1.0, 5, 5, 0, 0, 0, 0, false);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -12, 0, 0, 0, 0, 0, 1.25, 15, 0, 4, 0, 0, 0, false);
        macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

//Down Special Loop Sound
unsafe extern "C" fn ssbexo_lucina_down_special_loop_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
        }
        wait(agent.lua_state_agent, 2.0);   
    }
}

//Down Special Loop Expression
unsafe extern "C" fn ssbexo_lucina_down_special_loop_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 2, 200, 2, 0.3, -4, 8, 30, 20, 80);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 140.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special Attack ACMD
unsafe extern "C" fn ssbexo_lucina_down_special_attack_acmd(agent: &mut L2CAgentBase) {
    let effect_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    let distance = 13.0+(5.0*effect_count as f32);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 9.0, 361, 50, 0, 50, 5.0, 1.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 9.0, 361, 50, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("claviclel"), 9.0, 361, 50, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 9.0, 361, 50, 0, 50, 5.0, 1.0, 0.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 9.0, 361, 50, 0, 50, 7.0, 0.0, 7.0, distance, Some(0.0), Some(7.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    }
}

//Down Special Attack Effect
unsafe extern "C" fn ssbexo_lucina_down_special_attack_effect(agent: &mut L2CAgentBase) {
    let effect_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 10, Hash40::new("sword1"), 0, 0, 1.7, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    match effect_count {
        1 => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 1.2);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
        2 => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 1.4);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
        3 => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 1.6);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
        4 => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 1.8);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
        5 => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 2.0);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
        6 => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 2.2);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
        7 => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 2.4);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
        8 => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 2.6);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
        9 => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 2.8);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
        10 => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 3.0);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
        11 => {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_genesis_beam"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
        _ => {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_mc_1"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
                macros::LAST_EFFECT_SET_SCALE_W(agent, 1.0, 1.0, 1.0);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_genesis_beam"), true, true);
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

//Down Special Attack Sound
unsafe extern "C" fn ssbexo_lucina_down_special_attack_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_l02"));
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_special_l03"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_special_l"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Down Special Attack Expression
unsafe extern "C" fn ssbexo_lucina_down_special_attack_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

pub fn install() {
    Agent::new("lucina")
    .effect_acmd("effect_specialnstart", ssbexo_lucina_neutral_special_start_effect, Priority::Low)
    .game_acmd("game_specialnendmax", ssbexo_lucina_neutral_special_end_max_acmd, Priority::Low)
    .game_acmd("game_specialnendmaxhi", ssbexo_lucina_neutral_special_end_max_hi_acmd, Priority::Low)
    .game_acmd("game_specialnendmaxlw", ssbexo_lucina_neutral_special_end_max_lw_acmd, Priority::Low)
    .game_acmd("game_specialairnendmax", ssbexo_lucina_neutral_special_end_max_acmd, Priority::Low)
    .game_acmd("game_specialairnendmaxhi", ssbexo_lucina_neutral_special_end_max_hi_acmd, Priority::Low)
    .game_acmd("game_specialairnendmaxlw", ssbexo_lucina_neutral_special_end_max_lw_acmd, Priority::Low)
    .effect_acmd("effect_specialnendmax", ssbexo_lucina_neutral_special_end_max_effect, Priority::Low)
    .effect_acmd("effect_specialnendmaxhi", ssbexo_lucina_neutral_special_end_max_hi_effect, Priority::Low)
    .effect_acmd("effect_specialnendmaxlw", ssbexo_lucina_neutral_special_end_max_lw_effect, Priority::Low)
    .effect_acmd("effect_specialairnendmax", ssbexo_lucina_neutral_special_end_max_effect, Priority::Low)
    .effect_acmd("effect_specialairnendmaxhi", ssbexo_lucina_neutral_special_end_max_hi_effect, Priority::Low)
    .effect_acmd("effect_specialairnendmaxlw", ssbexo_lucina_neutral_special_end_max_lw_effect, Priority::Low)
    .sound_acmd("sound_specialnendmax", ssbexo_lucina_neutral_special_end_max_sound, Priority::Low)
    .sound_acmd("sound_specialnendmaxhi", ssbexo_lucina_neutral_special_end_max_sound, Priority::Low)
    .sound_acmd("sound_specialnendmaxlw", ssbexo_lucina_neutral_special_end_max_sound, Priority::Low)
    .sound_acmd("sound_specialairnendmax", ssbexo_lucina_neutral_special_end_max_sound, Priority::Low)
    .sound_acmd("sound_specialairnendmaxhi", ssbexo_lucina_neutral_special_end_max_sound, Priority::Low)
    .sound_acmd("sound_specialairnendmaxlw", ssbexo_lucina_neutral_special_end_max_sound, Priority::Low)
    .game_acmd("game_specials", ssbexo_lucina_grounded_side_special_acmd, Priority::Low)
    .effect_acmd("effect_specials", ssbexo_lucina_grounded_side_special_effect, Priority::Low)
    .sound_acmd("sound_specials", ssbexo_lucina_side_special_sound, Priority::Low)
    .expression_acmd("expression_specials", ssbexo_lucina_grounded_side_special_expression, Priority::Low)
    .game_acmd("game_specialairs", ssbexo_lucina_aerial_side_special_acmd, Priority::Low)
    .effect_acmd("effect_specialairs", ssbexo_lucina_aerial_side_special_effect, Priority::Low)
    .sound_acmd("sound_specialairs", ssbexo_lucina_side_special_sound, Priority::Low)
    .expression_acmd("expression_specialairs", ssbexo_lucina_aerial_side_special_expression, Priority::Low)
    .game_acmd("game_specialhi", ssbexo_lucina_up_special_acmd, Priority::Low)
    .effect_acmd("effect_specialhi", ssbexo_lucina_up_special_effect, Priority::Low)
    .sound_acmd("sound_specialhi", ssbexo_lucina_up_special_sound, Priority::Low)
    .game_acmd("game_specialairhi", ssbexo_lucina_up_special_acmd, Priority::Low)
    .effect_acmd("effect_specialairhi", ssbexo_lucina_up_special_effect, Priority::Low)
    .sound_acmd("sound_specialairhi", ssbexo_lucina_up_special_sound, Priority::Low)
    .game_acmd("game_specialhi2", ssbexo_lucina_up_special_2_acmd, Priority::Low)
    .effect_acmd("effect_specialhi2", ssbexo_lucina_up_special_2_effect, Priority::Low)
    .sound_acmd("sound_specialhi2", ssbexo_lucina_up_special_2_sound, Priority::Low)
    .expression_acmd("expression_specialhi2", ssbexo_lucina_up_special_2_expression, Priority::Low)
    .game_acmd("game_speciallwstart", ssbexo_lucina_down_special_start_acmd, Priority::Low)
    .effect_acmd("effect_speciallwstart", ssbexo_lucina_down_special_start_effect, Priority::Low)
    .sound_acmd("sound_speciallwstart", ssbexo_lucina_down_special_start_sound, Priority::Low)
    .expression_acmd("expression_speciallwstart", ssbexo_lucina_down_special_start_expression, Priority::Low)
    .game_acmd("game_specialairlwstart", ssbexo_lucina_down_special_start_acmd, Priority::Low)
    .effect_acmd("effect_specialairlwstart", ssbexo_lucina_down_special_start_effect, Priority::Low)
    .sound_acmd("sound_specialairlwstart", ssbexo_lucina_down_special_start_sound, Priority::Low)
    .expression_acmd("expression_specialairlwstart", ssbexo_lucina_down_special_start_expression, Priority::Low)
    .game_acmd("game_speciallw", ssbexo_lucina_down_special_loop_acmd, Priority::Low)
    .effect_acmd("effect_speciallw", ssbexo_lucina_down_special_loop_effect, Priority::Low)
    .sound_acmd("sound_speciallw", ssbexo_lucina_down_special_loop_sound, Priority::Low)
    .expression_acmd("expression_speciallw", ssbexo_lucina_down_special_loop_expression, Priority::Low)
    .game_acmd("game_specialairlw", ssbexo_lucina_down_special_loop_acmd, Priority::Low)
    .effect_acmd("effect_specialairlw", ssbexo_lucina_down_special_loop_effect, Priority::Low)
    .sound_acmd("sound_specialairlw", ssbexo_lucina_down_special_loop_sound, Priority::Low)
    .expression_acmd("expression_specialairlw", ssbexo_lucina_down_special_loop_expression, Priority::Low)
    .game_acmd("game_speciallwattack", ssbexo_lucina_down_special_attack_acmd, Priority::Low)
    .effect_acmd("effect_speciallwattack", ssbexo_lucina_down_special_attack_effect, Priority::Low)
    .sound_acmd("sound_speciallwattack", ssbexo_lucina_down_special_attack_sound, Priority::Low)
    .expression_acmd("expression_speciallwattack", ssbexo_lucina_down_special_attack_expression, Priority::Low)
    .game_acmd("game_specialairlwattack", ssbexo_lucina_down_special_attack_acmd, Priority::Low)
    .effect_acmd("effect_specialairlwattack", ssbexo_lucina_down_special_attack_effect, Priority::Low)
    .sound_acmd("sound_specialairlwattack", ssbexo_lucina_down_special_attack_sound, Priority::Low)
    .expression_acmd("expression_specialairlwattack", ssbexo_lucina_down_special_attack_expression, Priority::Low)
    .install()
    ;
}