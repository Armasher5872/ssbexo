use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_pitb_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 4.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(19.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 2.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 4.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(19.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 4.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 4.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(19.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 6.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 4.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(19.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 4.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(19.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 10.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 4.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(19.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 12.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 4.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(19.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 14.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 4.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(19.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_pitb_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 4.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(19.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_pitb_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 75, 60, 0, 40, 3.5, 0.0, 4.5, 16.0, Some(0.0), Some(7.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.37);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("pitb")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack100", ssbexo_pitb_rapid_jab_acmd, Low)
    .game_acmd("game_attack100sub", ssbexo_pitb_rapid_jab_sub_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_pitb_dash_attack_acmd, Low)
    .install()
    ;
}