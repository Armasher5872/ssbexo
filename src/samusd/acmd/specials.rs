use super::*;

//Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_samusd_neutral_special_start_acmd(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
        if HAS_FIRE_CHARGE_SHOT[entry_id] == false
        && CHARGE_SHOT_TIMER[entry_id] <= 0 {
            MotionModule::set_rate(agent.module_accessor, 0.6);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if HAS_FIRE_CHARGE_SHOT[entry_id] == false
        && CHARGE_SHOT_TIMER[entry_id] <= 0 {
            MotionModule::set_rate(agent.module_accessor, 0.6);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        if HAS_FIRE_CHARGE_SHOT[entry_id] == true
        && CHARGE_SHOT_TIMER[entry_id] > 0 {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
            MotionModule::change_motion(agent.module_accessor, Hash40::new("damage_n_2"), 1.0, 1.0, false, 0.0, false, false);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if HAS_FIRE_CHARGE_SHOT[entry_id] == true
        && CHARGE_SHOT_TIMER[entry_id] > 0 {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
            MotionModule::change_motion(agent.module_accessor, Hash40::new("damage_n_2"), 1.0, 1.0, false, 0.0, false, false);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, false, 0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_BULLET_DISP);
    }
}

//Neutral Special Fire ACMD
unsafe extern "C" fn ssbexo_samusd_neutral_special_fire_acmd(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
        if HAS_FIRE_CHARGE_SHOT[entry_id] == false
        && CHARGE_SHOT_TIMER[entry_id] <= 0 {
            HAS_FIRE_CHARGE_SHOT[entry_id] = true;
            CHARGE_SHOT_TIMER[entry_id] = 180;
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
        if HAS_FIRE_CHARGE_SHOT[entry_id] == false
        && CHARGE_SHOT_TIMER[entry_id] <= 0 {
            HAS_FIRE_CHARGE_SHOT[entry_id] = true;
            CHARGE_SHOT_TIMER[entry_id] = 180;
        }
    }
}

//Charge Shot ACMD
unsafe extern "C" fn ssbexo_samusd_charge_shot_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.1, 140, 42, 0, 14, 6.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.1, 140, 50, 0, 46, 1.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 1, Hash40::new("top"), 2.5, 140, 42, 0, 14, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 3, 1, Hash40::new("top"), 20.0, 140, 50, 0, 46, 1.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(agent.module_accessor, 0, 600, 60, 0.2, false);
        AttackModule::set_poison_param(agent.module_accessor, 1, 600, 60, 0.2, false);
        AttackModule::set_paralyze_frame(agent.module_accessor, 2, 25, false);
        AttackModule::set_attack_level(agent.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        attack!(agent, *MA_MSC_CMD_ATTACK_SET_LERP, 2, 3);
    }
}

//Missile ACMD
unsafe extern "C" fn ssbexo_samusd_missile_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 0, 25, 0, 26, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
        AttackModule::set_poison_param(agent.module_accessor, 0, 600, 60, 0.2, false);
    }
}

//Missile Effect 
unsafe extern "C" fn ssbexo_samusd_missile_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samusd_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, false);
    }
}

//Super Missile Initial ACMD
unsafe extern "C" fn ssbexo_samusd_super_missile_initial_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 0, 25, 0, 26, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
        AttackModule::set_poison_param(agent.module_accessor, 0, 600, 60, 0.2, false);
    }
}

//Super Missile Fire ACMD
unsafe extern "C" fn ssbexo_samusd_super_missile_fire_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 0, 25, 0, 26, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(agent.module_accessor, 0, 600, 60, 0.2, false);
    }
}

//Super Missile Effect 
unsafe extern "C" fn ssbexo_samusd_super_missile_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samusd_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Special ACMD
unsafe extern "C" fn ssbexo_samusd_up_special_acmd(_fighter: &mut L2CAgentBase) {}

//Up Special Effect
unsafe extern "C" fn ssbexo_samusd_up_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::BURN_COLOR(agent, 0.26, 0.71, 1.5, 0.7);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::BURN_COLOR(agent, 0.26, 0.71, 1.5, 0.7);
    }
    frame(agent.lua_state_agent, 94.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("samusd_win3_aura"), false, true);
        macros::BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
        macros::BURN_COLOR_NORMAL(agent);
    }
}


//Up Special Sound
unsafe extern "C" fn ssbexo_samusd_up_special_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samusd_appear01"));
    }
}

//Up Special Expression
unsafe extern "C" fn ssbexo_samusd_up_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Bomb Fall ACMD
unsafe extern "C" fn ssbexo_samusd_bomb_fall_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 45, 0, 22, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(agent.module_accessor, 0, 1200, 60, 0.2, false);
    }
}

//Bomb Detonate ACMD
unsafe extern "C" fn ssbexo_samusd_bomb_burst_attack_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 30, 0, 40, 7.38, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 1, Hash40::new("top"), 0.2, 0, 0, 0, 0, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(agent.module_accessor, 0, 1200, 60, 0.2, false);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 4.9);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 2.5);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Bomb Burst Attack Effect
unsafe extern "C" fn ssbexo_samusd_bomb_burst_attack_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samusd_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Bomb Burst Expression
unsafe extern "C" fn ssbexo_samusd_bomb_burst_attack_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 0.5, 0.02, 1000, 1, 0, 0, 16);
        sv_animcmd::AREA_WIND_2ND_RAD(agent.lua_state_agent);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

pub fn install() {
    Agent::new("samusd")
    .game_acmd("game_specialnstart", ssbexo_samusd_neutral_special_start_acmd)
    .game_acmd("game_specialairnstart", ssbexo_samusd_neutral_special_start_acmd)
    .game_acmd("game_specialnfire", ssbexo_samusd_neutral_special_fire_acmd)
    .game_acmd("game_specialairnfire", ssbexo_samusd_neutral_special_fire_acmd)
    .game_acmd("game_specialnfiremax", ssbexo_samusd_neutral_special_fire_acmd)
    .game_acmd("game_specialairnfiremax", ssbexo_samusd_neutral_special_fire_acmd)
    .game_acmd("game_specialhi", ssbexo_samusd_up_special_acmd)
    .game_acmd("game_specialairhi", ssbexo_samusd_up_special_acmd)
    .effect_acmd("effect_specialhi", ssbexo_samusd_up_special_effect)
    .effect_acmd("effect_specialairhi", ssbexo_samusd_up_special_effect)
    .sound_acmd("sound_specialhi", ssbexo_samusd_up_special_sound)
    .sound_acmd("sound_specialairhi", ssbexo_samusd_up_special_sound)
    .expression_acmd("expression_specialhi", ssbexo_samusd_up_special_expression)
    .expression_acmd("expression_specialairhi", ssbexo_samusd_up_special_expression)
    .install()
    ;
    Agent::new("samusd_cshot")
    .game_acmd("game_shoot", ssbexo_samusd_charge_shot_acmd)
    .install()
    ;
    Agent::new("samusd_missile")
    .game_acmd("game_homing", ssbexo_samusd_missile_acmd)
    .effect_acmd("effect_hburst", ssbexo_samusd_missile_effect)
    .install()
    ;
    Agent::new("samusd_supermissile")
    .game_acmd("game_ready", ssbexo_samusd_super_missile_initial_acmd)
    .game_acmd("game_straight", ssbexo_samusd_super_missile_fire_acmd)
    .effect_acmd("effect_sburst", ssbexo_samusd_super_missile_effect)
    .install()
    ;
    Agent::new("samusd_bomb")
    .game_acmd("game_fall", ssbexo_samusd_bomb_fall_acmd)
    .game_acmd("game_burstattack", ssbexo_samusd_bomb_burst_attack_acmd)
    .effect_acmd("effect_burstattack", ssbexo_samusd_bomb_burst_attack_effect)
    .expression_acmd("expression_burstattack", ssbexo_samusd_bomb_burst_attack_expression)
    .install()
    ;
}