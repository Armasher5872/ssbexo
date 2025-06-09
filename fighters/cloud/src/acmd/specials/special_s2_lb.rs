use super::*;

//Limit Break Cross Slash 2 Effect
unsafe extern "C" fn ssbexo_cloud_limit_break_cross_slash_2_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("cloud_kyogiri_stroke1_l"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("cloud_kyogiri_stroke1_r"), true, true);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.79, 1.1, 0.4);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.4);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_yellow"), Hash40::new("tex_cloud_sword2"), 3, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("cloud_kyogiri_stroke2_l"), Hash40::new("top"), 0, 10, 17, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_KYOU_EFFECT);
            sv_animcmd::LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("cloud_kyogiri_stroke2_r"), Hash40::new("top"), 0, 10, 17, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_KYOU_EFFECT);
            sv_animcmd::LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE_OFF(agent, 2);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("cloud_kyogiri_stroke2_l"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("cloud_kyogiri_stroke2_r"), false, false);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_specials2_lb", ssbexo_cloud_limit_break_cross_slash_2_effect, Low)
    .effect_acmd("effect_specialairs2_lb", ssbexo_cloud_limit_break_cross_slash_2_effect, Low)
    .install()
    ;
}