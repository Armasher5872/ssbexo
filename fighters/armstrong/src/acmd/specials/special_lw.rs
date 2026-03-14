use super::*;

//Grounded Down Special ACMD
unsafe extern "C" fn ssbexo_armstrong_grounded_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_COUNTER_ACTIVE);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        let damage_charge_multiplier = WorkModule::get_float(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
        SEARCH(agent, 0, 0, Hash40::new("top"), 13.5, 0.0, 7.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 13.5, 0.0, 7.0, 0.0, 0.0, 7.0, 0.0, damage_charge_multiplier, damage_charge_multiplier, 80, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.0);
        DamageModule::set_reaction_mul(agent.module_accessor, 0.85);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 100, 0, 60, 13.5, 6.0, 9.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_COUNTER_ACTIVE);
        AttackModule::clear_all(agent.module_accessor);
        armstrong_clear_charge(agent.module_accessor);
    }
}

//Grounded Down Special Effect
unsafe extern "C" fn ssbexo_armstrong_grounded_down_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("head"), 0.0, 0.0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("armstrong_ground_flame_crack"), Hash40::new("top"), -2.3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_blast_wave"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 0.3, true);
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_blast_shock"), Hash40::new("top"), 0, 11, -3, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.25);
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_blast_fire"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 2.0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Grounded Down Special Sound
unsafe extern "C" fn ssbexo_armstrong_grounded_down_special_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_h01"));
        PLAY_SE(agent, Hash40::new("vc_ganon_attackhard_h01"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("vc_ganon_attackhard_h01"));
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_h03"));
        PLAY_SE(agent, Hash40::new("vc_ganon_special_l01"));
    }
}

//Grounded Down Special Expression
unsafe extern "C" fn ssbexo_armstrong_grounded_down_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 12, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 2, 225, 2, 1, 0, 12, 30, 30, 80);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(agent, 0, 3, 90, 16, 1, 0, 12, 30, 30, 50);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

//Aerial Down Special ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_down_special_acmd(agent: &mut L2CAgentBase) {
    let damage_charge_multiplier = WorkModule::get_float(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 13.5, 0.0, 7.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 13.5, 0.0, 9.0, 6.0, 0.0, 7.0, 0.0, damage_charge_multiplier, damage_charge_multiplier, 80, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 100, 0, 60, 13.5, 6.0, 9.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        AttackModule::clear_all(agent.module_accessor);
        armstrong_clear_charge(agent.module_accessor);
    }
}

//Aerial Down Special Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_down_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("head"), 0.0, 0.0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_blast_wave"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 0.3, true);
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_blast_shock"), Hash40::new("top"), 0, 11, -3, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.25);
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_blast_fire"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 2.0, true);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Aerial Down Special Sound
unsafe extern "C" fn ssbexo_armstrong_aerial_down_special_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_h01"));
        PLAY_SE(agent, Hash40::new("vc_ganon_attackhard_h01"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("vc_ganon_attackhard_h01"));
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_h03"));
        PLAY_SE(agent, Hash40::new("vc_ganon_special_l01"));
    }
}

//Aerial Down Special Expression
unsafe extern "C" fn ssbexo_armstrong_aerial_down_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 12, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 2, 225, 2, 1, 0, 12, 30, 30, 80);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(agent, 0, 3, 90, 16, 1, 0, 12, 30, 30, 50);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .game_acmd("game_speciallw", ssbexo_armstrong_grounded_down_special_acmd, Low)
    .effect_acmd("effect_speciallw", ssbexo_armstrong_grounded_down_special_effect, Low)
    .sound_acmd("sound_speciallw", ssbexo_armstrong_grounded_down_special_sound, Low)
    .expression_acmd("expression_speciallw", ssbexo_armstrong_grounded_down_special_expression, Low)
    .game_acmd("game_specialairlw", ssbexo_armstrong_aerial_down_special_acmd, Low)
    .effect_acmd("effect_specialairlw", ssbexo_armstrong_aerial_down_special_effect, Low)
    .sound_acmd("sound_specialairlw", ssbexo_armstrong_aerial_down_special_sound, Low)
    .expression_acmd("expression_specialairlw", ssbexo_armstrong_aerial_down_special_expression, Low)
    .install()
    ;
}