#![allow(unused_macros)]
use {
    crate::functions::variables::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CAgentBase,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smashline::*,
    smash_script::*,
};

//PSI Offense Up ACMD
#[acmd_script( agent = "ness", script = "game_specialshield", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_shield_special_acmd(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(fighter.lua_state_agent, 107.0);
    if OFFENSE_UP_ACTIVE[entry_id] == true {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 0.5);
            macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        }
    }
    else {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 110.0);
    if OFFENSE_UP_ACTIVE[entry_id] == true {
        if macros::is_excute(fighter) {
            macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        }
    }
    else {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 118.0);
    if OFFENSE_UP_ACTIVE[entry_id] == true {
        if macros::is_excute(fighter) {
            HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 17.0, 95, 77, 0, 48, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            macros::ATTACK(fighter, 1, 0, Hash40::new("waist"), 12.0, 85, 77, 0, 48, 11.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);  
        }
    }
    else {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 123.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 160.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
}

//PSI Offense Up Effect
#[acmd_script( agent = "ness", script = "effect_specialshield", category = ACMD_EFFECT)]
unsafe fn ssbuexo_ness_shield_special_effect(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let vectora = Vector3f {x: 0.0, y: 5.0, z: 0.0};
    let vectorb = Vector3f {x: 0.0, y: 0.0, z: 0.0};
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    for _ in 0..13 {
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 9, 0, 9, 0, 0, 0, false);
            macros::FLASH(fighter, 0.7, 1, 0.2, 0.4);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0.8, 1, 1, 0.6);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 4.0);
    }
    if OFFENSE_UP_ACTIVE[entry_id] == true {
        frame(fighter.lua_state_agent, 107.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("ness_pkfl_hold"), Hash40::new("waist"), 0, 0, 0.0, 0.0, -90, 0, 0.75, 0, 0, 0, 0, 0, 360, true);
            macros::EFFECT(fighter, Hash40::new("ness_psi_atk"), Hash40::new("waist"), 0, 0, 0.0, 0.0, -90, 0, 0.75, 0, 0, 0, 0, 0, 360, true);
            macros::FLASH(fighter, 1, 0, 0, 0.4);
        }
        frame(fighter.lua_state_agent, 110.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("ness_pkfl_hold"), Hash40::new("waist"), 0, 0, 0.0, 0.0, -90, 0, 1.5, 0, 0, 0, 0, 0, 360, true);
            macros::EFFECT(fighter, Hash40::new("ness_psi_atk"), Hash40::new("waist"), 0, 0, 0.0, 0.0, -90, 0, 1.5, 0, 0, 0, 0, 0, 360, true);
            macros::FLASH(fighter, 2, 0, 0, 0.8);
        }
        frame(fighter.lua_state_agent, 118.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("ness_pkfl_bomb_max"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("ness_psi_atk"), Hash40::new("waist"), 0, 0, 0.0, 0.0, -90, 0, 2.0, 0, 0, 0, 0, 0, 360, true);
            macros::COL_NORMAL(fighter);
            smash::app::FighterUtil::flash_eye_info(fighter.module_accessor);
		    EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("head"), &vectora, &vectorb, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
    }
    else {
        frame(fighter.lua_state_agent, 108.0);
        for _ in 0..3 {
            if macros::is_excute(fighter) {
                macros::FLASH(fighter, 0.7, 1, 0.2, 0.4);
            }
            wait(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                macros::FLASH(fighter, 2, 1, 1, 0.6);
            }
            wait(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                macros::COL_NORMAL(fighter);
            }
            wait(fighter.lua_state_agent, 1.0);
        }
    }
    frame(fighter.lua_state_agent, 120.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("bust"), 1.5, -2, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
}

//PSI Offense Up Sound
#[acmd_script( agent = "ness", script = "sound_specialshield", category = ACMD_SOUND)]
unsafe fn ssbuexo_ness_shield_special_sound(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_special_n05"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_ness_special_n02"));
    }
    frame(fighter.lua_state_agent, 107.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_ness_special_n02"));
    }
    frame(fighter.lua_state_agent, 118.0);
    if OFFENSE_UP_ACTIVE[entry_id] == true
    && macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_ness_special_n04_ll"));
    }
    else {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_ness_special_n04_l"));
    }
    frame(fighter.lua_state_agent, 140.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ness_landing02"));
    }
}

//PSI Offense Up Expression
#[acmd_script( agent = "ness", script = "game_specialshield", category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_ness_shield_special_expression(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattack"), 0, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 107.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 115.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(fighter.lua_state_agent, 121.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackl"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 126.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
}

//PK Flash ACMD
#[acmd_script( agent = "ness_pkflash", script = "game_bang", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_pk_flash_acmd(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let rand_num = sv_math::rand(hash40("fighter"), 100);
    if PK_FLASH_TIMER[entry_id] >= 120 {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            if (60..100).contains(&rand_num) {
                if macros::is_excute(fighter) {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                }
            }
            else if (25..59).contains(&rand_num) {
                if macros::is_excute(fighter) {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x1C655B0AE7), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);   
                }
            }
            else if (14..24).contains(&rand_num) {
                if macros::is_excute(fighter) {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                }
            }
            else if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                AttackModule::set_poison_param(fighter.module_accessor, 0, 600, 20, 0.2, false);  
            }
        }
        else {
            if (53..100).contains(&rand_num) {
                if macros::is_excute(fighter) {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                }
            }
            else if (18..52).contains(&rand_num) {
                if macros::is_excute(fighter) {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x1C655B0AE7), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);   
                }
            }
            else if (3..17).contains(&rand_num) {
                if macros::is_excute(fighter) {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                    AttackModule::set_poison_param(fighter.module_accessor, 0, 600, 20, 0.2, false);  
                }
            }
            else if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            }
        }
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x27936db96d));
    }
}

//PK Fire Shoot ACMD
#[acmd_script( agent = "ness_pkfire", scripts = ["game_shoot", "game_shootair"], category = ACMD_GAME)]
unsafe fn ssbuexo_ness_pk_fire_shoot_acmd(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 150, 80, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::enable_safe_pos(fighter.module_accessor);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 80, 80, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::enable_safe_pos(fighter.module_accessor);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        }
    }
}

//Grounded PK Fire Pillar ACMD
#[acmd_script( agent = "ness_pkfire", script = "game_pillar", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_grounded_pk_fire_pillar_acmd(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            AttackModule::clear_all(fighter.module_accessor);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 68, 30, 0, 10, 6.5, 0.0, 3.1, 2.0, None, None, None, 0.5, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 68, 30, 0, 10, 4.5, 0.0, 9.6, 2.0, None, None, None, 0.5, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            fighter.clear_lua_stack();
            lua_args!(fighter, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
            smash::app::sv_animcmd::AREA_WIND_2ND_RAD_arg9(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
        }
    }
}

//Aerial PK Fire Pillar ACMD
#[acmd_script( agent = "ness_pkfire", script = "game_pillarair", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_aerial_pk_fire_pillar_acmd(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.9, 150, 80, 0, 30, 6.5, 0.0, 3.1, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.9, 150, 80, 0, 30, 4.5, 0.0, 9.6, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            fighter.clear_lua_stack();
            lua_args!(fighter, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
            smash::app::sv_animcmd::AREA_WIND_2ND_RAD_arg9(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 58, 45, 0, 16, 6.5, 0.0, 3.1, 2.0, None, None, None, 0.5, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 58, 45, 0, 16, 4.5, 0.0, 9.6, 2.0, None, None, None, 0.5, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);  
            fighter.clear_lua_stack();
            lua_args!(fighter, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
            smash::app::sv_animcmd::AREA_WIND_2ND_RAD_arg9(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
        }
    }
}

//PK Thunderball ACMD
#[acmd_script( agent = "ness_pkthunder", script = "game_move", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_pk_thunder_ball_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 362, 50, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
}

//PK Thunderball Effect
#[acmd_script( agent = "ness_pkthunder", script = "effect_move", category = ACMD_EFFECT)]
unsafe fn ssbuexo_ness_pk_thunder_ball_effect(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ness_pkt_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, true);
    }
}

//PK Thunder Tackle ACMD
#[acmd_script( agent = "ness", script = "game_specialairhi", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_pk_thunder_tackle_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        DamageModule::add_damage(fighter.module_accessor, 3.0, 0);
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_NESS_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 25.0, 361, 80, 0, 83, 7.0, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        JostleModule::set_status(fighter.module_accessor, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 17.0, 361, 70, 0, 45, 4.8, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_ATTACK_FALL_START);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
        JostleModule::set_status(fighter.module_accessor, true);
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_NESS_CLIFF_HANG_DATA_DEFAULT as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_ness_shield_special_acmd,
        ssbuexo_ness_shield_special_effect,
        ssbuexo_ness_shield_special_sound,
        ssbuexo_ness_shield_special_expression,
        ssbuexo_ness_pk_flash_acmd,
        ssbuexo_ness_pk_fire_shoot_acmd,
        ssbuexo_ness_grounded_pk_fire_pillar_acmd,
        ssbuexo_ness_aerial_pk_fire_pillar_acmd,
        ssbuexo_ness_pk_thunder_ball_acmd,
        ssbuexo_ness_pk_thunder_ball_effect,
        ssbuexo_ness_pk_thunder_tackle_acmd
    );
}