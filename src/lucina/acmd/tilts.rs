use super::*;

//Forward Tilt ACMD
unsafe extern "C" fn ssbexo_lucina_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 9.0, 65, 80, 0, 40, 3.5, 1.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 9.0, 65, 80, 0, 40, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 9.0, 65, 80, 0, 40, 3.5, 1.0, 0.0, 7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Tilt Effect
unsafe extern "C" fn ssbexo_lucina_forward_tilt_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
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
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
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
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

//Forward Tilt Sound
unsafe extern "C" fn ssbexo_lucina_forward_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_attackl_s01"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_lucina_up_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 8.0, 100, 100, 0, 50, 3.5, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 8.0, 100, 100, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("claviclel"), 8.0, 100, 100, 0, 50, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 8.0, 100, 100, 0, 50, 3.5, 0.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("sword1"), 8.0, 100, 100, 0, 50, 3.5, 0.0, 0.0, 2.0, Some(4.5), Some(0.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("sword1"), 8.0, 100, 100, 0, 50, 3.5, 0.0, 0.0, 6.7, Some(6.0), Some(0.0), Some(6.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 4, false);
        AttackModule::clear(agent.module_accessor, 5, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 8.0, 85, 100, 0, 50, 3.5, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 8.0, 100, 100, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("claviclel"), 8.0, 100, 100, 0, 50, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 8.0, 85, 100, 0, 50, 3.5, 0.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Tilt Effect
unsafe extern "C" fn ssbexo_lucina_up_tilt_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
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
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 8, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
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
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
        macros::AFTER_IMAGE_OFF(agent, 1);
    }
}

//Up Tilt Sound
unsafe extern "C" fn ssbexo_lucina_up_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_swing_l"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Down Tilt ACMD
unsafe extern "C" fn ssbexo_lucina_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.5, 60, 45, 0, 45, 2.7, 0.0, 2.7, 16.7, Some(0.0), Some(4.3), Some(9.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 8.5, 60, 45, 0, 45, 2.7, 0.0, 0.0, 8.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Tilt Effect
unsafe extern "C" fn ssbexo_lucina_down_tilt_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
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
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

//Down Tilt Sound
unsafe extern "C" fn ssbexo_lucina_down_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_attackl_l01"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

pub fn install() {
    Agent::new("lucina")
    .game_acmd("game_attacks3", ssbexo_lucina_forward_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attacks3", ssbexo_lucina_forward_tilt_effect, Priority::Low)
    .sound_acmd("sound_attacks3", ssbexo_lucina_forward_tilt_sound, Priority::Low)
    .game_acmd("game_attackhi3", ssbexo_lucina_up_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attackhi3", ssbexo_lucina_up_tilt_effect, Priority::Low)
    .sound_acmd("sound_attackhi3", ssbexo_lucina_up_tilt_sound, Priority::Low)
    .game_acmd("game_attacklw3", ssbexo_lucina_down_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attacklw3", ssbexo_lucina_down_tilt_effect, Priority::Low)
    .sound_acmd("sound_attacklw3", ssbexo_lucina_down_tilt_sound, Priority::Low)
    .install()
    ;
}