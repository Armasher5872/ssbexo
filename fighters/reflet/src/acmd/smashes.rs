use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_reflet_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 16.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 361, 90, 0, 53, 4.0, 0.0, 10.0, 17.0, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 87, 0, 50, 3.0, 0.0, 10.0, 14.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 10.0, 361, 80, 0, 60, 1.5, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 28.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 32.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            if is_excute(agent) {
                VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
                WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        FighterSpecializer_Reflet::throwaway_sword(agent.battle_object as *mut Fighter, &Vector2f{x: 1.0, y: 8.0}, true);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_reflet_up_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 15.0, 92, 86, 0, 55, 4.3, 0.0, 7.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 70, 79, 0, 70, 3.5, 0.0, 9.0, 7.5, Some(0.0), Some(9.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 9.0, 55, 84, 0, 45, 4.2, 0.0, 5.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 55, 84, 0, 45, 3.5, 0.0, 9.0, 7.5, Some(0.0), Some(9.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 5.8);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    wait(agent.lua_state_agent, 4.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 10.0, 70, 80, 0, 60, 2.5, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 33.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 39.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            if is_excute(agent) {
                VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
                WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        FighterSpecializer_Reflet::throwaway_sword(agent.battle_object as *mut Fighter, &Vector2f{x: 1.5, y: 22.0}, true);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_reflet_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 16.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 55, 95, 0, 50, 4.5, 0.0, 13.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 55, 95, 0, 50, 4.5, 0.0, 16.0, 12.0, Some(0.0), Some(2.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 28, 95, 0, 50, 3.0, 0.0, 13.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 28, 95, 0, 50, 3.0, 0.0, 16.0, 10.0, Some(0.0), Some(2.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, 14.5, Some(0.0), Some(3.5), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, -6.5, Some(0.0), Some(3.5), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }   
    }
    frame(agent.lua_state_agent, 21.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, 19.0, Some(0.0), Some(3.5), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, -11.0, Some(0.0), Some(3.5), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }   
    }
    frame(agent.lua_state_agent, 23.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, 19.0, Some(0.0), Some(3.5), Some(14.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, -11.0, Some(0.0), Some(3.5), Some(-6.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }   
    }
    frame(agent.lua_state_agent, 24.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, 19.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, -11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }   
    }
    frame(agent.lua_state_agent, 29.0);
    FT_MOTION_RATE(agent, 0.8);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            if is_excute(agent) {
                VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
                WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        FighterSpecializer_Reflet::throwaway_sword(agent.battle_object as *mut Fighter, &Vector2f{x: -2.5, y: 7.0}, true);
    }
}

pub fn install() {
    Agent::new("reflet")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4", ssbexo_reflet_forward_smash_acmd, Low)
    .game_acmd("game_attackhi4", ssbexo_reflet_up_smash_acmd, Low)
    .game_acmd("game_attacklw4", ssbexo_reflet_down_smash_acmd, Low)
    .install()
    ;
}