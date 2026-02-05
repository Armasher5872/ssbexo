use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_gamewatch_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.8, 361, 20, 0, 12, 5.5, 0.0, 5.0, 14.5, None, None, None, 0.7, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 361, 20, 0, 12, 4.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(5.0), 0.7, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 4.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 10);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 10);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        wait_loop_clear(agent);
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_100"), false, -1.0);
        }
        frame(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.8, 361, 20, 0, 12, 5.5, 0.0, 5.0, 14.5, None, None, None, 0.7, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 361, 20, 0, 12, 4.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(5.0), 0.7, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 4.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 10);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 10);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_gamewatch_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.8, 361, 20, 0, 12, 5.5, 0.0, 5.0, 14.5, None, None, None, 0.7, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 361, 20, 0, 12, 4.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(5.0), 0.7, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 4.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 10);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 10);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_gamewatch_dash_attack_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 9.0, 4.0);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 50, 70, 0, 70, 6.0, 0.0, 3.5, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.5, 50, 70, 0, 70, 5.5, 0.0, 3.5, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 39.0);
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
}

pub fn install() {
    Agent::new("gamewatch")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack100", ssbexo_gamewatch_rapid_jab_acmd, Low)
    .game_acmd("game_attack100sub", ssbexo_gamewatch_rapid_jab_sub_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_gamewatch_dash_attack_acmd, Low)
    .install()
    ;
}