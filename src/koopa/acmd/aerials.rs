use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_koopa_nair_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 10.0, 50, 60, 0, 60, 8.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("waist"), 10.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 12.0, 50, 60, 0, 60, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("arml"), 12.0, 50, 60, 0, 60, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 4, 0, Hash40::new("footr"), 8.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 5, 0, Hash40::new("footl"), 8.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 6.0, 50, 60, 0, 60, 8.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("waist"), 6.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 8.0, 50, 60, 0, 60, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("arml"), 8.0, 50, 60, 0, 60, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 4, 0, Hash40::new("footr"), 4.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 5, 0, Hash40::new("footl"), 4.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair Effect
unsafe extern "C" fn ssbexo_koopa_nair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            let spv1 = Vector3f{x: 0.0, y: 18.2, z: 0.0};
            let spv2 = Vector3f{x: 0.0, y: 18.15, z: 0.0};
            let spv3 = Vector3f{x: 0.0, y: 18.3, z: 0.0};
            let spv4 = Vector3f{x: 0.0, y: 18.1, z: 0.0};
            let spv5 = Vector3f{x: 0.0, y: 18.25, z: 0.0};
            let stv1 = Vector3f{x: 0.0, y: 18.25, z: 18.0};
            let stv2 = Vector3f{x: 0.0, y: 18.25, z: -18.0};
            let stv3 = Vector3f{x: 0.0, y: 18.25, z: 9.5};
            let stv4 = Vector3f{x: 0.0, y: 18.25, z: -9.5};
            let spin1: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv1, &NONE_VECTOR, 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin2: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv2, &NONE_VECTOR, 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin3: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv3, &NONE_VECTOR, 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin4: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv4, &NONE_VECTOR, 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin5: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv5, &NONE_VECTOR, 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star1: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &spv5, &NONE_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star2: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv1, &NONE_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star3: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv2, &NONE_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star4: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 1.4, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star5: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 1.4, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star6: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv3, &NONE_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star7: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv4, &NONE_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(agent.module_accessor, spin1, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, spin2, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, spin3, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, spin4, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, spin5, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, star1, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, star2, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, star3, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, star4, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, star5, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, star6, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(agent.module_accessor, star7, 1.0, 0.0, 1.0);
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

//Nair Sound
unsafe extern "C" fn ssbexo_koopa_nair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_koopa_attackhard_s01"));
    }
}

//Nair Expression
unsafe extern "C" fn ssbexo_koopa_nair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 11, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Fair ACMD
unsafe extern "C" fn ssbexo_koopa_fair_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 18.0, 361, 80, 0, 30, 7.0, -10.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 18.0, 361, 80, 0, 30, 5.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 9.0, 70, 70, 0, 30, 6.0, -9.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 9.0, 70, 70, 0, 30, 5.0, -1.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Fair Effect
unsafe extern "C" fn ssbexo_koopa_fair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("koopa_smash_line"), Hash40::new("koopa_smash_line"), Hash40::new("top"), 0, 13, -20, 0, 0, 0, 2.3, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("koopa_smash_line"), Hash40::new("koopa_smash_line"), Hash40::new("top"), 0, 10, -26, 0, 0, 0, 2.3, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 0.9);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12, 22, 0, 0, 0, 2.6, true);
    }
}

//Fair Sound
unsafe extern "C" fn ssbexo_koopa_fair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_koopa_smash_s01"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_koopa_smash_s02"));
    }
}

//Fair Expression
unsafe extern "C" fn ssbexo_koopa_fair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Bair ACMD
unsafe extern "C" fn ssbexo_koopa_bair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 100, 60, 0, 7.0, 0.0, 12.0, -13.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 367, 100, 60, 0, 7.0, 0.0, 8.0, -18.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 367, 100, 60, 0, 7.0, 0.0, 4.0, -13.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 3.0, 367, 100, 60, 0, 7.0, 0.0, 12.0, -13.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 1, Hash40::new("top"), 3.0, 367, 100, 60, 0, 7.0, 0.0, 8.0, -18.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 1, Hash40::new("top"), 3.0, 367, 100, 60, 0, 7.0, 0.0, 4.0, -13.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 65, 70, 0, 70, 8.0, 0.0, 12.0, -13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 65, 70, 0, 70, 8.0, 0.0, 8.0, -18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 65, 70, 0, 70, 8.0, 0.0, 4.0, -13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Bair Effect
unsafe extern "C" fn ssbexo_koopa_bair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("head"), -2, 5, 0, 180, 0, 50, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 8, -13, 0, 0, 0, 0.9, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("head"), -2, 5, 0, 180, 0, 50, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 8, -15, 0, 0, 0, 0.9, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("head"), -2, 5, 0, 180, 0, 50, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 8, -17, 0, 0, 0, 0.9, true);
    }
}

//Bair Sound
unsafe extern "C" fn ssbexo_koopa_bair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("se_common_c_fire_l_short_02"));
        sv_animcmd::SET_TAKEOUT_SE(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("se_common_c_fire_l_short_02"));
        sv_animcmd::SET_TAKEOUT_SE(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("se_common_c_fire_l_short_02"));
        sv_animcmd::SET_TAKEOUT_SE(agent.lua_state_agent);
    }
}

//Bair Expression
unsafe extern "C" fn ssbexo_koopa_bair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Uair ACMD
unsafe extern "C" fn ssbexo_koopa_uair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 20.0, 85, 60, 0, 80, 6.0, 2.4, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("mouth2"), *HIT_STATUS_INVINCIBLE);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Uair Effect
unsafe extern "C" fn ssbexo_koopa_uair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1.5, 16, 2, -153, 90, -49, 1.5, true, *EF_FLIP_YZ, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 1);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_b"), true, true);
    }
}

//Uair Sound
unsafe extern "C" fn ssbexo_koopa_uair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_koopa_attackair_h01"));
    }
}

//Uair Expression
unsafe extern "C" fn ssbexo_koopa_uair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
    }
}

//Dair ACMD
unsafe extern "C" fn ssbexo_koopa_dair_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PostureModule::set_lr(agent.module_accessor, 1.0);
        PostureModule::update_rot_y_lr(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 18.0, 270, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 17.0, 361, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 16.0, 361, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 16.0, 361, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 15.0, 361, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 14.0, 361, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair Effect
unsafe extern "C" fn ssbexo_koopa_dair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.0, true, 2);
    }
}

//Dair Sound
unsafe extern "C" fn ssbexo_koopa_dair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_koopa_attackair_b01"));
    }
}

//Dair Expression
unsafe extern "C" fn ssbexo_koopa_dair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Dair Landing ACMD
unsafe extern "C" fn ssbexo_koopa_dair_landing_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("koopa")
    .game_acmd("game_attackairn", ssbexo_koopa_nair_acmd, Priority::Low)
    .effect_acmd("effect_attackairn", ssbexo_koopa_nair_effect, Priority::Low)
    .sound_acmd("sound_attackairn", ssbexo_koopa_nair_sound, Priority::Low)
    .expression_acmd("expression_attackairn", ssbexo_koopa_nair_expression, Priority::Low)
    .game_acmd("game_attackairf", ssbexo_koopa_fair_acmd, Priority::Low)
    .effect_acmd("effect_attackairf", ssbexo_koopa_fair_effect, Priority::Low)
    .sound_acmd("sound_attackairf", ssbexo_koopa_fair_sound, Priority::Low)
    .expression_acmd("expression_attackairf", ssbexo_koopa_fair_expression, Priority::Low)
    .game_acmd("game_attackairb", ssbexo_koopa_bair_acmd, Priority::Low)
    .effect_acmd("effect_attackairb", ssbexo_koopa_bair_effect, Priority::Low)
    .sound_acmd("sound_attackairb", ssbexo_koopa_bair_sound, Priority::Low)
    .expression_acmd("expression_attackairb", ssbexo_koopa_bair_expression, Priority::Low)
    .game_acmd("game_attackairhi", ssbexo_koopa_uair_acmd, Priority::Low)
    .effect_acmd("effect_attackairhi", ssbexo_koopa_uair_effect, Priority::Low)
    .sound_acmd("sound_attackairhi", ssbexo_koopa_uair_sound, Priority::Low)
    .expression_acmd("expression_attackairhi", ssbexo_koopa_uair_expression, Priority::Low)
    .game_acmd("game_attackairlw", ssbexo_koopa_dair_acmd, Priority::Low)
    .effect_acmd("effect_attackairlw", ssbexo_koopa_dair_effect, Priority::Low)
    .sound_acmd("sound_attackairlw", ssbexo_koopa_dair_sound, Priority::Low)
    .expression_acmd("expression_attackairlw", ssbexo_koopa_dair_expression, Priority::Low)
    .game_acmd("game_landingairlw", ssbexo_koopa_dair_landing_acmd, Priority::Low)
    .install()
    ;
}