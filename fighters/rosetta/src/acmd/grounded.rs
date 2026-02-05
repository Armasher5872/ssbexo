use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_rosetta_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 361, 12, 0, 8, 8.0, 0.0, 10.0, 16.5, Some(0.0), Some(12.0), Some(16.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 2.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 361, 12, 0, 8, 8.0, 0.0, 10.0, 16.5, Some(0.0), Some(12.0), Some(16.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 361, 12, 0, 8, 8.0, 0.0, 10.0, 16.5, Some(0.0), Some(12.0), Some(16.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 361, 12, 0, 8, 8.0, 0.0, 10.0, 16.5, Some(0.0), Some(12.0), Some(16.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_rosetta_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 361, 12, 0, 8, 8.0, 0.0, 10.0, 16.5, Some(0.0), Some(12.0), Some(16.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_rosetta_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 60, 12, 0, 47, 4.0, 0.0, 4.0, 15.0, Some(0.0), Some(4.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 30, 50, 0, 30, 4.0, 0.0, 4.0, 15.0, Some(0.0), Some(4.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 4.0, 65, 125, 0, 70, 7.0, 0.0, 10.5, 13.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 1, Hash40::new("top"), 4.0, 65, 125, 0, 70, 5.0, 0.0, 10.5, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("rosetta_tico")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack100", ssbexo_rosetta_rapid_jab_acmd, Low)
    .game_acmd("game_attack100sub", ssbexo_rosetta_rapid_jab_sub_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_rosetta_dash_attack_acmd, Low)
    .install()
    ;
}