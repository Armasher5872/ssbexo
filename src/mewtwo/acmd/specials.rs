use super::*;

//Shadowball Charge ACMD
unsafe extern "C" fn ssbexo_mewtwo_shadowball_charge_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 85, 90, 0, 90, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 3.0);
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
        EffectModule::req_follow(agent.module_accessor, Hash40::new("mewtwo_final_aura"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 1.25, true, 0, 0, 0, 0, 0, false, false);
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
    .game_acmd("game_specialhi", ssbexo_mewtwo_up_special_acmd, Priority::Low)
    .game_acmd("game_specialairhi", ssbexo_mewtwo_up_special_acmd, Priority::Low)
    .effect_acmd("effect_specialhi", ssbexo_mewtwo_up_special_effect, Priority::Low)
    .effect_acmd("effect_specialairhi", ssbexo_mewtwo_up_special_effect, Priority::Low)
    .sound_acmd("sound_specialhi", ssbexo_mewtwo_up_special_sound, Priority::Low)
    .sound_acmd("sound_specialairhi", ssbexo_mewtwo_up_special_sound, Priority::Low)
    .expression_acmd("expression_specialhi", ssbexo_mewtwo_up_special_expression, Priority::Low)
    .expression_acmd("expression_specialairhi", ssbexo_mewtwo_up_special_expression, Priority::Low)
    .install()
    ;
    Agent::new("mewtwo_shadowball")
    .game_acmd("game_charge", ssbexo_mewtwo_shadowball_charge_acmd, Priority::Low)
    .game_acmd("game_chargemax", ssbexo_mewtwo_shadowball_charge_acmd, Priority::Low)
    .install()
    ;
    Agent::new("mewtwo_bindball")
    .game_acmd("game_shoot", ssbexo_mewtwo_futuresight_ball_acmd, Priority::Low)
    .effect_acmd("effect_shoot", ssbexo_mewtwo_futuresight_ball_effect, Priority::Low)
    .sound_acmd("sound_shoot", ssbexo_mewtwo_futuresight_ball_sound, Priority::Low)
    .install()
    ;
}