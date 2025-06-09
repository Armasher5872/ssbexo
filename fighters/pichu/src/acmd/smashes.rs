use super::*;

//Up Smash ACMD
unsafe extern "C" fn ssbexo_pichu_up_smash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 95, 120, 80, 0, 5.0, 0.0, 4.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 95, 120, 80, 0, 5.0, 0.0, 4.0, -6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        FT_ADD_DAMAGE(agent, 3.5);
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 90, 80, 0, 60, 8.0, 0.0, 25.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_pichu_up_smash_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 5, 7, 0, 0, 0, 1, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 5, -7, 0, 180, 0, 1, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec2"), false, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit2"), Hash40::new("top"), 0, 22, 0, 0, 90, 0, 1.15, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit"), Hash40::new("top"), 0, 22, 0, 0, 90, 0, 0.85, true);
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pichu_kaminari_hit2"), false, true);
        FLASH(agent, 0, 0, 0, 0);
        BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 2, 0, 0, 0, 0);
        BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pichu_kaminari_hit"), false, true);
    }
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Smash Sound
unsafe extern "C" fn ssbexo_pichu_up_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pichu_smash_l01"));
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_pichu_attack06"));
    }
}

//Up Smash Expression
unsafe extern "C" fn ssbexo_pichu_up_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 2.0);
    sv_animcmd::execute(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 52.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 8);
    }
}

pub fn install() {
    Agent::new("pichu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackhi4", ssbexo_pichu_up_smash_acmd, Low)
    .effect_acmd("effect_attackhi4", ssbexo_pichu_up_smash_effect, Low)
    .sound_acmd("sound_attackhi4", ssbexo_pichu_up_smash_sound, Low)
    .expression_acmd("expression_attackhi4", ssbexo_pichu_up_smash_expression, Low)
    .install()
    ;
}