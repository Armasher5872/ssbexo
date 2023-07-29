use super::*;

//Neutral Special Start ACMD
#[acmd_script( agent = "samusd", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME)]
unsafe fn ssbuexo_samusd_neutral_special_start_acmd(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if HAS_FIRE_CHARGE_SHOT[entry_id] == false
        && CHARGE_SHOT_TIMER[entry_id] <= 0 {
            MotionModule::set_rate(fighter.module_accessor, 0.6);
        }
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if HAS_FIRE_CHARGE_SHOT[entry_id] == false
        && CHARGE_SHOT_TIMER[entry_id] <= 0 {
            MotionModule::set_rate(fighter.module_accessor, 0.6);
        }
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        if HAS_FIRE_CHARGE_SHOT[entry_id] == true
        && CHARGE_SHOT_TIMER[entry_id] > 0 {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("damage_n_2"), 1.0, 1.0, false, 0.0, false, false);
        }
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if HAS_FIRE_CHARGE_SHOT[entry_id] == true
        && CHARGE_SHOT_TIMER[entry_id] > 0 {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("damage_n_2"), 1.0, 1.0, false, 0.0, false, false);
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
unsafe fn ssbuexo_samusd_neutral_special_fire_acmd(fighter: &mut L2CAgentBase) 
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
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
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
unsafe fn ssbuexo_samusd_charge_shot_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 140, 42, 0, 14, 6.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.1, 140, 50, 0, 46, 1.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 2.5, 140, 42, 0, 14, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 20.0, 140, 50, 0, 46, 1.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 600, 60, 0.2, false);
        AttackModule::set_poison_param(fighter.module_accessor, 1, 600, 60, 0.2, false);
        AttackModule::set_paralyze_frame(fighter.module_accessor, 2, 25, false);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 2, 3);
    }
}

//Missile ACMD
#[acmd_script( agent = "samusd_missile", script = "game_homing", category = ACMD_GAME)]
unsafe fn ssbuexo_samusd_missile_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 0, 25, 0, 26, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 600, 60, 0.2, false);
    }
}

//Missile Effect 
#[acmd_script( agent = "samusd_missile", script = "effect_hburst", category = ACMD_EFFECT)]
unsafe fn ssbuexo_samusd_missile_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, false);
    }
}

//Super Missile Initial ACMD
#[acmd_script( agent = "samusd_supermissile", script = "game_ready", category = ACMD_GAME)]
unsafe fn ssbuexo_samusd_super_missile_initial_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 0, 25, 0, 26, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 600, 60, 0.2, false);
    }
}

//Super Missile Fire ACMD
#[acmd_script( agent = "samusd_supermissile", script = "game_straight", category = ACMD_GAME)]
unsafe fn ssbuexo_samusd_super_missile_fire_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 0, 25, 0, 26, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 600, 60, 0.2, false);
    }
}

//Super Missile Effect 
#[acmd_script( agent = "samusd_supermissile", script = "effect_sburst", category = ACMD_EFFECT)]
unsafe fn ssbuexo_samusd_super_missile_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Special ACMD
#[acmd_script( agent = "samusd", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME)]
unsafe fn ssbuexo_samusd_up_special_acmd(_fighter: &mut L2CAgentBase) {}

//Up Special Effect
#[acmd_script( agent = "samusd", scripts = ["effect_specialhi", "effect_specialairhi"], category = ACMD_EFFECT)]
unsafe fn ssbuexo_samusd_up_special_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    frame(fighter.lua_state_agent, 94.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        macros::BURN_COLOR_NORMAL(fighter);
    }
}


//Up Special Sound
#[acmd_script( agent = "samusd", scripts = ["sound_specialhi", "sound_specialairhi"], category = ACMD_SOUND)]
unsafe fn ssbuexo_samusd_up_special_sound(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_appear01"));
    }
}

//Up Special Expression
#[acmd_script( agent = "samusd", script = "expression_specialairhi", category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_samusd_up_special_expression(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Bomb Fall ACMD
#[acmd_script( agent = "samusd_bomb", script = "game_fall", category = ACMD_GAME)]
unsafe fn ssbuexo_samusd_bomb_fall_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 45, 0, 22, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 1200, 60, 0.2, false);
    }
}

//Bomb Detonate ACMD
#[acmd_script( agent = "samusd_bomb", script = "game_burstattack", category = ACMD_GAME)]
unsafe fn ssbuexo_samusd_bomb_detonate_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 30, 0, 40, 7.38, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 0.2, 0, 0, 0, 0, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 1200, 60, 0.2, false);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor, 0, 4.9);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor, 0, 2.5);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Bomb Burst Attack Effect
#[acmd_script( agent = "samusd_bomb", script = "effect_burstattack", category = ACMD_EFFECT)]
unsafe fn ssbuexo_samusd_bomb_burst_attack_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Bomb Burst Expression
#[acmd_script( agent = "samusd_bomb", script = "expression_burstattack", category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_samusd_bomb_burst_attack_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, 0, 0.5, 0.02, 1000, 1, 0, 0, 16);
        sv_animcmd::AREA_WIND_2ND_RAD(fighter.lua_state_agent);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AreaModule::erase_wind(fighter.module_accessor, 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_samusd_neutral_special_start_acmd,
        ssbuexo_samusd_neutral_special_fire_acmd,
        ssbuexo_samusd_charge_shot_acmd,
        ssbuexo_samusd_missile_acmd,
        ssbuexo_samusd_missile_effect,
        ssbuexo_samusd_super_missile_initial_acmd,
        ssbuexo_samusd_super_missile_fire_acmd,
        ssbuexo_samusd_super_missile_effect,
        ssbuexo_samusd_up_special_acmd,
        ssbuexo_samusd_up_special_effect,
        ssbuexo_samusd_up_special_sound,
        ssbuexo_samusd_up_special_expression,
        ssbuexo_samusd_bomb_fall_acmd,
        ssbuexo_samusd_bomb_detonate_acmd,
        ssbuexo_samusd_bomb_burst_attack_effect,
        ssbuexo_samusd_bomb_burst_attack_expression
    );
}