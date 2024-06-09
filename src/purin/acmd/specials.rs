use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_purin_neutral_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_PURIN_GENERATE_ARTICLE_DISARMING_VOICE, false, -1);
    }
}

//Neutral Special Effect
unsafe extern "C" fn ssbexo_purin_neutral_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Neutral Special Sound
unsafe extern "C" fn ssbexo_purin_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("vc_purin_003"));
    }
}

//Neutral Special Expression
unsafe extern "C" fn ssbexo_purin_neutral_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_beamm"), 0);
    }
}

//Disarming Voice ACMD
unsafe extern "C" fn ssbexo_purin_disarming_voice_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 20, 0, 40, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
}

//Disarming Voice Effect
unsafe extern "C" fn ssbexo_purin_disarming_voice_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("poke_meloetta_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1000, true);
    }
    for _ in 0..15 {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("rosetta_ring_erase"), false, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 0, 0, 0, 0, 25, 1000, false);
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
        }
        wait(agent.lua_state_agent, 4.0);
    }
}

//Grounded Side Special ACMD
unsafe extern "C" fn ssbexo_purin_grounded_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 75, 75, 0, 52, 4.5, 0.0, 4.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 75, 75, 0, 52, 4.5, 0.0, 4.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 1.5);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Side Special ACMD
unsafe extern "C" fn ssbexo_purin_aerial_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 75, 75, 0, 52, 4.5, 0.0, 4.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 75, 75, 0, 52, 4.5, 0.0, 4.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 1.5);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_INPUT);
        WorkModule::set_int(agent.module_accessor, *FIGHTER_PURIN_SPECIAL_S_ENERGY_MODE_BRAKE, *FIGHTER_PURIN_STATUS_SPECIAL_S_WORK_INT_ENERGY_MODE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_CHANGE_ENERGY);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_PURIN_SPECIAL_S_ENERGY_MODE_FALL, *FIGHTER_PURIN_STATUS_SPECIAL_S_WORK_INT_ENERGY_MODE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_CHANGE_ENERGY);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Up Special ACMD
unsafe extern "C" fn ssbexo_purin_up_special_acmd(agent: &mut L2CAgentBase) {
    let mut size = 0.0;
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 0.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..5 {
        if macros::is_excute(agent) {
            size += 0.2;
            AttackModule::set_size(agent.module_accessor, 0, size);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 8.0);
    for _ in 0..5 {
        if macros::is_excute(agent) {
            size += 0.4;
            AttackModule::set_size(agent.module_accessor, 0, size);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 13.0);
    for _ in 0..9 {
        if macros::is_excute(agent) {
            size += 0.6;
            AttackModule::set_size(agent.module_accessor, 0, size);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 16.0);
        AttackModule::set_size(agent.module_accessor, 1, 16.0);
    }
    frame(agent.lua_state_agent, 116.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("purin")
    .game_acmd("game_specialnstartr", ssbexo_purin_neutral_special_acmd)
    .game_acmd("game_specialairnstartr", ssbexo_purin_neutral_special_acmd)
    .effect_acmd("effect_specialnstartr", ssbexo_purin_neutral_special_effect)
    .effect_acmd("effect_specialairnstartr", ssbexo_purin_neutral_special_effect)
    .sound_acmd("sound_specialnstartr", ssbexo_purin_neutral_special_sound)
    .sound_acmd("sound_specialairnstartr", ssbexo_purin_neutral_special_sound)
    .expression_acmd("expression_specialnstartr", ssbexo_purin_neutral_special_expression)
    .expression_acmd("expression_specialairnstartr", ssbexo_purin_neutral_special_expression)
    .game_acmd("game_specials", ssbexo_purin_grounded_side_special_acmd)
    .game_acmd("game_specialairs", ssbexo_purin_aerial_side_special_acmd)
    .game_acmd("game_specialhir", ssbexo_purin_up_special_acmd)
    .game_acmd("game_specialhil", ssbexo_purin_up_special_acmd)
    .game_acmd("game_specialairhir", ssbexo_purin_up_special_acmd)
    .game_acmd("game_specialairhil", ssbexo_purin_up_special_acmd)
    .install()
    ;
    Agent::new("purin_disarmingvoice")
    .game_acmd("game_shoot", ssbexo_purin_disarming_voice_acmd)
    .effect_acmd("effect_shoot", ssbexo_purin_disarming_voice_effect)
    .install()
    ;
}