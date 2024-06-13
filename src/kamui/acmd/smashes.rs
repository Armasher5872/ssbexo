use super::*;

//Down Smash Charge Effect
unsafe extern "C" fn ssbexo_kamui_down_smash_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("toel"), -8, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("toer"), -8, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, false);
    }
}

//Down Smash Charge Expression
unsafe extern "C" fn ssbexo_kamui_down_smash_charge_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 0.5, 210, 2, 0.5, -10, 5, 20, 10, 10);
        AREA_WIND_2ND_arg10(agent.lua_state_agent);
        physics!(agent, *MA_MSC_CMD_PHYSICS_START_CHARGE, 1, 0.5, -1, 0.8, 0.3, 0.1, Hash40::new("invalid"));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_mask"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_rspearfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_lspearfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_rfoot"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_lfoot"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_wing"), false);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_kamui_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }  
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 270, 100, 0, 15, 7.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 270, 100, 0, 15, 5.0, 0.0, -7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }  
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 80, 63, 0, 65, 9.0, 0.0, 6.0, 11.0, Some(0.0), Some(6.0), Some(16.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 80, 63, 0, 65, 9.0, 0.0, 6.0, -11.0, Some(0.0), Some(6.0), Some(-16.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_kamui_down_smash_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_transform_splash_start"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 11, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("kamui_water_hamon"), Hash40::new("top"), 0, 0.5, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("kamui_water_splash"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
    }
    frame(agent.lua_state_agent, 20.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("kamui_counter_splash"), Hash40::new("top"), -7, 0, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(agent, Hash40::new("kamui_counter_splash"), Hash40::new("top"), -7, 0, -12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("kamui_counter_splash"), Hash40::new("top"), 7, 0, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(agent, Hash40::new("kamui_counter_splash"), Hash40::new("top"), 7, 0, -12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        }        
    }
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("kamui_counter_ripple"), Hash40::new("top"), 0, 0.5, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
    }
}

//Down Smash Sound
unsafe extern "C" fn ssbexo_kamui_down_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_horn_start"));
        macros::PLAY_SE(agent, Hash40::new("se_kamui_special_l02"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if WorkModule::get_float(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE) == 1.0 {
        if macros::is_excute(agent) {
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_kamui_rnd_attack_smash_l"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_kamui_attack05"));
        }
    }
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_smash_l01"));
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_smash_l02"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_special_l03"));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_horn_end"));
    }
}

//Down Smash Expression
unsafe extern "C" fn ssbexo_kamui_down_smash_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_mask"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_rspearfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_lspearfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_rfoot"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_lfoot"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_wing"), false);
    }
    frame(agent.lua_state_agent, 2.0);
    execute(agent.lua_state_agent, 2.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_fronthair"), false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_hair"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_mask"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_wing"), true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFFECT_SWORD_AURA);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_rfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_lfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_rspearfoot"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_lspearfoot"), true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_piercesm"), 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 14);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 3, 110, 300, 1, 0, 12, 30, 30, 40);
        AREA_WIND_2ND_arg10(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_hair"), true);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_fronthair"), true);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_mask"), false);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFFECT_SWORD_AURA);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_wing"), false);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_lfoot"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_lspearfoot"), false);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 14);
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_rfoot"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kamui_rspearfoot"), false);
    }
    frame(agent.lua_state_agent, 87.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 14);
    }
}

pub fn install() {
    Agent::new("kamui")
    .effect_acmd("effect_attacklw4charge", ssbexo_kamui_down_smash_charge_effect, Priority::Low)
    .expression_acmd("expression_attacklw4charge", ssbexo_kamui_down_smash_charge_expression, Priority::Low)
    .game_acmd("game_attacklw4", ssbexo_kamui_down_smash_acmd, Priority::Low)
    .effect_acmd("effect_attacklw4", ssbexo_kamui_down_smash_effect, Priority::Low)
    .sound_acmd("sound_attacklw4", ssbexo_kamui_down_smash_sound, Priority::Low)
    .expression_acmd("expression_attacklw4", ssbexo_kamui_down_smash_expression, Priority::Low)
    .install()
    ;
}