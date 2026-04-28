use super::*;

//Grounded Neutral Special Attack ACMD
unsafe extern "C" fn ssbexo_armstrong_grounded_neutral_special_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_ARMSTRONG_GENERATE_ARTICLE_FIREPILLAR, false, -1);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        let charge = WorkModule::get_float(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_NEUTRAL_SPECIAL_CHARGE);
        let flame_pillar_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_FLAME_PILLAR_ID);
        let is_active = sv_battle_object::is_active(flame_pillar_id as u32);
        let damage = 10.0+(30.0*charge);
        if is_active {
            let firepillar_boma = sv_battle_object::module_accessor(flame_pillar_id as u32);
            WorkModule::set_float(firepillar_boma, 16.0, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_X);
            WorkModule::set_float(firepillar_boma, 0.0, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_Y);
            WorkModule::set_float(firepillar_boma, charge, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_CHARGE);
        }
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), damage, 361, 46, 0, 80, 5.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("bust"), damage, 361, 46, 0, 80, 5.8, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), damage, 361, 46, 0, 80, 5.8, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 40.0, 361, 46, 0, 80, 5.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("bust"), 40.0, 361, 46, 0, 80, 5.8, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("arml"), 40.0, 361, 46, 0, 80, 5.8, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Aerial Neutral Special Attack ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_neutral_special_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        let charge = WorkModule::get_float(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_NEUTRAL_SPECIAL_CHARGE);
        let damage = 10.0+(30.0*charge);
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), damage, 361, 46, 0, 80, 5.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("bust"), damage, 361, 46, 0, 80, 5.8, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), damage, 361, 46, 0, 80, 5.8, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 40.0, 361, 46, 0, 80, 5.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("bust"), 40.0, 361, 46, 0, 80, 5.8, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("arml"), 40.0, 361, 46, 0, 80, 5.8, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        }
    }
}

//Neutral Special Attack Effect
unsafe extern "C" fn ssbexo_armstrong_neutral_special_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 12, 3, 0, -40, -90, 1.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_flame"), Hash40::new("havel"), 0, 0.0, 0.0, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
}

//Neutral Special Attack Sound
unsafe extern "C" fn ssbexo_armstrong_neutral_special_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

//Grounded Neutral Special Attack Expression
unsafe extern "C" fn ssbexo_armstrong_grounded_neutral_special_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 3, 120, 2, 1, 0, 12, 50, 30, 0);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

//Aerial Neutral Special Attack Expression
unsafe extern "C" fn ssbexo_armstrong_aerial_neutral_special_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
}

pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    Agent::new("ganon")
    .set_costume(costume.to_vec())
    .game_acmd("game_specialnattack", ssbexo_armstrong_grounded_neutral_special_attack_acmd, Low)
    .effect_acmd("effect_specialnattack", ssbexo_armstrong_neutral_special_attack_effect, Low)
    .sound_acmd("sound_specialnattack", ssbexo_armstrong_neutral_special_attack_sound, Low)
    .expression_acmd("expression_specialnattack", ssbexo_armstrong_grounded_neutral_special_attack_expression, Low)
    .game_acmd("game_specialairnattack", ssbexo_armstrong_aerial_neutral_special_attack_acmd, Low)
    .effect_acmd("effect_specialairnattack", ssbexo_armstrong_neutral_special_attack_effect, Low)
    .sound_acmd("sound_specialairnattack", ssbexo_armstrong_neutral_special_attack_sound, Low)
    .expression_acmd("expression_specialairnattack", ssbexo_armstrong_aerial_neutral_special_attack_expression, Low)
    .install()
    ;
}