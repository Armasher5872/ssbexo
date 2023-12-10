use super::*;

//Up Smash ACMD
unsafe extern "C" fn ssbexo_sonic_up_smash_acmd(agent: &mut L2CAgentBase) {
    let parried = WorkModule::get_int(agent.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("trans"), 1.0, 115, 120, 120, 0, 3.0, 0.0, 3.0, -12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("trans"), 1.0, 115, 120, 120, 0, 3.0, 0.0, 3.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        if parried == 1 {
            macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 14.0, 90, 73, 0, 73, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        else {
            for _ in 0..5 {
                if macros::is_excute(agent) {
                    macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 1.0, 90, 110, 110, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
                }
                wait(agent.lua_state_agent, 1.0);
                if macros::is_excute(agent) {
                    AttackModule::clear_all(agent.module_accessor);
                }
                wait(agent.lua_state_agent, 1.0);
            }
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        if parried == 1 {
            macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 14.0, 90, 73, 0, 73, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 3.0, 82, 130, 0, 80, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_sonic_up_smash_effect(agent: &mut L2CAgentBase) {
    let parried = WorkModule::get_int(agent.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        if parried == 1 {
            macros::EFFECT_FOLLOW(agent, Hash40::new_raw(0x0e27bc68a2u64), Hash40::new("top"), 0, 20, 1, 180, 270, 90, 0.85, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0);
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 19, 0, 0, 0, 0, 0.5, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0);
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        if parried == 0 {
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sphere"), 0, 0, 0, 0, -160, 270, 0.75, true, 1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if parried == 0 {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        if parried == 0 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spinwind"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spinblur_plain"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        if parried == 0 {
            macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("sphere"), -0.5, 3, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 360, false);   
        }
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        if parried == 0 {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sonic_spinwind"), true, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sonic_spinblur_plain"), true, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sonic_spintrace"), false, true);   
        }
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        if parried == 0 {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);   
        }
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_sonic_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("footr"), 13.0, 20, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footl"), 13.0, 20, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 13.0, 20, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("kneel"), 13.0, 20, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("legr"), 13.0, 20, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 5, 0, Hash40::new("legl"), 13.0, 20, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_sonic_down_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new_raw(0x0e27bc68a2u64), Hash40::new("top"), 0.0, 3.0, 0.0, 0, 90, 0, 1.0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.05, 0.07);
	    macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new_raw(0x0e27bc68a2u64), Hash40::new("top"), 0.0, 3.0, 0.0, 0, 270, 0, 1.0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.05, 0.07);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

pub fn install() {
    Agent::new("sonic")
    .game_acmd("game_attackhi4", ssbexo_sonic_up_smash_acmd)
    .effect_acmd("effect_attackhi4", ssbexo_sonic_up_smash_effect)
    .game_acmd("game_attacklw4", ssbexo_sonic_down_smash_acmd)
    .effect_acmd("effect_attacklw4", ssbexo_sonic_down_smash_effect)
    .install()
    ;
}