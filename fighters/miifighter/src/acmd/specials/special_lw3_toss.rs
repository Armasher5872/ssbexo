use super::*;

//Counter Throw Throw Toss ACMD
unsafe extern "C" fn ssbexo_miifighter_counter_throw_throw_toss_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let counter_throw_object_id = WorkModule::get_int(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 2.0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("haver"), 11.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 1.5, 50, false, 1.5, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        EffectModule::kill_kind(boma, Hash40::new("sys_reflection"), true, true);
    }
    frame(lua_state, 18.0);
    if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            if is_excute(agent) {
                WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
                WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
                GroundModule::set_ignore_boss(counter_throw_boma, false);
                GroundModule::set_passable_check(counter_throw_boma, true);
                GroundModule::set_collidable(counter_throw_boma, true);
                JostleModule::set_status(counter_throw_boma, true);
                shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
            }
        }
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            if is_excute(agent) {
                WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
                WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
                GroundModule::set_ignore_boss(counter_throw_boma, false);
                GroundModule::set_passable_check(counter_throw_boma, true);
                GroundModule::set_collidable(counter_throw_boma, true);
                JostleModule::set_status(counter_throw_boma, true);
                shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
            } 
        }
    }
    else {
        if is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
            WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
            shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
    }
}

//Grounded Counter Throw Throw Toss Effect
unsafe extern "C" fn ssbexo_miifighter_grounded_counter_throw_throw_toss_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        COL_PRI(agent, 101);
        FLASH(agent, 1, 1, 1, 0);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_catch"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("miifighter_counter_arc"), Hash40::new("top"), -1, 8, 1, 0, 112, 90, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("top"), 0, 11, 4, -13, -56, 46, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//Aerial Counter Throw Throw Toss Effect
unsafe extern "C" fn ssbexo_miifighter_aerial_counter_throw_throw_toss_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        COL_PRI(agent, 101);
        FLASH(agent, 1, 1, 1, 0);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_catch"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("miifighter_counter_arc"), Hash40::new("top"), -1, 8, 1, 0, 112, 90, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("top"), 0, 11, 4, -13, -56, 46, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_reflection"), true, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_reflection"), true, true);
    }
}

//Counter Throw Throw Toss Sound
unsafe extern "C" fn ssbexo_miifighter_counter_throw_throw_toss_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_cliff_catch"));
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_special_c3_l01"));
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_special_c3_l03"));
    }
}

//Grounded Counter Throw Throw Toss Expression
unsafe extern "C" fn ssbexo_miifighter_grounded_counter_throw_throw_toss_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x23c33f3bdc));
    }
}

//Aerial Counter Throw Throw Toss Expression
unsafe extern "C" fn ssbexo_miifighter_aerial_counter_throw_throw_toss_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x23c33f3bdc));
    }
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallw3throwtoss", ssbexo_miifighter_counter_throw_throw_toss_acmd, Low)
    .acmd("game_specialairlw3throwtoss", ssbexo_miifighter_counter_throw_throw_toss_acmd, Low)
    .acmd("effect_speciallw3throwtoss", ssbexo_miifighter_grounded_counter_throw_throw_toss_effect, Low)
    .acmd("effect_specialairlw3throwtoss", ssbexo_miifighter_aerial_counter_throw_throw_toss_effect, Low)
    .acmd("sound_speciallw3throwtoss", ssbexo_miifighter_counter_throw_throw_toss_sound, Low)
    .acmd("sound_specialairlw3throwtoss", ssbexo_miifighter_counter_throw_throw_toss_sound, Low)
    .acmd("expression_speciallw3throwtoss", ssbexo_miifighter_grounded_counter_throw_throw_toss_expression, Low)
    .acmd("expression_specialairlw3throwtoss", ssbexo_miifighter_aerial_counter_throw_throw_toss_expression, Low)
    .install()
    ;
}