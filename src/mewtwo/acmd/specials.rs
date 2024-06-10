use super::*;

//Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_mewtwo_neutral_special_start_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Shoot ACMD
unsafe extern "C" fn ssbexo_mewtwo_neutral_special_shoot_acmd(agent: &mut L2CAgentBase) {
    let damage_multiplier = WorkModule::get_float(agent.module_accessor, FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLOAT_PSYCHIC_GLARE_POWER);
    if macros::is_excute(agent) {
        AttackModule::set_power_up(agent.module_accessor, damage_multiplier);
        AttackModule::set_reaction_mul(agent.module_accessor, damage_multiplier/2.0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 75, 0, 45, 6.0, 0.0, 11.0, 2.0, Some(0.0), Some(11.0), Some(22.0*damage_multiplier), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::set_float(agent.module_accessor, 1.0, FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLOAT_PSYCHIC_GLARE_POWER);
        AttackModule::set_power_up(agent.module_accessor, 1.0);
        AttackModule::set_reaction_mul(agent.module_accessor, 1.0);
    }
}

//Neutral Special Shoot Effect
unsafe extern "C" fn ssbexo_mewtwo_neutral_special_shoot_effect(agent: &mut L2CAgentBase) {
    let damage_multiplier = WorkModule::get_float(agent.module_accessor, FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLOAT_PSYCHIC_GLARE_POWER);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.55);
        macros::EFFECT_FOLLOW(agent, Hash40::new("robot_robobeam_shot"), Hash40::new("head"), 0.7, 1.9, -1.4, 0, 0, 0, 0.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.35, 0.5, 1, 0.4);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 5, 0, 0, 0.35, 0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.55);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.35, 0.5, 1, 0.4);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 5, 0, 0, 0.35, 0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.55);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.35, 0.5, 1, 0.4);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 5, 0, 0, 0.35, 0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari_eye"), Hash40::new("head"), 0.7, 1.9, 1.4, 0, 0, 0, 0.5, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.0, 14.0, 3.0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("robot_robobeam_s"), Hash40::new("top"), 0.0, 14.0, 24.0*damage_multiplier, 0, 0, 0, 0.55*damage_multiplier, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("robot_robobeam_s"), false, false);
    }
}

//Neutral Special Shoot Sound
unsafe extern "C" fn ssbexo_mewtwo_neutral_special_shoot_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mewtwo_special_l01"));
    }
}

//Neutral Special Shoot Expression
unsafe extern "C" fn ssbexo_mewtwo_neutral_special_shoot_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        agent.clear_lua_stack();
        lua_args!(agent, 0, 0.5, 220, 3, 0.2, -10, 5, 20, 10, 80);
        sv_animcmd::AREA_WIND_2ND_arg10(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

//Up Special ACMD
unsafe extern "C" fn ssbexo_mewtwo_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        StatusModule::set_situation_kind(agent.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
    }
}

//Up Special Effect
unsafe extern "C" fn ssbexo_mewtwo_up_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        EffectModule::req_follow(agent.module_accessor, Hash40::new("mewtwo_final_aura"), Hash40::new("hip"), &NONE_VECTOR, &NONE_VECTOR, 1.25, true, 0, 0, 0, 0, 0, false, false);
    }
    frame(agent.lua_state_agent, 67.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("mewtwo_final_aura"), false, true);
    }
}

//Up Special Sound
unsafe extern "C" fn ssbexo_mewtwo_up_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_mewtwo_appeal_h01"));
        macros::PLAY_STATUS(agent, Hash40::new("vc_mewtwo_appeal01"));
    }
}

//Up Special Expression
unsafe extern "C" fn ssbexo_mewtwo_up_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Future Sight Ball ACMD
unsafe extern "C" fn ssbexo_mewtwo_futuresight_ball_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 300.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 40.0, 361, 45, 0, 25, 20.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 311.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Future Sight Ball Effect
unsafe extern "C" fn ssbexo_mewtwo_futuresight_ball_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_sign"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_metamon_aura"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_flash"), false, false);
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.15, true);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
        macros::EFFECT(agent, Hash40::new("sys_metamon_aura"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, true);
    }
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.45, true);
    }
    frame(agent.lua_state_agent, 120.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
        macros::EFFECT(agent, Hash40::new("sys_metamon_aura"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, true);
    }
    frame(agent.lua_state_agent, 150.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(agent.lua_state_agent, 180.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
        macros::EFFECT(agent, Hash40::new("sys_metamon_aura"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, true);
    }
    frame(agent.lua_state_agent, 210.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, true);
    }
    frame(agent.lua_state_agent, 240.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
        macros::EFFECT(agent, Hash40::new("sys_metamon_aura"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
        macros::EFFECT(agent, Hash40::new("sys_explosion_sign"), Hash40::new("top"), 0.0, 0.0, 0.5, 0, 0, 0, 0.85, 5, 5, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(agent.lua_state_agent, 260.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_sign"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_metamon_aura"), false, false);
    }
    frame(agent.lua_state_agent, 270.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_explosion_flash"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 1.0, 5, 5, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
    }
    frame(agent.lua_state_agent, 300.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_flash"), false, false);
        macros::EFFECT(agent, Hash40::new("sys_hit_purple"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_dead_dark"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.85);
    }
}

//Future Sight Ball Sound
unsafe extern "C" fn ssbexo_mewtwo_futuresight_ball_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 120.0);
    if macros::is_excute(agent) {
        let vortex = SoundModule::play_se(agent.module_accessor, Hash40::new("se_mewtwo_special_n01"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, vortex as i32, 0.4, 0);
    }
    frame(agent.lua_state_agent, 260.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_mewtwo_special_n01"));
    }
    frame(agent.lua_state_agent, 270.0);
    if macros::is_excute(agent) {
        let flash = SoundModule::play_se(agent.module_accessor, Hash40::new("se_mewtwo_final04"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, flash as i32, 1.5, 0);
    }
    frame(agent.lua_state_agent, 300.0);
    if macros::is_excute(agent) {
        let bomb = SoundModule::play_se(agent.module_accessor, Hash40::new("se_common_bomb_ll"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, bomb as i32, 2.0, 0);
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .game_acmd("game_specialnstart", ssbexo_mewtwo_neutral_special_start_acmd)
    .game_acmd("game_specialairnstart", ssbexo_mewtwo_neutral_special_start_acmd)
    .game_acmd("game_specialnshoot", ssbexo_mewtwo_neutral_special_shoot_acmd)
    .game_acmd("game_specialairnshoot", ssbexo_mewtwo_neutral_special_shoot_acmd)
    .effect_acmd("effect_specialnshoot", ssbexo_mewtwo_neutral_special_shoot_effect)
    .effect_acmd("effect_specialairnshoot", ssbexo_mewtwo_neutral_special_shoot_effect)
    .sound_acmd("sound_specialnshoot", ssbexo_mewtwo_neutral_special_shoot_sound)
    .sound_acmd("sound_specialairnshoot", ssbexo_mewtwo_neutral_special_shoot_sound)
    .expression_acmd("expression_specialnshoot", ssbexo_mewtwo_neutral_special_shoot_expression)
    .expression_acmd("expression_specialairnshoot", ssbexo_mewtwo_neutral_special_shoot_expression)
    .game_acmd("game_specialhistart", ssbexo_mewtwo_up_special_acmd)
    .game_acmd("game_specialairhistart", ssbexo_mewtwo_up_special_acmd)
    .effect_acmd("effect_specialhistart", ssbexo_mewtwo_up_special_effect)
    .effect_acmd("effect_specialairhistart", ssbexo_mewtwo_up_special_effect)
    .sound_acmd("sound_specialhistart", ssbexo_mewtwo_up_special_sound)
    .sound_acmd("sound_specialairhistart", ssbexo_mewtwo_up_special_sound)
    .expression_acmd("expression_specialhistart", ssbexo_mewtwo_up_special_expression)
    .expression_acmd("expression_specialairhistart", ssbexo_mewtwo_up_special_expression)
    .install()
    ;
    Agent::new("mewtwo_bindball")
    .game_acmd("game_shoot", ssbexo_mewtwo_futuresight_ball_acmd)
    .effect_acmd("effect_shoot", ssbexo_mewtwo_futuresight_ball_effect)
    .sound_acmd("sound_shoot", ssbexo_mewtwo_futuresight_ball_sound)
    .install()
    ;
}