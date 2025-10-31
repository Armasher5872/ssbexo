use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_ridley_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 6.0, 7.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 54.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 6.0, 5.0);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_ridley_up_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("legr"), 17.0, 82, 78, 0, 58, 4.5, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 17.0, 82, 78, 0, 58, 4.5, 0.0, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("toer"), 17.0, 82, 78, 0, 58, 6.0, -1.5, -1.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_ridley_down_smash_acmd(agent: &mut L2CAgentBase) {
     frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 85, 76, 0, 69, 7.5, 0.0, 5.0, 21.0, Some(0.0), Some(5.0), Some(-21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Expression
unsafe extern "C" fn ssbexo_ridley_down_smash_expression(agent : &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        slope!(agent,*MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install() {
    Agent::new("ridley")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4", ssbexo_ridley_forward_smash_acmd, Low)
    .game_acmd("game_attackhi4", ssbexo_ridley_up_smash_acmd, Low)
    .game_acmd("game_attacklw4", ssbexo_ridley_down_smash_acmd, Low)
    .expression_acmd("expression_attacklw4", ssbexo_ridley_down_smash_expression, Low)
    .install()
    ;
}