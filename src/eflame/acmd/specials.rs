use super::*;

//Neutral Special 1 ACMD
unsafe extern "C" fn ssbexo_eflame_neutral_special_1_acmd(agent: &mut L2CAgentBase) {
    let kirby_check = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if macros::is_excute(agent) {
        WorkModule::set_int64(agent.module_accessor, hash40("special_n1") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Neutral Special 1 Common ACMD
unsafe extern "C" fn ssbexo_eflame_neutral_special_1_common_acmd(agent: &mut L2CAgentBase) {
    let kirby_check = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Neutral Special 2 ACMD
unsafe extern "C" fn ssbexo_eflame_neutral_special_2_acmd(agent: &mut L2CAgentBase) {
    let kirby_check = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if macros::is_excute(agent) {
        WorkModule::set_int64(agent.module_accessor, hash40("special_n2") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
}

//Neutral Special 2 Common ACMD
unsafe extern "C" fn ssbexo_eflame_neutral_special_2_common_acmd(agent: &mut L2CAgentBase) {
    let kirby_check = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
}

//Neutral Special 3 ACMD
unsafe extern "C" fn ssbexo_eflame_neutral_special_3_acmd(agent: &mut L2CAgentBase) {
    let kirby_check = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if macros::is_excute(agent) {
        WorkModule::set_int64(agent.module_accessor, hash40("special_n3") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Neutral Special 3 Common ACMD
unsafe extern "C" fn ssbexo_eflame_neutral_special_3_common_acmd(agent: &mut L2CAgentBase) {
    let kirby_check = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    if kirby_check {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 367, 50, 70, 0, 6.0, 0.0, 6.5, 14.0, Some(0.0), Some(6.5), Some(-14.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    if macros::is_excute(agent) {
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("eflame")
    .game_acmd("game_specialn1", ssbexo_eflame_neutral_special_1_acmd, Priority::Low)
    .game_acmd("game_specialn1common", ssbexo_eflame_neutral_special_1_common_acmd, Priority::Low)
    .game_acmd("game_specialn2", ssbexo_eflame_neutral_special_2_acmd, Priority::Low)
    .game_acmd("game_specialn2common", ssbexo_eflame_neutral_special_2_common_acmd, Priority::Low)
    .game_acmd("game_specialn3", ssbexo_eflame_neutral_special_3_acmd, Priority::Low)
    .game_acmd("game_specialn3common", ssbexo_eflame_neutral_special_3_common_acmd, Priority::Low)
    .install()
    ;
}