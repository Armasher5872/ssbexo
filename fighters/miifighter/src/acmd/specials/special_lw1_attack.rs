use super::*;

//Armor Crushing Thunder Kick Attack ACMD
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_attack_acmd(agent: &mut L2CAgentBase) {
    let damage = WorkModule::get_float(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_DAMAGE);
    let attribute = WorkModule::get_int(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_ARMOR_CRUSHING_THUNDER_KICK_ATTRIBUTE);
    let collision_attr = match attribute {
        0 => Hash40::new("collision_attr_elec"),
        _ => Hash40::new("collision_attr_saving")
    };
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_ACTIVE_ARMOR);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_DASH_CANCEL);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), damage, 361, 40, 0, 40, 4.0, 6.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), damage, 361, 40, 0, 40, 3.0, -4.7, 0.0, 0.0, Some(2.2), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        match attribute {
            1 => {
                AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
                AttackModule::set_attack_level(agent.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_2 as u8);
            },
            2 => {
                AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
                AttackModule::set_attack_level(agent.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
            },
            _ => {}
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Armor Crushing Thunder Kick Attack Effect
unsafe extern "C" fn ssbexo_miifighter_grounded_armor_crushing_thunder_kick_attack_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_flash"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("toel"), 0, 0, 0, 0, 0, 60, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_dead_dark"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.15, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 3.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Armor Crushing Thunder Kick Attack Effect
unsafe extern "C" fn ssbexo_miifighter_aerial_armor_crushing_thunder_kick_attack_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_flash"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("toel"), 0, 0, 0, 0, 0, 60, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_dead_dark"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.15, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
    }
}

//Armor Crushing Thunder Kick Attack Sound
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_attack_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_special_c3_n02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_special_c3_n01"));
    }
}

//Armor Crushing Thunder Kick Attack Expression
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_acmd, Low)
    .game_acmd("game_specialairlw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_acmd, Low)
    .effect_acmd("effect_speciallw1attack", ssbexo_miifighter_grounded_armor_crushing_thunder_kick_attack_effect, Low)
    .effect_acmd("effect_specialairlw1attack", ssbexo_miifighter_aerial_armor_crushing_thunder_kick_attack_effect, Low)
    .sound_acmd("sound_speciallw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_sound, Low)
    .sound_acmd("sound_specialairlw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_sound, Low)
    .expression_acmd("expression_speciallw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_expression, Low)
    .expression_acmd("expression_specialairlw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_expression, Low)
    .install()
    ;
}