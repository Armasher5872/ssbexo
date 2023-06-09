use super::*;

//Nair ACMD
#[acmd_script( agent = "koopa", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn ssbuexo_koopa_nair_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 10.0, 50, 60, 0, 60, 8.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("waist"), 10.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.0, 50, 60, 0, 60, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("arml"), 12.0, 50, 60, 0, 60, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 4, 0, Hash40::new("footr"), 8.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 5, 0, Hash40::new("footl"), 8.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 6.0, 50, 60, 0, 60, 8.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("waist"), 6.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 8.0, 50, 60, 0, 60, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("arml"), 8.0, 50, 60, 0, 60, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 4, 0, Hash40::new("footr"), 4.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 5, 0, Hash40::new("footl"), 4.0, 50, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair Effect
#[acmd_script( agent = "koopa", script = "effect_attackairn", category = ACMD_EFFECT)]
unsafe fn ssbuexo_koopa_nair_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 9.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            let spv1 = Vector3f{x: 0.0, y: 18.2, z: 0.0};
            let spv2 = Vector3f{x: 0.0, y: 18.15, z: 0.0};
            let spv3 = Vector3f{x: 0.0, y: 18.3, z: 0.0};
            let spv4 = Vector3f{x: 0.0, y: 18.1, z: 0.0};
            let spv5 = Vector3f{x: 0.0, y: 18.25, z: 0.0};
            let stv1 = Vector3f{x: 0.0, y: 18.25, z: 18.0};
            let stv2 = Vector3f{x: 0.0, y: 18.25, z: -18.0};
            let stv3 = Vector3f{x: 0.0, y: 18.25, z: 9.5};
            let stv4 = Vector3f{x: 0.0, y: 18.25, z: -9.5};
            let spin1: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv1, &NONE_VECTOR, 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin2: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv2, &NONE_VECTOR, 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin3: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv3, &NONE_VECTOR, 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin4: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv4, &NONE_VECTOR, 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin5: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv5, &NONE_VECTOR, 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star1: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &spv5, &NONE_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star2: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv1, &NONE_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star3: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv2, &NONE_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star4: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 1.4, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star5: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 1.4, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star6: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv3, &NONE_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star7: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv4, &NONE_VECTOR, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(fighter.module_accessor, spin1, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, spin2, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, spin3, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, spin4, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, spin5, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, star1, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, star2, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, star3, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, star4, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, star5, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, star6, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, star7, 1.0, 0.0, 1.0);
            EffectModule::set_alpha(fighter.module_accessor, spin1, 0.15);
            EffectModule::set_alpha(fighter.module_accessor, spin2, 0.15);
            EffectModule::set_alpha(fighter.module_accessor, spin3, 0.15);
            EffectModule::set_alpha(fighter.module_accessor, spin4, 0.15);
            EffectModule::set_alpha(fighter.module_accessor, spin5, 0.2);
            EffectModule::set_alpha(fighter.module_accessor, star1, 0.6);
            EffectModule::set_alpha(fighter.module_accessor, star2, 0.6);
            EffectModule::set_alpha(fighter.module_accessor, star3, 0.6);
            EffectModule::set_alpha(fighter.module_accessor, star4, 0.6);
            EffectModule::set_alpha(fighter.module_accessor, star5, 0.6);
            EffectModule::set_alpha(fighter.module_accessor, star6, 0.6);
            EffectModule::set_alpha(fighter.module_accessor, star7, 0.6);
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}

//Fair ACMD
#[acmd_script( agent = "koopa", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn ssbuexo_koopa_fair_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 9.0, 50, 80, 0, 30, 7.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 9.0, 50, 80, 0, 30, 7.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 9.0, 50, 80, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 9.0, 50, 80, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 4, 0, Hash40::new("shoulderl"), 9.0, 50, 80, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 5, 0, Hash40::new("shoulderr"), 9.0, 50, 80, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 13.0, 290, 70, 0, 30, 7.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 13.0, 290, 70, 0, 30, 7.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 13.0, 290, 70, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 13.0, 290, 70, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 4, 0, Hash40::new("shoulderl"), 13.0, 290, 70, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 5, 0, Hash40::new("shoulderr"), 13.0, 290, 70, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Fair Effect
#[acmd_script( agent = "koopa", script = "effect_attackairf", category = ACMD_EFFECT)]
unsafe fn ssbuexo_koopa_fair_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 12, 6, 180, -180, 100, 1.1, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.88, 0.35, 0.13);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
}

//Bair ACMD
#[acmd_script( agent = "koopa", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn ssbuexo_koopa_bair_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 35, 88, 0, 20, 7.0, 0.0, 8.0, -13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK)
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 70, 0, 30, 7.0, 0.0, 8.0, -13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK)
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Uair ACMD
#[acmd_script( agent = "koopa", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn ssbuexo_koopa_uair_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 20.0, 85, 60, 0, 80, 6.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(fighter, Hash40::new("mouth2"), *HIT_STATUS_INVINCIBLE);
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Uair Effect
#[acmd_script( agent = "koopa", script = "effect_attackairhi", category = ACMD_EFFECT)]
unsafe fn ssbuexo_koopa_uair_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1.5, 16, 2, -153, 90, -49, 1.5, true, *EF_FLIP_YZ, 0.7);
        macros::LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
    }
}

//Dair ACMD
#[acmd_script( agent = "koopa", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn ssbuexo_koopa_dair_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 18.0, 270, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 17.0, 361, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 16.0, 361, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 16.0, 361, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 15.0, 361, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 14.0, 361, 60, 0, 30, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair Effect
#[acmd_script( agent = "koopa", script = "effect_attackairlw", category = ACMD_EFFECT)]
unsafe fn ssbuexo_koopa_dair_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.0, true, 2);
    }
}

//Dair Landing ACMD
#[acmd_script( agent = "koopa", script = "game_landingairlw", category = ACMD_GAME)]
unsafe fn ssbuexo_koopa_dair_landing_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_koopa_nair_acmd,
        ssbuexo_koopa_nair_effect,
        ssbuexo_koopa_fair_acmd,
        ssbuexo_koopa_fair_effect,
        ssbuexo_koopa_bair_acmd,
        ssbuexo_koopa_uair_acmd,
        ssbuexo_koopa_uair_effect,
        ssbuexo_koopa_dair_acmd,
        ssbuexo_koopa_dair_effect,
        ssbuexo_koopa_dair_landing_acmd
    );
}