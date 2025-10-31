use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_ryu_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 16.0, 361, 98, 0, 26, 3.2, -1.5, -1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 16.0, 361, 98, 0, 26, 3.2, -6.2, -1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 17.5, 361, 98, 0, 26, 3.9, 4.3, -1.7, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_ryu_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 35, 47, 0, 50, 3.6, 0.0, 2.5, 12.0, Some(0.0), Some(3.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 80, 50, 0, 50, 2.5, 0.0, 3.0, 2.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 80, 50, 0, 50, 2.5, 0.0, 3.0, 2.5, Some(0.0), Some(3.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

pub fn install() {
    Agent::new("ryu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4", ssbexo_ryu_forward_smash_acmd, Low)
    .game_acmd("game_attacklw4", ssbexo_ryu_down_smash_acmd, Low)
    .install()
    ;
}