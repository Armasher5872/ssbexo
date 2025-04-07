use super::*;

//Neutral Special Attack ACMD
unsafe extern "C" fn ssbexo_mario_neutral_special_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 14.7, 361, 95, 0, 25, 2.0, -1.0, 0.7, 0.0, Some(-3.0), Some(1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 17.8, 361, 89, 0, 25, 5.0, 5.4, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Neutral Special Attack Effect
unsafe extern "C" fn ssbexo_mario_grounded_neutral_special_attack_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 0, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 8, 12, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
        macros::EFFECT(agent, Hash40::new("mario_fb_bullet_r"), Hash40::new("top"), 0, 8, 12, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 360, true);
        macros::FLASH(agent, 1, 0, 0, 0.353);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_bullet_r"), false, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, true);
    }
}

//Aerial Neutral Special Attack Effect
unsafe extern "C" fn ssbexo_mario_aerial_neutral_special_attack_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 0, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 8, 12, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mario_fb_bullet_r"), Hash40::new("handl"), 0.75, -1, 0, 0, 0, 0, 1.6, true);
        macros::FLASH(agent, 1, 0, 0, 0.353);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_bullet_r"), false, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, true);
    }
}

//Neutral Special Attack Sound
unsafe extern "C" fn ssbexo_mario_neutral_special_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_mario_attack05"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_smash_s01"));
    }
}

//Neutral Special Attack Expression
unsafe extern "C" fn ssbexo_mario_neutral_special_attack_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 16, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Side Special ACMD
unsafe extern "C" fn ssbexo_mario_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        let sum_speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x*1.5, 0.0);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.3);
        sv_kinetic_energy!(set_accel, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.18);
        sv_kinetic_energy!(set_limit_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.4);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 65, 60, 0, 50, 3.5, 0.0, 0.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 65, 60, 0, 50, 3.5, 0.0, 0.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}

//Side Special Effect
unsafe extern "C" fn ssbexo_mario_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_mario_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_mario_012"));
        macros::PLAY_SE(agent, Hash40::new("se_mario_jump01"));
    }
}

//Side Special Expression
unsafe extern "C" fn ssbexo_mario_side_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
}

//Side Special Loop ACMD
unsafe extern "C" fn ssbexo_mario_side_special_loop_acmd(agent: &mut L2CAgentBase) {
    let attack_frame = WorkModule::get_float(agent.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ATTACK_FRAME);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT) && attack_frame <= 32.0 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 65, 60, 0, 50, 3.5, 0.0, 0.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 65, 60, 0, 50, 3.5, 0.0, 0.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    }
}

//Side Special Loop Effect
unsafe extern "C" fn ssbexo_mario_side_special_loop_effect(_agent: &mut L2CAgentBase) {}

//Side Special Loop Sound
unsafe extern "C" fn ssbexo_mario_side_special_loop_sound(_agent: &mut L2CAgentBase) {}

//Side Special Loop Expression
unsafe extern "C" fn ssbexo_mario_side_special_loop_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Side Special Bonk ACMD
unsafe extern "C" fn ssbexo_mario_side_special_bonk_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, -1.8*lr, 0.0);
        sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.05);
    }
}

//Side Special Bonk Effect
unsafe extern "C" fn ssbexo_mario_side_special_bonk_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_step_jump"), Hash40::new("top"), 0, 0, -7, 0, 0, 180, 1.5, false);
    }
}

//Side Special Bonk Sound
unsafe extern "C" fn ssbexo_mario_side_special_bonk_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_camerahit"));
        macros::PLAY_SE(agent, Hash40::new("vc_mario_missfoot01"));
    }
}

//Side Special Bonk Expression
unsafe extern "C" fn ssbexo_mario_side_special_bonk_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//Side Special Jump ACMD
unsafe extern "C" fn ssbexo_mario_side_special_jump_acmd(_agent: &mut L2CAgentBase) {}

//Side Special Jump Effect
unsafe extern "C" fn ssbexo_mario_side_special_jump_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 0, -90, 0.6, true, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 90, -90, 0.8, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Side Special Jump Sound
unsafe extern "C" fn ssbexo_mario_side_special_jump_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_jump01"));
    }
}

//Side Special Jump Expression
unsafe extern "C" fn ssbexo_mario_side_special_jump_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//Side Special Landing ACMD
unsafe extern "C" fn ssbexo_mario_side_special_landing_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        let sum_speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x/1.5, 1.3);
    }
}

//Side Special Landing Effect
unsafe extern "C" fn ssbexo_mario_side_special_landing_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Side Special Landing Sound
unsafe extern "C" fn ssbexo_mario_side_special_landing_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_mario_passive"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_mario_landing02"));
    }
}

//Side Special Landing Expression
unsafe extern "C" fn ssbexo_mario_side_special_landing_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_mario_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 79, 25, 0, 90, 8.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 79, 25, 0, 90, 8.0, 0.0, 6.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::FT_MOTION_RATE(agent, 2.0);
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_mario_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            let spv1 = Vector3f{x: 0.0, y: 10.2, z: 0.0};
            let spv2 = Vector3f{x: 0.0, y: 10.15, z: 0.0};
            let spv3 = Vector3f{x: 0.0, y: 10.3, z: 0.0};
            let spv4 = Vector3f{x: 0.0, y: 10.1, z: 0.0};
            let spv5 = Vector3f{x: 0.0, y: 10.25, z: 0.0};
            let stv1 = Vector3f{x: 0.0, y: 9.3, z: 9.0};
            let stv2 = Vector3f{x: 0.0, y: 9.3, z: -9.0};
            let stv3 = Vector3f{x: 0.0, y: 9.3, z: 4.5};
            let stv4 = Vector3f{x: 0.0, y: 9.3, z: -4.5};
            let spin1: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv1, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin2: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv2, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin3: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv3, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin4: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv4, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin5: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv5, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star1: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &spv5, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star2: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv1, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star3: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv2, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star4: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star5: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star6: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv3, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star7: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv4, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(agent.module_accessor, spin1, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, spin2, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, spin3, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, spin4, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, spin5, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star1, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star2, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star3, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star4, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star5, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star6, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star7, 0.045, 0.345, 2.05);
            EffectModule::set_alpha(agent.module_accessor, spin1, 0.15);
            EffectModule::set_alpha(agent.module_accessor, spin2, 0.15);
            EffectModule::set_alpha(agent.module_accessor, spin3, 0.15);
            EffectModule::set_alpha(agent.module_accessor, spin4, 0.15);
            EffectModule::set_alpha(agent.module_accessor, spin5, 0.2);
            EffectModule::set_alpha(agent.module_accessor, star1, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star2, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star3, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star4, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star5, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star6, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star7, 0.6);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_mario_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_mario_rnd_attack"));
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_mario_down_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("mario")
    .game_acmd("game_specialnattack", ssbexo_mario_neutral_special_attack_acmd, Priority::Low)
    .game_acmd("game_specialairnattack", ssbexo_mario_neutral_special_attack_acmd, Priority::Low)
    .effect_acmd("effect_specialnattack", ssbexo_mario_grounded_neutral_special_attack_effect, Priority::Low)
    .effect_acmd("effect_specialairnattack", ssbexo_mario_aerial_neutral_special_attack_effect, Priority::Low)
    .sound_acmd("sound_specialnattack", ssbexo_mario_neutral_special_attack_sound, Priority::Low)
    .sound_acmd("sound_specialairnattack", ssbexo_mario_neutral_special_attack_sound, Priority::Low)
    .expression_acmd("expression_specialnattack", ssbexo_mario_neutral_special_attack_expression, Priority::Low)
    .expression_acmd("expression_specialairnattack", ssbexo_mario_neutral_special_attack_expression, Priority::Low)
    .game_acmd("game_specials", ssbexo_mario_side_special_acmd, Priority::Low)
    .game_acmd("game_specialairs", ssbexo_mario_side_special_acmd, Priority::Low)
    .effect_acmd("effect_specials", ssbexo_mario_side_special_effect, Priority::Low)
    .effect_acmd("effect_specialairs", ssbexo_mario_side_special_effect, Priority::Low)
    .sound_acmd("sound_specials", ssbexo_mario_side_special_sound, Priority::Low)
    .sound_acmd("sound_specialairs", ssbexo_mario_side_special_sound, Priority::Low)
    .expression_acmd("expression_specials", ssbexo_mario_side_special_expression, Priority::Low)
    .expression_acmd("expression_specialairs", ssbexo_mario_side_special_expression, Priority::Low)
    .game_acmd("game_specialsloop", ssbexo_mario_side_special_loop_acmd, Priority::Low)
    .effect_acmd("effect_specialsloop", ssbexo_mario_side_special_loop_effect, Priority::Low)
    .sound_acmd("sound_specialsloop", ssbexo_mario_side_special_loop_sound, Priority::Low)
    .expression_acmd("expression_specialsloop", ssbexo_mario_side_special_loop_expression, Priority::Low)
    .game_acmd("game_specialsbonk", ssbexo_mario_side_special_bonk_acmd, Priority::Low)
    .effect_acmd("effect_specialsbonk", ssbexo_mario_side_special_bonk_effect, Priority::Low)
    .sound_acmd("sound_specialsbonk", ssbexo_mario_side_special_bonk_sound, Priority::Low)
    .expression_acmd("expression_specialsbonk", ssbexo_mario_side_special_bonk_expression, Priority::Low)
    .game_acmd("game_specialsjump", ssbexo_mario_side_special_jump_acmd, Priority::Low)
    .effect_acmd("effect_specialsjump", ssbexo_mario_side_special_jump_effect, Priority::Low)
    .sound_acmd("sound_specialsjump", ssbexo_mario_side_special_jump_sound, Priority::Low)
    .expression_acmd("expression_specialsjump", ssbexo_mario_side_special_jump_expression, Priority::Low)
    .game_acmd("game_specialslanding", ssbexo_mario_side_special_landing_acmd, Priority::Low)
    .effect_acmd("effect_specialslanding", ssbexo_mario_side_special_landing_effect, Priority::Low)
    .sound_acmd("sound_specialslanding", ssbexo_mario_side_special_landing_sound, Priority::Low)
    .expression_acmd("expression_specialslanding", ssbexo_mario_side_special_landing_expression, Priority::Low)
    .game_acmd("game_speciallw", ssbexo_mario_down_special_acmd, Priority::Low)
    .game_acmd("game_specialairlw", ssbexo_mario_down_special_acmd, Priority::Low)
    .effect_acmd("effect_speciallw", ssbexo_mario_down_special_effect, Priority::Low)
    .effect_acmd("effect_specialairlw", ssbexo_mario_down_special_effect, Priority::Low)
    .sound_acmd("sound_speciallw", ssbexo_mario_down_special_sound, Priority::Low)
    .sound_acmd("sound_specialairlw", ssbexo_mario_down_special_sound, Priority::Low)
    .expression_acmd("expression_speciallw", ssbexo_mario_down_special_expression, Priority::Low)
    .expression_acmd("expression_specialairlw", ssbexo_mario_down_special_expression, Priority::Low)
    .install()
    ;
}