use super::*;

//Shield Special Effect
#[acmd_script( agent = "pzenigame", script = "effect_specialshield", category = ACMD_EFFECT)]
unsafe fn ssbuexo_squirtle_shield_special_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 50.0);
    for _ in 0..5 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_steam3"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 2.0, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_steam3"), Hash40::new("top"), 0, 20, -10, 0, 0, 0, 2.0, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_steam3"), Hash40::new("top"), 0, 20, 10, 0, 0, 0, 2.0, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 0.2);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_atk_speedline"), Hash40::new("top"), 0, 15, 0, -90, 0, 0, 1.5, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.698, 1.0, 1.0);
        }
        wait(fighter.lua_state_agent, 6.0);
    }
}

//Shield Special Sound
#[acmd_script( agent = "pzenigame", script = "sound_specialshield", category = ACMD_SOUND)]
unsafe fn ssbuexo_squirtle_shield_special_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pzenigame_jump03"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pzenigame_landing01"));
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_pzenigame_attack06"));
    }
}

//Up Special ACMD
#[acmd_script( agent = "pzenigame", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME)]
unsafe fn ssbuexo_squirtle_up_special_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4, 367, 100, 110, 0, 9.0, 0.0, 1.0, 9.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS); 
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 55, 152, 0, 70, 10.0, 0.0, 1.0, 9.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_squirtle_shield_special_effect,
        ssbuexo_squirtle_shield_special_sound,
        ssbuexo_squirtle_up_special_acmd
    );
}