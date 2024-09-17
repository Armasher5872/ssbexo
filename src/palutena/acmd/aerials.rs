use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_palutena_nair_acmd(agent: &mut L2CAgentBase) {
    let prev_status_kind = StatusModule::prev_status_kind(agent.module_accessor, 0);
    if ![*FIGHTER_STATUS_KIND_SPECIAL_HI, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_GLIDE, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_TURN].contains(&prev_status_kind) {
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("stick"), 1.4, 367, 100, 0, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("stick"), 1.4, 367, 100, 0, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 3, 0, Hash40::new("stick"), 1.4, 95, 100, 80, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("stick"), 1.4, 367, 100, 65, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("stick"), 1.4, 367, 100, 65, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 3, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("stick"), 5.1, 55, 80, 0, 65, 9.8, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("stick"), 5.1, 55, 80, 0, 65, 11.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 50.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE) {
        if macros::is_excute(agent) {
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
    }
}

//Fair ACMD
unsafe extern "C" fn ssbexo_palutena_fair_acmd(agent: &mut L2CAgentBase) {
    let prev_status_kind = StatusModule::prev_status_kind(agent.module_accessor, 0);
    if ![*FIGHTER_STATUS_KIND_SPECIAL_HI, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_GLIDE, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_TURN].contains(&prev_status_kind) {
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 45, 92, 0, 31, 3.0, 0.0, 6.9, 9.0, Some(0.0), Some(6.9), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 43.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE) {
        if macros::is_excute(agent) {
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
    }
}

//Bair ACMD
unsafe extern "C" fn ssbexo_palutena_bair_acmd(agent: &mut L2CAgentBase) {
    let prev_status_kind = StatusModule::prev_status_kind(agent.module_accessor, 0);
    if ![*FIGHTER_STATUS_KIND_SPECIAL_HI, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_GLIDE, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_TURN].contains(&prev_status_kind) {
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("virtualshield"), *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 95, 0, 30, 7.0, 0.0, 10.2, -14.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("virtualshield"), *HIT_STATUS_OFF);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 48.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE) {
        if macros::is_excute(agent) {
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
    }
}

//Uair ACMD
unsafe extern "C" fn ssbexo_palutena_uair_acmd(agent: &mut L2CAgentBase) {
    let prev_status_kind = StatusModule::prev_status_kind(agent.module_accessor, 0);
    if ![*FIGHTER_STATUS_KIND_SPECIAL_HI, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_GLIDE, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_TURN].contains(&prev_status_kind) {
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 105, 100, 40, 0, 4.0, 0.0, 20.0, 6.0, Some(0.0), Some(20.0), Some(-6.0), 0.6, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 367, 100, 20, 0, 4.0, 0.0, 22.8, 0.0, None, None, None, 0.6, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 367, 100, 20, 0, 3.0, 0.0, 18.0, 4.0, Some(0.0), Some(22.0), Some(9.0), 0.6, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 367, 100, 20, 0, 3.0, 0.0, 18.0, -4.0, Some(0.0), Some(22.0), Some(-9.0), 0.6, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 88, 152, 0, 50, 6.0, 0.0, 23.0, 0.0, None, None, None, 1.5, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 88, 152, 0, 50, 4.0, 0.0, 18.0, 4.0, Some(0.0), Some(23.0), Some(12.0), 1.5, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 88, 152, 0, 50, 4.0, 0.0, 18.0, -4.0, Some(0.0), Some(23.0), Some(-12.0), 1.5, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 50.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE) {
        if macros::is_excute(agent) {
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
    }
}

//Dair ACMD
unsafe extern "C" fn ssbexo_palutena_dair_acmd(agent: &mut L2CAgentBase) {
    let prev_status_kind = StatusModule::prev_status_kind(agent.module_accessor, 0);
    if ![*FIGHTER_STATUS_KIND_SPECIAL_HI, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_GLIDE, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_TURN].contains(&prev_status_kind) {
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 270, 100, 0, 20, 4.2, 0.0, -3.7, 0.5, Some(0.0), Some(-5.0), Some(0.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 55, 100, 0, 30, 6.3, 0.0, -5.0, 1.5, Some(0.0), Some(-5.0), Some(-0.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 57.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE) {
        if macros::is_excute(agent) {
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
    }
}

pub fn install() {
    Agent::new("palutena")
    .game_acmd("game_attackairn", ssbexo_palutena_nair_acmd, Priority::Low)
    .game_acmd("game_attackairf", ssbexo_palutena_fair_acmd, Priority::Low)
    .game_acmd("game_attackairb", ssbexo_palutena_bair_acmd, Priority::Low)
    .game_acmd("game_attackairhi", ssbexo_palutena_uair_acmd, Priority::Low)
    .game_acmd("game_attackairlw", ssbexo_palutena_dair_acmd, Priority::Low)
    .install()
    ;
}