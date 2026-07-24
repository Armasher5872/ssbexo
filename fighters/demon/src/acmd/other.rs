use super::*;

//Up Taunt ACMD
unsafe extern "C" fn ssbexo_demon_up_taunt_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 55.0);
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    }
    frame(lua_state, 56.0);
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    }
    frame(lua_state, 57.0);
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    }
    frame(lua_state, 58.0);
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    }
    frame(lua_state, 59.0);
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE);
        if MotionModule::motion_kind_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_DEVIL) != hash40("invalid") {
            MotionModule::remove_motion_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_DEVIL, false);
        }
    }
}

//Up Taunt Effect
unsafe extern "C" fn ssbexo_demon_up_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        frame(lua_state, 53.0);
        if is_excute(agent) {
            ColorBlendModule::set_disable_camera_depth_influence(boma, true);
            FLASH(agent, 0.097, 0.006, 0.238, 0.15);
            BURN_COLOR(agent, 6, 0, 40, 0.045);
            FLASH_FRM(agent, 2, 0.097, 0.006, 0.238, 0.45);
            BURN_COLOR_FRAME(agent, 2, 6, 0, 40, 0.13);
        }
        frame(lua_state, 54.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("demon_devil_end"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(lua_state, 56.0);
        if is_excute(agent) {
            FLASH(agent, 0.097, 0.006, 0.238, 0.45);
            BURN_COLOR(agent, 6, 0, 40, 0.13);
            FLASH_FRM(agent, 8, 0.097, 0.006, 0.238, 0);
            BURN_COLOR_FRAME(agent, 8, 6, 0, 40, 0);
        }
    }
}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_demon_up_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 27.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_demon_appeal_h01"));
        PLAY_SE(agent, Hash40::new("vc_demon_appeal01"));
    }
    frame(lua_state, 58.0);
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_demon_spark_end"));
        }
    }
    frame(lua_state, 72.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_demon_appeal_h02"));
    }
}

//Down Taunt ACMD
unsafe extern "C" fn ssbexo_demon_down_taunt_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    frame(lua_state, 22.0);
    FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE);
    }
    frame(lua_state, 25.0);
    FighterSpecializer_Demon::set_devil(boma, true, 3.0);
}

//Down Taunt Effect
unsafe extern "C" fn ssbexo_demon_down_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("demon_devil_sign_flash"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
        FLASH(agent, 0.25, 0.08, 0.6, 0);
        agent.clear_lua_stack();
        lua_args!(agent, -1, 0, 0);
        sv_animcmd::FLASH_SET_DIRECTION(lua_state);
        BURN_COLOR(agent, 8, 3, 36, 0);
        FLASH_FRM(agent, 2, 0.25, 0.08, 0.6, 0.4);
        BURN_COLOR_FRAME(agent, 2, 8, 3, 36, 0.4);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0.4, 0, -0.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("demon_devil_start_appeal"), Hash40::new("top"), 0.4, 0, -0.5, 0, 180, 0, 1, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("demon_devil_start_appeal"), Hash40::new("top"), 0.4, 0, -0.5, 0, 0, 0, 1, true);
        }
    }
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("demon_devil_start_elec"), Hash40::new("bust"), 1, -0.5, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        FLASH(agent, 0.25, 0.08, 0.6, 0.4);
        BURN_COLOR(agent, 8, 3, 36, 0.4);
        FLASH_FRM(agent, 3, 0.25, 0.08, 0.6, 0);
        BURN_COLOR_FRAME(agent, 3, 8, 3, 36, 0);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("demon_devil_start_elec"), false, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("demon_devil_start_elec"), Hash40::new("bust"), 1, -0.5, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("demon_devil_start_elec"), false, true);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("demon_devil_start_elec"), Hash40::new("bust"), 1, -0.5, 0, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("demon_devil_start_elec"), false, true);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("demon_devil_start_elec"), false, true);
    }
    frame(lua_state, 75.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_demon_down_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_demon_appeal03"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_demon_appeal_l01"));
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_appealhil", ssbexo_demon_up_taunt_acmd, Low)
    .acmd("game_appealhir", ssbexo_demon_up_taunt_acmd, Low)
    .acmd("effect_appealhil", ssbexo_demon_up_taunt_effect, Low)
    .acmd("effect_appealhir", ssbexo_demon_up_taunt_effect, Low)
    .acmd("sound_appealhil", ssbexo_demon_up_taunt_sound, Low)
    .acmd("sound_appealhir", ssbexo_demon_up_taunt_sound, Low)
    .acmd("game_appeallwl", ssbexo_demon_down_taunt_acmd, Low)
    .acmd("game_appeallwr", ssbexo_demon_down_taunt_acmd, Low)
    .acmd("effect_appeallwl", ssbexo_demon_down_taunt_effect, Low)
    .acmd("effect_appeallwr", ssbexo_demon_down_taunt_effect, Low)
    .acmd("sound_appeallwl", ssbexo_demon_down_taunt_sound, Low)
    .acmd("sound_appeallwr", ssbexo_demon_down_taunt_sound, Low)
    .install()
    ;
}