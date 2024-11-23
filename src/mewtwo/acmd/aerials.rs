use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_mewtwo_nair_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 7.0);
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 367, 100, 60, 0, 11.0, 0.0, 7.0, -2.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 110, 0, 40, 13.5, 0.0, 7.0, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair Effect
unsafe extern "C" fn ssbexo_mewtwo_nair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_pk_attack_f"), Hash40::new("top"), 0, 9, -0.5, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 9, -0.5, 0, 0, 0, 2, true);
    }
    for _ in 0..10 {
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 0.7, 0.2, 1, 0.7);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 1, 0.7, 0.2, 1, 0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 0.7, 1, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH_FRM(agent, 1, 1, 0.7, 1, 0);
            macros::COL_NORMAL(agent);
        }
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

//Bair ACMD
unsafe extern "C" fn ssbexo_mewtwo_bair_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("s_tail3"), 12.0, 361, 90, 0, 30, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("s_tail5"), 12.0, 361, 90, 0, 30, 3.9, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("s_tail7"), 12.0, 361, 90, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Bair Effect
unsafe extern "C" fn ssbexo_mewtwo_bair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_2"), 4, Hash40::new("s_tail6"), 0.0, 0.0, 0.0, Hash40::new("s_tail7"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail7"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail5"), 0.0, 0.0, 0.0, Hash40::new("s_tail6"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail6"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail4"), 0.0, 0.0, 0.0, Hash40::new("s_tail5"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail5"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail3"), 0.0, 0.0, 0.0, Hash40::new("s_tail4"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail4"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail2"), 0.0, 0.0, 0.0, Hash40::new("s_tail3"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail3"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail1"), 0.0, 0.0, 0.0, Hash40::new("s_tail2"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
    }
}

//Bair Sound
unsafe extern "C" fn ssbexo_mewtwo_bair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mewtwo_attackair_b01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//Bair Expression
unsafe extern "C" fn ssbexo_mewtwo_bair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Uair ACMD
unsafe extern "C" fn ssbexo_mewtwo_uair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 3.0, 90, 110, 90, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 90, 110, 90, 0, 6.0, 0.0, 5.0, 12.0, Some(0.0), Some(5.0), Some(12.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 3.0, 90, 110, 90, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 90, 110, 90, 0, 6.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 10.0, 90, 70, 0, 65, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 90, 70, 0, 65, 6.0, 0.0, 11.0, 10.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Uair Effect
unsafe extern "C" fn ssbexo_mewtwo_uair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.65, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_tail_short"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.3, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_appeal_aura"), Hash40::new("top"), 0, -1.5, 0, 0, 0, 0, 0.8, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_tail_short"), false, false);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_dead_dark"), Hash40::new("throw"), 0.0, 0.0, 0, 0, 0, 0, 0.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_appeal_aura"), false, false);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_dead_dark"), false, false);
    }
}

//Uair Sound
unsafe extern "C" fn ssbexo_mewtwo_uair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_mewtwo_appeal_s01"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_mewtwo_rnd_attack"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        let bomb = SoundModule::play_se(agent.module_accessor, Hash40::new("se_common_bomb_m"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, bomb as i32, 1.5, 0);
    }
}

//Uair Expression
unsafe extern "C" fn ssbexo_mewtwo_uair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .game_acmd("game_attackairn", ssbexo_mewtwo_nair_acmd, Priority::Low)
    .effect_acmd("effect_attackairn", ssbexo_mewtwo_nair_effect, Priority::Low)
    .game_acmd("game_attackairb", ssbexo_mewtwo_bair_acmd, Priority::Low)
    .effect_acmd("effect_attackairb", ssbexo_mewtwo_bair_effect, Priority::Low)
    .sound_acmd("sound_attackairb", ssbexo_mewtwo_bair_sound, Priority::Low)
    .expression_acmd("expression_attackairb", ssbexo_mewtwo_bair_expression, Priority::Low)
    .game_acmd("game_attackairhi", ssbexo_mewtwo_uair_acmd, Priority::Low)
    .effect_acmd("effect_attackairhi", ssbexo_mewtwo_uair_effect, Priority::Low)
    .sound_acmd("sound_attackairhi", ssbexo_mewtwo_uair_sound, Priority::Low)
    .expression_acmd("expression_attackairhi", ssbexo_mewtwo_uair_expression, Priority::Low)
    .install()
    ;
}