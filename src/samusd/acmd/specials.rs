#![allow(unused_macros)]
use {
    crate::functions::{
        CHARGE_SHOT_TIMER,
        HAS_FIRE_CHARGE_SHOT,
        SAMUSD_HAS_FLOAT
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::L2CAgentBase,
        phx::Hash40,
    },
    smash_script::*,
    smashline::*,
};

//Neutral Special Start ACMD
#[acmd_script( agent = "samusd", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME)]
unsafe fn ssbuexo_dark_samus_neutral_special_start_acmd(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if HAS_FIRE_CHARGE_SHOT[entry_id] == true
        && CHARGE_SHOT_TIMER[entry_id] > 0 {
            if PostureModule::lr(fighter.module_accessor) == 1.0 {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_l"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        else if HAS_FIRE_CHARGE_SHOT[entry_id] == false
        && CHARGE_SHOT_TIMER[entry_id] <= 0 {
            MotionModule::set_rate(fighter.module_accessor, 0.6);
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, false, 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_BULLET_DISP);
    }
}

//Neutral Special Fire ACMD
#[acmd_script( agent = "samusd", scripts = ["game_specialnfire", "game_specialairnfire", "game_specialnfiremax", "game_specialairnfiremax"], category = ACMD_GAME)]
unsafe fn ssbuexo_dark_samus_neutral_special_fire_acmd(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
        if HAS_FIRE_CHARGE_SHOT[entry_id] == false
        && CHARGE_SHOT_TIMER[entry_id] <= 0 {
            HAS_FIRE_CHARGE_SHOT[entry_id] = true;
            CHARGE_SHOT_TIMER[entry_id] = 180;
        }
    }
}

//Charge Shot ACMD
#[acmd_script( agent = "samusd_cshot", script = "game_shoot", category = ACMD_GAME)]
unsafe fn ssbuexo_dark_samus_charge_shot_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 140, 42, 0, 14, 6.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.1, 40, 50, 0, 46, 1.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 2.5, 140, 42, 0, 14, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 20.0, 40, 50, 0, 46, 1.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 60, 5, 0.2, false);
        AttackModule::set_poison_param(fighter.module_accessor, 1, 300, 6, 0.2, false);
        AttackModule::set_paralyze_frame(fighter.module_accessor, 2, 25, false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 2, 3);
    }
}

//Missile ACMD
#[acmd_script( agent = "samusd_missile", script = "game_homing", category = ACMD_GAME)]
unsafe fn ssbuexo_dark_samus_missile_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 0, 25, 0, 26, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 60, 5, 0.2, false);
    }
}

//Super Missile Initial ACMD
#[acmd_script( agent = "samusd_supermissile", script = "game_ready", category = ACMD_GAME)]
unsafe fn ssbuexo_dark_samus_super_missile_initial_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 0, 25, 0, 26, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 60, 5, 0.2, false);
    }
}

//Super Missile Fire ACMD
#[acmd_script( agent = "samusd_supermissile", script = "game_straight", category = ACMD_GAME)]
unsafe fn ssbuexo_dark_samus_super_missile_fire_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 0, 25, 0, 26, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 60, 5, 0.2, false);
    }
}

//Float ACMD
#[acmd_script( agent = "samusd", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME)]
unsafe fn ssbuexo_dark_samus_float_acmd(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SAMUSD_HAS_FLOAT[entry_id] == true {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r"), 1.0, 1.0, false, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_l"), 1.0, 1.0, false, 0.0, false, false);
        }
    }
}

//Float Expression
#[acmd_script( agent = "samusd", scripts = ["expression_speciallw", "expression_specialairlw"], category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_dark_samus_float_expression(_fighter: &mut L2CAgentBase) {
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_dark_samus_neutral_special_start_acmd,
        ssbuexo_dark_samus_neutral_special_fire_acmd,
        ssbuexo_dark_samus_charge_shot_acmd,
        ssbuexo_dark_samus_missile_acmd,
        ssbuexo_dark_samus_super_missile_initial_acmd,
        ssbuexo_dark_samus_super_missile_fire_acmd,
        ssbuexo_dark_samus_float_acmd,
        ssbuexo_dark_samus_float_expression
    );
}