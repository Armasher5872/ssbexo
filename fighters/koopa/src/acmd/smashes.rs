use super::*;

//Forward Smash Charge Effect
unsafe extern "C" fn ssbexo_koopa_forward_smash_charge_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 20, 0, 8, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    EFFECT_FLIP(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), -2, 3, 8, 0, 0, 0, 1, 6, 3, 6, 0, 0, 0, true, *EF_FLIP_YZ);
    EFFECT_FLIP(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -8, 4, -10, 0, 0, 0, 1, 6, 3, 6, 0, 0, 0, true, *EF_FLIP_YZ);
}

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_koopa_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12.0);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 24.0, 35, 90, 0, 25, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 24.0, 35, 90, 0, 25, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 24.0, 35, 90, 0, 25, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_koopa_forward_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 12, 15, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("koopa_smash_line"), Hash40::new("koopa_smash_line"), Hash40::new("top"), 0, 13, -20, 0, 0, 0, 2.3, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("koopa_smash_line"), Hash40::new("koopa_smash_line"), Hash40::new("top"), 0, 10, -26, 0, 0, 0, 2.3, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12, 22, 0, 0, 0, 2.6, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_line"), true, true);
    }
}

//Forward Smash Sound
unsafe extern "C" fn ssbexo_koopa_forward_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
        PLAY_SE(agent, Hash40::new("vc_koopa_attack05"));
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_smash_s01"));
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_smash_s02"));
    }
}

//Forward Smash Expression
unsafe extern "C" fn ssbexo_koopa_forward_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 14.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
    }
}

pub fn install() {
    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_attacks4charge", ssbexo_koopa_forward_smash_charge_effect, Low)
    .game_acmd("game_attacks4", ssbexo_koopa_forward_smash_acmd, Low)
    .effect_acmd("effect_attacks4", ssbexo_koopa_forward_smash_effect, Low)
    .sound_acmd("sound_attacks4", ssbexo_koopa_forward_smash_sound, Low)
    .expression_acmd("expression_attacks4", ssbexo_koopa_forward_smash_expression, Low)
    .install()
    ;
}