use super::*;

//Glide Start ACMD
unsafe extern "C" fn ssbexo_metaknight_glide_start_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, 0);
    }
}

//Glide Start Effect
unsafe extern "C" fn ssbexo_metaknight_glide_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -5.3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 6.4, true);
        LAST_EFFECT_SET_COLOR(agent, 0.68, 0.87, 2.0);
    }
}

//Glide Start Sound
unsafe extern "C" fn ssbexo_metaknight_glide_start_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_jump04"));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_metaknight_appeal_s03"));
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_metaknight_glide_start"));
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_metaknight_glide_loop"));
    }
}

//Glide Wing Effect
unsafe extern "C" fn ssbexo_metaknight_glide_wing_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_aura_light"), false, false);
    }
}

//Glide Attack ACMD
unsafe extern "C" fn ssbexo_metaknight_glide_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 70, 80, 0, 30, 8.0, 7.0, 11.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 70, 80, 0, 30, 11.0, 0.0, 7.5, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 70, 80, 0, 30, 8.0, -7.0, 3.5, 9.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Glide Attack Effect
unsafe extern "C" fn ssbexo_metaknight_glide_attack_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_air_lw"), Hash40::new("top"), 0.0, 8.5, 0, -41.3, 28.5, -6.0, 1.9, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
}

//Glide Attack Sound
unsafe extern "C" fn ssbexo_metaknight_glide_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_attack100_03"));
    }
}

//Glide Landing Effect
unsafe extern "C" fn ssbexo_metaknight_glide_landing_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

//Glide Landing Sound
unsafe extern "C" fn ssbexo_metaknight_glide_landing_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_metaknight_glide_start"));
        STOP_SE(agent, Hash40::new("se_metaknight_glide_loop"));
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_DOWN_SE(agent, Hash40::new("se_common_down_soil_s"));
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        PLAY_DOWN_SE(agent, Hash40::new("se_common_down_soil_ss"));
    }
}

//Glide End Sound
unsafe extern "C" fn ssbexo_metaknight_glide_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_metaknight_glide_start"));
        STOP_SE(agent, Hash40::new("se_metaknight_glide_loop"));
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_dash_turn"));
    }
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_glidestart", ssbexo_metaknight_glide_start_acmd, Low)
    .effect_acmd("effect_glidestart", ssbexo_metaknight_glide_start_effect, Low)
    .sound_acmd("sound_glidestart", ssbexo_metaknight_glide_start_sound, Low)
    .effect_acmd("effect_glidewing", ssbexo_metaknight_glide_wing_effect, Low)
    .game_acmd("game_glideattack", ssbexo_metaknight_glide_attack_acmd, Low)
    .effect_acmd("effect_glideattack", ssbexo_metaknight_glide_attack_effect, Low)
    .sound_acmd("sound_glideattack", ssbexo_metaknight_glide_attack_sound, Low)
    .effect_acmd("effect_glidelanding", ssbexo_metaknight_glide_landing_effect, Low)
    .sound_acmd("sound_glidelanding", ssbexo_metaknight_glide_landing_sound, Low)
    .sound_acmd("sound_glideend", ssbexo_metaknight_glide_end_sound, Low)
    .install()
    ;
}