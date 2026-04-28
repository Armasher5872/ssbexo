use super::*;

unsafe extern "C" fn ssbexo_captain_grounded_neutral_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
    }
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        let is_stored = WorkModule::is_flag(agent.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
        if is_stored {
            ATTACK(agent, 0, 0, Hash40::new("armr"), 18.0, 361, 59, 0, 93, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 18.0, 361, 59, 0, 93, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 18.0, 361, 59, 0, 93, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
        else {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            ATTACK(agent, 0, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 25.0, 361, 59, 0, 93, 2.5, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 2.5, -3.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 5.0, 1.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 25.0, 361, 59, 0, 93, 4.0, 5.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ssbexo_captain_aerial_neutral_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
    }
    frame(agent.lua_state_agent, 51.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(agent.module_accessor, 1, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        let is_stored = WorkModule::is_flag(agent.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
        if is_stored {
            ATTACK(agent, 0, 0, Hash40::new("armr"), 18.0, 361, 59, 0, 93, 5.175, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 18.0, 361, 59, 0, 93, 2.875, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 18.0, 361, 59, 0, 93, 4.6, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
        else {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            ATTACK(agent, 0, 0, Hash40::new("armr"), 22.0, 361, 59, 0, 93, 5.175, -1.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("shoulderr"), 22.0, 361, 59, 0, 93, 2.875, -2.5, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 22.0, 361, 59, 0, 93, 4.6, 4.2, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("armr"), 22.0, 361, 68, 0, 58, 2.5, -3.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("armr"), 22.0, 361, 68, 0, 58, 5.0, 1.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 22.0, 361, 68, 0, 58, 4.0, 5.0, 0.0, 2.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 2, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
}

unsafe extern "C" fn ssbexo_captain_neutral_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_flash"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_hold"), Hash40::new("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 1, true);
    }
    frame(agent.lua_state_agent, 33.0);
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 51.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(agent, Hash40::new("captain_fp_hold"), false, true);
        let is_stored = WorkModule::is_flag(agent.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
        if is_stored {
            EFFECT_FOLLOW_ALPHA(agent, Hash40::new("captain_fn_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true, 1.0);
        }
    }
}

unsafe extern "C" fn ssbexo_captain_neutral_special_sound(agent: &mut L2CAgentBase) {
    let is_stored = WorkModule::is_flag(agent.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        if !is_stored {
            PLAY_SE(agent, Hash40::new("vc_captain_001"));
        }
        else {
            PLAY_SE(agent, Hash40::new("vc_captain_002"));
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_n04"));
    }
    wait(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
    wait(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_captain_special_n04"));
        if !is_stored {
            PLAY_SE(agent, Hash40::new("vc_captain_002"));
        }
        PLAY_SE(agent, Hash40::new("se_captain_special_n03"));
    }
}

unsafe extern "C" fn ssbexo_captain_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn", ssbexo_captain_grounded_neutral_special_acmd, Low)
    .game_acmd("game_specialairn", ssbexo_captain_aerial_neutral_special_acmd, Low)
    .effect_acmd("effect_specialn", ssbexo_captain_neutral_special_effect, Low)
    .effect_acmd("effect_specialairn", ssbexo_captain_neutral_special_effect, Low)
    .sound_acmd("sound_specialn", ssbexo_captain_neutral_special_sound, Low)
    .sound_acmd("sound_specialairn", ssbexo_captain_neutral_special_sound, Low)
    .expression_acmd("expression_specialn", ssbexo_captain_neutral_special_expression, Low)
    .expression_acmd("expression_specialairn", ssbexo_captain_neutral_special_expression, Low)
    .install()
    ;
}