use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_koopajr_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 93, 0, 0, 20, 6.0, 0.0, 8.0, 16.5, Some(0.0), Some(12.5), Some(16.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 20, 6.0, 0.0, 7.0, 9.7, Some(0.0), Some(12.0), Some(9.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 46, 136, 0, 40, 6.0, 0.0, 9.0, 17.0, Some(0.0), Some(12.0), Some(17.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Hi ACMD
unsafe extern "C" fn ssbexo_koopajr_forward_smash_hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 93, 0, 0, 20, 6.0, 0.0, 12.0, 15.0, Some(0.0), Some(16.5), Some(14.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 20, 6.0, 0.0, 10.0, 8.3, Some(0.0), Some(15.0), Some(7.8), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 46, 136, 0, 40, 6.0, 0.0, 13.0, 16.0, Some(0.0), Some(16.0), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Lw ACMD
unsafe extern "C" fn ssbexo_koopajr_forward_smash_lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 93, 0, 0, 20, 6.0, 0.0, 3.5, 16.0, Some(0.0), Some(7.5), Some(17.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 0, 0, 20, 6.0, 0.0, 4.5, 9.1, Some(0.0), Some(9.0), Some(10.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 46, 136, 0, 40, 6.0, 0.0, 4.5, 16.5, Some(0.0), Some(6.5), Some(17.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_koopajr_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 4.0);
    FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 53, 91, 0, 41, 5.2, 0.0, 3.5, 12.5, Some(0.0), Some(6.5), Some(12.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 53, 91, 0, 41, 5.2, 0.0, 3.5, -12.0, Some(0.0), Some(6.5), Some(-12.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    FT_MOTION_RATE(agent, 1.0);
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    FT_MOTION_RATE(agent, 0.9);
    frame(agent.lua_state_agent, 65.0);
    FT_MOTION_RATE(agent, 0.85);
}

pub fn install() {
    Agent::new("koopajr")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4", ssbexo_koopajr_forward_smash_acmd, Low)
    .game_acmd("game_attacks4hi", ssbexo_koopajr_forward_smash_hi_acmd, Low)
    .game_acmd("game_attacks4lw", ssbexo_koopajr_forward_smash_lw_acmd, Low)
    .game_acmd("game_attacklw4", ssbexo_koopajr_down_smash_acmd, Low)
    .install()
    ;
}