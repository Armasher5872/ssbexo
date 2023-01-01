#![allow(unused_macros)]
use {
    crate::functions::{
        DISCHARGE_ACTIVE,
        VOLT_SWITCH_COUNT
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lib::lua_const::*,
        lua2cpp::L2CAgentBase,
        phx::Hash40,
    },
    smashline::*,
    smash_script::*,
};

//Shield Special ACMD
#[acmd_script( agent = "pichu", script = "game_specialshield", category = ACMD_GAME)]
unsafe fn ssbuexo_pichu_shield_special_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 2.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 65, 0, 90, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 59.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Shield Special Effect
#[acmd_script( agent = "pichu", script = "effect_specialshield", category = ACMD_EFFECT)]
unsafe fn ssbuexo_pichu_shield_special_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.1, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 13.0);
    }
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_elec2"), false, true);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_kaminari_hit2"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.8, true);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_kaminari_hit"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.9, true);
    }
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0, 0, 0, 0);
            macros::BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::BURN_COLOR_NORMAL(fighter);
            macros::COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 80.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_kaminari_hit2"), false, true);
        macros::FLASH(fighter, 0, 0, 0, 0);
        macros::BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek"), false, true);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Shield Special Sound
#[acmd_script( agent = "pichu", script = "sound_specialshield", category = ACMD_SOUND)]
unsafe fn ssbuexo_pichu_shield_special_sound(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_pichu_appeal_h01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_pichu_appeal01"));
    }
    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter,Hash40::new("vc_pichu_appeal01"));
        macros::STOP_SE(fighter,Hash40::new("se_pichu_appeal_h01"));
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        smash::app::sv_module_access::sound(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pichu_special_l03"));
    }
}

//Neutral Special Shoot ACMD
#[acmd_script( agent = "pichu", scripts = ["game_specialn", "game_specialairn", "game_specialnshoot"], category = ACMD_GAME)]
unsafe fn ssbuexo_pichu_neutral_special_shoot_acmd(fighter: &mut L2CAgentBase) 
{
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if DISCHARGE_ACTIVE[entry_id] == false {
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 100, 35, 0, 80, 5.3, 0.0, 4.7, 11.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 100, 35, 0, 80, 4.3, 0.0, 4.7, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    if DISCHARGE_ACTIVE[entry_id] == true {
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, 0);
            DamageModule::add_damage(fighter.module_accessor, -1.4, 0);
        }
    }
}

//Grounded Electroweb ACMD
#[acmd_script( agent = "pichu_dengekidama", script = "game_regular", category = ACMD_GAME)]
unsafe fn ssbuexo_pichu_grounded_electroweb_acmd(fighter: &mut L2CAgentBase) 
{ 
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 30, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
}

//Aerial Electroweb ACMD
#[acmd_script( agent = "pichu_dengeki", script = "game_regular", category = ACMD_GAME)]
unsafe fn ssbuexo_pichu_aerial_electroweb_acmd(fighter: &mut L2CAgentBase) 
{ 
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 30, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
}

//Grounded Tackle ACMD
#[acmd_script( agent = "pichu", script = "game_specialstackle", category = ACMD_GAME)]
unsafe fn ssbuexo_pichu_grounded_tackle_acmd(fighter: &mut L2CAgentBase) 
{ 
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 8.0, 40, 55, 0, 65, 4.0, 5.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 69.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
}

//Aerial Tackle ACMD
#[acmd_script( agent = "pichu", script = "game_specialairstackle", category = ACMD_GAME)]
unsafe fn ssbuexo_pichu_aerial_tackle_acmd(fighter: &mut L2CAgentBase) 
{ 
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 8.0, 40, 55, 0, 65, 4.0, 5.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 69.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
    }
}

//Wild Charge ACMD
#[acmd_script( agent = "pichu", script = "game_specialairs", category = ACMD_GAME)]
unsafe fn ssbuexo_pichu_wild_charge_acmd(fighter: &mut L2CAgentBase) 
{ 
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 90, 0, 15, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, true);
    }
}

//Up Special 1 ACMD
#[acmd_script( agent = "pichu", scripts = ["game_specialhi1", "game_specialairhi1"], category = ACMD_GAME)]
unsafe fn ssbuexo_pichu_up_special_1_acmd(fighter: &mut L2CAgentBase) 
{
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if DISCHARGE_ACTIVE[entry_id] == true {
            VOLT_SWITCH_COUNT[entry_id] += 1;
        }
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

//Up Special 2 ACMD
#[acmd_script( agent = "pichu", scripts = ["game_specialhi2", "game_specialairhi2"], category = ACMD_GAME)]
unsafe fn ssbuexo_pichu_up_special_2_acmd(fighter: &mut L2CAgentBase) 
{
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if DISCHARGE_ACTIVE[entry_id] == true {
            VOLT_SWITCH_COUNT[entry_id] += 1;
        }
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_pichu_shield_special_acmd,
        ssbuexo_pichu_shield_special_effect,
        ssbuexo_pichu_shield_special_sound,
        ssbuexo_pichu_neutral_special_shoot_acmd,
        ssbuexo_pichu_grounded_electroweb_acmd,
        ssbuexo_pichu_aerial_electroweb_acmd,
        ssbuexo_pichu_grounded_tackle_acmd,
        ssbuexo_pichu_aerial_tackle_acmd,
        ssbuexo_pichu_wild_charge_acmd,
        ssbuexo_pichu_up_special_1_acmd,
        ssbuexo_pichu_up_special_2_acmd
    );
}