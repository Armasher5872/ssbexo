use super::*;

//Taunt Effect
unsafe extern "C" fn ssbexo_armstrong_taunt_effect(_agent: &mut L2CAgentBase) {}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_armstrong_up_taunt_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_appeal_h01"));
    }
}

//Side Taunt Sound
unsafe extern "C" fn ssbexo_armstrong_side_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_appeal_s01"));
    }
}

//Down Taunt ACMD
unsafe extern "C" fn ssbexo_armstrong_down_taunt_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
        HitModule::set_check_catch(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        HitModule::set_check_catch(agent.module_accessor, true, 0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_armstrong_down_taunt_sound(_agent: &mut L2CAgentBase) {}

//Down Taunt Expression
unsafe extern "C" fn ssbexo_armstrong_down_taunt_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 96.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Ledge Attack Effect
unsafe extern "C" fn ssbexo_armstrong_ledge_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 10, 4, 0, 25, 25, 1.25, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    }
}

//Face Down Getup Attack Effect
unsafe extern "C" fn ssbexo_armstrong_face_down_getup_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 2.5, 5, -15, 180, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8, -18.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 1.5);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -0.5, 10, 0.8, 180, -230, 90, 1.4, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 14, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_d"), false, false);
    }
}

//Face Up Getup Attack Effect
unsafe extern "C" fn ssbexo_armstrong_face_up_getup_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -3, 6, -1, 0, 180, 15, 1.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_d"), false, false);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 5.5, 0, 0, 40, 13, 1.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

//Standing Tech Sound
unsafe extern "C" fn ssbexo_armstrong_passive_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_s"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_m"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_l"));
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_cliffcatch"));
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_ganon_landing01"));
    }
}

//Ceiling Tech Sound
unsafe extern "C" fn ssbexo_armstrong_passive_ceil_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_s"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_m"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_l"));
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_cliffcatch"));
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_ganon_landing01"));
    }
}

//Wall Tech Sound
unsafe extern "C" fn ssbexo_armstrong_passive_wall_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_s"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_m"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_l"));
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_cliffcatch"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_landing01"));
    }
}

pub fn install() {
    Agent::new("ganon")
    .effect_acmd("effect_appealhir", ssbexo_armstrong_taunt_effect)
    .effect_acmd("effect_appealhil", ssbexo_armstrong_taunt_effect)
    .effect_acmd("effect_appealsr", ssbexo_armstrong_taunt_effect)
    .effect_acmd("effect_appealsl", ssbexo_armstrong_taunt_effect)
    .effect_acmd("effect_appeallwr", ssbexo_armstrong_taunt_effect)
    .effect_acmd("effect_appeallwl", ssbexo_armstrong_taunt_effect)
    .expression_acmd("expression_appeallwr", ssbexo_armstrong_down_taunt_expression)
    .expression_acmd("expression_appeallwl", ssbexo_armstrong_down_taunt_expression)
    .sound_acmd("sound_appealhir", ssbexo_armstrong_up_taunt_sound)
    .sound_acmd("sound_appealhil", ssbexo_armstrong_up_taunt_sound)
    .sound_acmd("sound_appealsr", ssbexo_armstrong_side_taunt_sound)
    .sound_acmd("sound_appealsl", ssbexo_armstrong_side_taunt_sound)
    .game_acmd("game_appeallwr", ssbexo_armstrong_down_taunt_acmd)
    .game_acmd("game_appeallwl", ssbexo_armstrong_down_taunt_acmd)
    .sound_acmd("sound_appeallwr", ssbexo_armstrong_down_taunt_sound)
    .sound_acmd("sound_appeallwl", ssbexo_armstrong_down_taunt_sound)
    .effect_acmd("effect_cliffattack", ssbexo_armstrong_ledge_attack_effect)
    .effect_acmd("effect_attackdownd", ssbexo_armstrong_face_down_getup_attack_effect)
    .effect_acmd("effect_attackdownu", ssbexo_armstrong_face_up_getup_attack_effect)
    .sound_acmd("sound_passive", ssbexo_armstrong_passive_sound)
    .sound_acmd("sound_passiveceil", ssbexo_armstrong_passive_ceil_sound)
    .sound_acmd("sound_passivewall", ssbexo_armstrong_passive_wall_sound)
    .install()
    ;
}