use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_koopa_neutral_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_STEP_START, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_STEP);
    }
}

//Grounded Neutral Special Effect
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 0.537, 0.537, 0.588);
        macros::FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Neutral Special Effect
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 0.537, 0.537, 0.588);
        macros::FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

//Grounded Neutral Special Sound
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_koopa_step_left_m"));
    }
}

//Aerial Neutral Special Sound
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
}

//Grounded Neutral Special Expression
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Neutral Special Expression
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Fire Breath Move ACMD
unsafe extern "C" fn ssbexo_koopa_firebreath_move_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

//Fire Breath Move Effect
unsafe extern "C" fn ssbexo_koopa_firebreath_move_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        wait(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
        }
        wait(agent.lua_state_agent, 15.0);
    }
}

//Fire Breath Move Sound
unsafe extern "C" fn ssbexo_koopa_firebreath_move_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_koopa_special_n02"));
    }
}

pub fn install() {
    Agent::new("koopa")
    .game_acmd("game_specialn", ssbexo_koopa_neutral_special_acmd, Priority::Low)
    .game_acmd("game_specialairn", ssbexo_koopa_neutral_special_acmd, Priority::Low)
    .effect_acmd("effect_specialn", ssbexo_koopa_grounded_neutral_special_effect, Priority::Low)
    .effect_acmd("effect_specialairn", ssbexo_koopa_aerial_neutral_special_effect, Priority::Low)
    .sound_acmd("sound_specialn", ssbexo_koopa_grounded_neutral_special_sound, Priority::Low)
    .sound_acmd("sound_specialairn", ssbexo_koopa_aerial_neutral_special_sound, Priority::Low)
    .expression_acmd("expression_specialn", ssbexo_koopa_grounded_neutral_special_expression, Priority::Low)
    .expression_acmd("expression_specialairn", ssbexo_koopa_aerial_neutral_special_expression, Priority::Low)
    .install()
    ;
    Agent::new("koopa_breath")
    .game_acmd("game_move", ssbexo_koopa_firebreath_move_acmd, Priority::Low)
    .effect_acmd("effect_move", ssbexo_koopa_firebreath_move_effect, Priority::Low)
    .sound_acmd("sound_move", ssbexo_koopa_firebreath_move_sound, Priority::Low)
    .install()
    ;
}