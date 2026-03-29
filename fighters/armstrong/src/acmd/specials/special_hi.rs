use super::*;

//Up Special ACMD
unsafe extern "C" fn ssbexo_armstrong_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 6.5, 0.0, 6.0, 6.0, None, None, None, *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_GA);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
        armstrong_clear_charge(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
    }
}

//Up Special Effect
unsafe extern "C" fn ssbexo_armstrong_up_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_flame_grab_hold"), Hash40::new("arml"), 2, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("armstrong_flame_grab_hold"), false, true);
    }
}

//Up Special Sound
unsafe extern "C" fn ssbexo_armstrong_up_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h05"));
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_h01"));
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_h05"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_h01"));
    }
}

//Up Special Expression
unsafe extern "C" fn ssbexo_armstrong_up_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
}

pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    Agent::new("ganon")
    .set_costume(costume.to_vec())
    .game_acmd("game_specialhi", ssbexo_armstrong_up_special_acmd, Low)
    .effect_acmd("effect_specialhi", ssbexo_armstrong_up_special_effect, Low)
    .sound_acmd("sound_specialhi", ssbexo_armstrong_up_special_sound, Low)
    .expression_acmd("expression_specialhi", ssbexo_armstrong_up_special_expression, Low)
    .game_acmd("game_specialairhi", ssbexo_armstrong_up_special_acmd, Low)
    .effect_acmd("effect_specialairhi", ssbexo_armstrong_up_special_effect, Low)
    .sound_acmd("sound_specialairhi", ssbexo_armstrong_up_special_sound, Low)
    .expression_acmd("expression_specialairhi", ssbexo_armstrong_up_special_expression, Low)
    .install()
    ;
}