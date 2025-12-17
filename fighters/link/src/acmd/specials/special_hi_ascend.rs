use super::*;

//Up Special Ascend Effect
unsafe extern "C" fn ssbexo_link_special_hi_ascend_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {        
        let target_y = WorkModule::get_float(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
        let pos_y = PostureModule::pos_y(agent.module_accessor);
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, target_y-pos_y+5.0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1.0, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("hip"), 0, 0.0, 0, 0, 0, 0, 3.0, false);
    }
    for _ in 0..i32::MAX {
        wait(agent.lua_state_agent, 2.0);
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -4, 0, -90, 0, 0, 1.25, true, *EF_FLIP_YZ);
            LAST_PARTICLE_SET_COLOR(agent, 0.25, 1.0, 0.5);
            FLASH(agent, 0.25, 1.0, 0.5, 0.0);
        }
        wait(agent.lua_state_agent, 3.0);
    }
}

//Up Special Ascend Sound
unsafe extern "C" fn ssbexo_link_special_hi_ascend_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {        
        PLAY_STATUS(agent,Hash40::new("se_link_special_h03"));
    }
}

//Up Special Ascend Expression
unsafe extern "C" fn ssbexo_link_special_hi_ascend_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {  
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32); 
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);  
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_specialhiascend", ssbexo_link_special_hi_ascend_effect, Low)
    .sound_acmd("sound_specialhiascend", ssbexo_link_special_hi_ascend_sound, Low)
    .expression_acmd("expression_specialhiascend", ssbexo_link_special_hi_ascend_expression, Low)
    .install()
    ;
}