use super::*;

//Forward Smash Charge Effect
unsafe extern "C" fn ssbexo_plizardon_forward_smash_charge_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 15, 0, 8, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    for _ in 0..59 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 3, 4.0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
        }
        wait(agent.lua_state_agent, 3.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 3, 4.0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
        }
    }
}

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_plizardon_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        ATTACK(agent, 0, 0, Hash40::new("top"), 21.0, 361, 50, 0, 75, 10.0, 0.0, 5.0, 10.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_plizardon_forward_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    for _ in 0..3 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 5.0, 10.0, 0, 0, 0, 1.0, true);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_bomb_a"), false, true);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_plizardon_up_smash_acmd(agent: &mut L2CAgentBase) {
    let vector = Vector2f{x: 0.0, y: 18.0};
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 110, 100, 100, 0, 6.0, 0.0, 9.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 7.0);
    FT_MOTION_RATE(agent, 0.6);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("wingl4"), 5.0, 368, 100, 0, 0, 4.2, 1.5, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("wingl2"), 5.0, 368, 100, 0, 0, 5.0, 3.5, -1.5, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(agent, 3, 0, Hash40::new("wingl4"), 5.0, 280, 100, 32, 0, 4.2, 7.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &vector, 8, false);
        AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &vector, 8, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("wingl4"), 5.0, 320, 100, 42, 0, 4.2, 7.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("wingl4"), 5.0, 340, 100, 60, 0, 4.2, 7.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(agent, 0, 0, Hash40::new("wingl4"), 5.0, 368, 100, 0, 0, 4.2, 7.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &vector, 8, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("wingr2"), 11.0, 83, 75, 0, 40, 5.5, 3.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("wingr4"), 11.0, 83, 75, 0, 40, 5.0, 2.7, -2.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("wingr7"), 11.0, 83, 75, 0, 40, 4.5, 6.5, 0.0, 6.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_plizardon_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 80, 79, 0, 40, 6.0, 0.0, 5.0, 21.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 80, 79, 0, 40, 6.0, 0.0, 5.0, -15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 16.0, 80, 79, 0, 40, 10.0, 0.0, 5.5, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 16.0, 80, 79, 0, 40, 10.0, 0.0, 5.5, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("plizardon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_attacks4charge", ssbexo_plizardon_forward_smash_charge_effect, Low)
    .game_acmd("game_attacks4", ssbexo_plizardon_forward_smash_acmd, Low)
    .effect_acmd("effect_attacks4", ssbexo_plizardon_forward_smash_effect, Low)
    .game_acmd("game_attackhi4", ssbexo_plizardon_up_smash_acmd, Low)
    .game_acmd("game_attacklw4", ssbexo_plizardon_down_smash_acmd, Low)
    .install()
    ;
}