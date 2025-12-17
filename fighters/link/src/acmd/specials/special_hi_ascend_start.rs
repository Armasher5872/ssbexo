use super::*;

//Up Special Ascend Start ACMD
unsafe extern "C" fn ssbexo_link_special_hi_ascend_start_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {       
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
}

//Up Special Ascend Start Effect
unsafe extern "C" fn ssbexo_link_special_hi_ascend_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {        
        let target_y = WorkModule::get_float(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
        let pos_y = PostureModule::pos_y(agent.module_accessor);
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, target_y-pos_y+5.0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1.0, 0.5);
        EFFECT(agent, Hash40::new("sys_assist_out"), Hash40::new("top"), 0, -2.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.25);
        EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("hip"), 0, 0.0, 0, 0, 0, 0, 3.0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {       
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1.0, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
}

//Up Special Ascend Start Sound
unsafe extern "C" fn ssbexo_link_special_hi_ascend_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {        
        STOP_SE(agent, Hash40::new("se_link_special_h01"));
        PLAY_SE(agent, Hash40::new("se_link_special_h02"));
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_link_jump01"));
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_escapeair"));
    }
}

//Up Special Ascend Start Expression
unsafe extern "C" fn ssbexo_link_special_hi_ascend_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {  
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32); 
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);  
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhiascendstart", ssbexo_link_special_hi_ascend_start_acmd, Low)
    .effect_acmd("effect_specialhiascendstart", ssbexo_link_special_hi_ascend_start_effect, Low)
    .sound_acmd("sound_specialhiascendstart", ssbexo_link_special_hi_ascend_start_sound, Low)
    .expression_acmd("expression_specialhiascendstart", ssbexo_link_special_hi_ascend_start_expression, Low)
    .install()
    ;
}