use super::*;

//Neutral Special Max Fire ACMD
unsafe extern "C" fn ssbexo_ganon_neutral_special_max_fire_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_VOLLEY, false, -1);
    }
}

//Grounded Neutral Special Max Fire Effect
unsafe extern "C" fn ssbexo_ganon_grounded_neutral_special_max_fire_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, false);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("ganon_volley"), true, true);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    }
    frame(agent.lua_state_agent, 75.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Neutral Special Max Fire Effect
unsafe extern "C" fn ssbexo_ganon_aerial_neutral_special_max_fire_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("footr"), 0, 0, 0, -0.286, -45, 25, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("footl"), 0, 0, 0, -0.286, -45, 25, 1, true);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("ganon_volley"), true, true);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    }
}

//Neutral Special Max Fire Sound
unsafe extern "C" fn ssbexo_ganon_neutral_special_max_fire_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_attackhard_h01"));
    }
}

//Neutral Special Max Fire Expression
unsafe extern "C" fn ssbexo_ganon_neutral_special_max_fire_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        AREA_WIND_2ND_arg10(agent, 0, 2, 75, 2, 1, 0, 12, 50, 30, 50);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_23_hold"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 77.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnfiremax", ssbexo_ganon_neutral_special_max_fire_acmd, Low)
    .effect_acmd("effect_specialnfiremax", ssbexo_ganon_grounded_neutral_special_max_fire_effect, Low)
    .sound_acmd("sound_specialnfiremax", ssbexo_ganon_neutral_special_max_fire_sound, Low)
    .expression_acmd("expression_specialnfiremax", ssbexo_ganon_neutral_special_max_fire_expression, Low)
    .game_acmd("game_specialairnfiremax", ssbexo_ganon_neutral_special_max_fire_acmd, Low)
    .effect_acmd("effect_specialairnfiremax", ssbexo_ganon_aerial_neutral_special_max_fire_effect, Low)
    .sound_acmd("sound_specialairnfiremax", ssbexo_ganon_neutral_special_max_fire_sound, Low)
    .expression_acmd("expression_specialairnfiremax", ssbexo_ganon_neutral_special_max_fire_expression, Low)
    .install()
    ;
}