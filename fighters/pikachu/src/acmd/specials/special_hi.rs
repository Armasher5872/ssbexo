use super::*;

//Thunder Shock ACMD
unsafe extern "C" fn ssbexo_pikachu_up_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 75, 120, 100, 0, 5.0, 0.0, 4.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);   
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if WorkModule::is_flag(boma, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_WEAK) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 33, 0, 85, 11.0, 0.0, 4.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 55, 0, 85, 11.0, 0.0, 4.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Grounded Thunder Shock Effect
unsafe extern "C" fn ssbexo_pikachu_grounded_up_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pikachu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("pikachu_denko_elec"), Hash40::new("hip"), 5, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FLW_POS_UNSYNC_VIS(lua_state);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("pikachu_denko_line"), Hash40::new("hip"), 0, 0, 10, 90, 0, 0, 1.5, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pikachu_denko_elec"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("pikachu_denko_line"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pikachu_kaminari_hit2"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 1.3, true);
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pikachu_kaminari_hit2"), false, true);
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pikachu_cheek"), false, true);
    }
}

//Aerial Thunder Shock Effect
unsafe extern "C" fn ssbexo_pikachu_aerial_up_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pikachu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("pikachu_denko_elec"), Hash40::new("hip"), 5, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FLW_POS_UNSYNC_VIS(lua_state);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("pikachu_denko_line"), Hash40::new("hip"), 0, 0, 10, 90, 0, 0, 1.5, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pikachu_denko_elec"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("pikachu_denko_line"), true, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pikachu_kaminari_hit2"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 1.3, true);
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pikachu_kaminari_hit2"), false, true);
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pikachu_cheek"), false, true);
    }
}

//Thunder Shock Sound
unsafe extern "C" fn ssbexo_pikachu_up_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_pikachu_003"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pikachu_special_h01"));
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_pikachu_005"));
        PLAY_SE(agent, Hash40::new("se_pikachu_special_l03"));
    }
}

//Grounded Thunder Shock Expression
unsafe extern "C" fn ssbexo_pikachu_grounded_up_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

//Aerial Thunder Shock Expression
unsafe extern "C" fn ssbexo_pikachu_aerial_up_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

pub fn install() {
    Agent::new("pikachu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialhi", ssbexo_pikachu_up_special_acmd, Low)
    .acmd("game_specialairhi", ssbexo_pikachu_up_special_acmd, Low)
    .acmd("effect_specialhi", ssbexo_pikachu_grounded_up_special_effect, Low)
    .acmd("effect_specialairhi", ssbexo_pikachu_aerial_up_special_effect, Low)
    .acmd("sound_specialhi", ssbexo_pikachu_up_special_sound, Low)
    .acmd("sound_specialairhi", ssbexo_pikachu_up_special_sound, Low)
    .acmd("expression_specialhi", ssbexo_pikachu_grounded_up_special_expression, Low)
    .acmd("expression_specialairhi", ssbexo_pikachu_aerial_up_special_expression, Low)
    .install()
    ;
}