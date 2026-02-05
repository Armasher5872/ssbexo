use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_palutena_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 20, 0, 10, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(13.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 7);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        wait_loop_clear(agent);
        frame(agent.lua_state_agent, 2.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 20, 0, 10, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(13.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 7);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 20, 0, 10, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(13.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 7);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 20, 0, 10, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(13.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 7);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 11.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 20, 0, 10, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(13.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 7);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 14.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 20, 0, 10, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(13.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 7);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 17.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 20, 0, 10, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(13.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 7);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_palutena_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 20, 0, 10, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(13.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 7);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_palutena_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(agent, Hash40::new("virtualshield"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 70, 76, 0, 71, 4.5, 0.0, 9.0, 13.0, Some(0.0), Some(11.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 60, 76, 0, 71, 3.5, 0.0, 10.0, 5.0, Some(0.0), Some(10.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 60, 56, 0, 61, 2.5, 0.0, 9.0, 10.0, Some(0.0), Some(12.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("virtualshield"), *HIT_STATUS_OFF);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("palutena")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack100", ssbexo_palutena_rapid_jab_acmd, Low)
    .game_acmd("game_attack100sub", ssbexo_palutena_rapid_jab_sub_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_palutena_dash_attack_acmd, Low)
    .install()
    ;
}