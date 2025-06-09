use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_metaknight_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 30, 0, 10, 7.0, 0.0, 6.5, 8.0, Some(0.0), Some(6.5), Some(15.5), 0.5, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        }
        wait(agent.lua_state_agent, 3.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

//Rapid Jab Finisher ACMD
unsafe extern "C" fn ssbexo_metaknight_rapid_jab_finisher_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 140, 0, 60, 5.0, 0.0, 8.0, 11.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 140, 0, 60, 6.0, 0.0, 8.0, 22.0, Some(0.0), Some(8.0), Some(22.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 140, 0, 60, 6.0, 0.0, 12.5, 16.5, Some(0.0), Some(12.5), Some(22.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    MotionModule::set_rate(agent.module_accessor, 1.13);
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_metaknight_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 40, 85, 0, 40, 6.0, 0.0, 10.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 40, 85, 0, 40, 6.0, 0.0, 4.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 40, 85, 0, 40, 8.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 8.0);
        ATK_POWER(agent, 1, 8.0);
        ATK_POWER(agent, 2, 8.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATK_POWER(agent, 0, 6.0);
        ATK_POWER(agent, 1, 6.0);
        ATK_POWER(agent, 2, 6.0);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_metaknight_dash_attack_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_drilllush"), Hash40::new("haver"), 0, 16, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 9.0);
    for _ in 0..2 {
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("rot"), 0, 0, 20, 90, 0, 0, 0.5, 0, 0, 10, 20, 15, 360, true);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("rot"), 0, 0, 15, 90, 0, 0, 0.8, 0, 0, 10, 20, 15, 360, true);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("rot"), 0, 0, 10, 90, 0, 0, 1.1, 0, 0, 10, 20, 15, 360, true);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_spin_wind_s"), Hash40::new("rot"), 0, 0, 5, 90, 0, 0, 1.4, 0, 0, 10, 20, 15, 360, true);
        }
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_drilllush"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
}

//Dash Attack Sound
unsafe extern "C" fn ssbexo_metaknight_dash_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("vc_metaknight_special_s01"));
        PLAY_SE(agent, Hash40::new("se_metaknight_special_s02"));
    }
    wait(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_special_s02"));
    }
    wait(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_special_s02_02"));
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_metaknight_special_s02"));
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_special_s03"));
    }
}

//Dash Attack Expression
unsafe extern "C" fn ssbexo_metaknight_dash_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_normal") as i64);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashss"), 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_27_spinslash"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_normal") as i64);
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack100", ssbexo_metaknight_rapid_jab_acmd, Low)
    .game_acmd("game_attack100end", ssbexo_metaknight_rapid_jab_finisher_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_metaknight_dash_attack_acmd, Low)
    .effect_acmd("effect_attackdash", ssbexo_metaknight_dash_attack_effect, Low)
    .sound_acmd("sound_attackdash", ssbexo_metaknight_dash_attack_sound, Low)
    .expression_acmd("expression_attackdash", ssbexo_metaknight_dash_attack_expression, Low)
    .install()
    ;
}