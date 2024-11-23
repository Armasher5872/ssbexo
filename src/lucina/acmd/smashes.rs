use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_lucina_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 15.0, 361, 80, 0, 65, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 15.0, 361, 80, 0, 65, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("bust"), 15.0, 361, 80, 0, 65, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 15.0, 361, 80, 0, 65, 3.5, 1.0, 0.0, 7.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_lucina_forward_smash_effect(agent: &mut L2CAgentBase) {
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
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0.0, 8, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
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
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
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
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Forward Smash Sound
unsafe extern "C" fn ssbexo_lucina_forward_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucina_attack05"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_smash_s01"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Forward Smash Expression
unsafe extern "C" fn ssbexo_lucina_forward_smash_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    execute(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_lucina_up_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 14.25, 89, 90, 0, 42, 5.8, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 14.25, 89, 90, 0, 42, 4.6, 0.0, 0.0, 7.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 14.25, 89, 90, 0, 42, 5.8, 0.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 125, 100, 155, 0, 5.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-9.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_lucina_up_smash_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
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
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 5, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 18, 0, -90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.15, 0, 10, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
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
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 1);
    }
}

//Up Smash Sound
unsafe extern "C" fn ssbexo_lucina_up_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucina_attack06"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_smash_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_lucina_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 14.0, 75, 88, 0, 45, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 14.0, 75, 88, 0, 45, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 14.0, 75, 88, 0, 45, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 14.0, 75, 88, 0, 45, 3.5, 1.0, 0.0, 7.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 14.0, 75, 88, 0, 45, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 14.0, 75, 88, 0, 45, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 14.0, 75, 88, 0, 45, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 14.0, 75, 88, 0, 45, 3.5, 0.5, 0.0, 7.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_lucina_down_smash_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
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
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 12, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_light"), Hash40::new("sword1"), 0, 0, 10, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
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
        macros::AFTER_IMAGE_OFF(agent, 5);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_light"), false, true);
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
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 10, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_light"), Hash40::new("sword1"), 0, 0, 10, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_damage_elec"), false, true);
        let effect_a = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 2.0}, &Vector3f{x: 0.0, y: 0.0, z: 2.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_b = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 5.0}, &Vector3f{x: 0.0, y: 0.0, z: 5.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        let effect_c = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 8.0}, &Vector3f{x: 0.0, y: 0.0, z: 8.0}, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(agent.module_accessor, effect_a as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_b as u32, 0.0, 0.635, 0.91);
        EffectModule::set_rgb(agent.module_accessor, effect_c as u32, 0.0, 0.635, 0.91);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 5);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_light"), false, true);
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
    }
}

//Down Smash Sound
unsafe extern "C" fn ssbexo_lucina_down_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucina_attack07"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_smash_l01"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
    wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucina_smash_l01"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

pub fn install() {
    Agent::new("lucina")
    .game_acmd("game_attacks4", ssbexo_lucina_forward_smash_acmd, Priority::Low)
    .effect_acmd("effect_attacks4", ssbexo_lucina_forward_smash_effect, Priority::Low)
    .sound_acmd("sound_attacks4", ssbexo_lucina_forward_smash_sound, Priority::Low)
    .expression_acmd("expression_attacks4", ssbexo_lucina_forward_smash_expression, Priority::Low)
    .game_acmd("game_attackhi4", ssbexo_lucina_up_smash_acmd, Priority::Low)
    .effect_acmd("effect_attackhi4", ssbexo_lucina_up_smash_effect, Priority::Low)
    .sound_acmd("sound_attackhi4", ssbexo_lucina_up_smash_sound, Priority::Low)
    .game_acmd("game_attacklw4", ssbexo_lucina_down_smash_acmd, Priority::Low)
    .effect_acmd("effect_attacklw4", ssbexo_lucina_down_smash_effect, Priority::Low)
    .sound_acmd("sound_attacklw4", ssbexo_lucina_down_smash_sound, Priority::Low)
    .install()
    ;
}