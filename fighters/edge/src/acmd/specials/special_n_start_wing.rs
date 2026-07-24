use super::*;

//Grounded Teraflare ACMD
unsafe extern "C" fn ssbexo_edge_grounded_teraflare_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_S, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_M, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 79.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 99.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 100.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_L, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 190.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_XL, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 210.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(lua_state, 230.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}

//Aerial Teraflare ACMD
unsafe extern "C" fn ssbexo_edge_aerial_teraflare_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_S, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_M, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 79.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 99.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 100.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_L, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 120.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0.0, 2.5);
    }
    frame(lua_state, 190.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_XL, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 210.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(lua_state, 230.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}

//Teraflare Effect
unsafe extern "C" fn ssbexo_edge_teraflare_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire1_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 10, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 6.0);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_fire1_hold"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    for _ in 0..6 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 10, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 6.0);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("arml"), 3, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    for _ in 0..6 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 10, 0, 4, 0, 0, 0, false);
        }
        wait(lua_state, 5.0);
    }
    frame(lua_state, 100.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_fire2_hold"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire3_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire3_hold_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire3_trigger"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 102.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("arml"), 3, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 118.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_fire2_hold_aura"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("arml"), 3, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 10, 0, 4, 0, 0, 0, false);
            EFFECT_FOLLOW(agent, Hash40::new("edge_shadowflare_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.2, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 1.0);
        }
        wait(lua_state, 6.0);
    }
    frame(lua_state, 148.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire3_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    for _ in 0..6 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 10, 0, 4, 0, 0, 0, false);
            EFFECT_FOLLOW(agent, Hash40::new("edge_shadowflare_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.2, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 1.0);
        }
        wait(lua_state, 6.0);
    }
    frame(lua_state, 190.0);
    if is_excute(agent) {
        EffectModule::req_screen(boma, Hash40::new("edge_fire3_screen1"), false, false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("edge_fire3_hold"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("edge_fire3_flash"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire4_hold_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 192.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("arml"), 3, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire4_trigger"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 200.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_fire2_hold_aura"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("arml"), 3, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 210.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

//Teraflare Sound
unsafe extern "C" fn ssbexo_edge_teraflare_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_edge_rnd_attack_special_n_start"));
    }
    frame(lua_state, 209.0);
    if is_excute(agent) {
        let rand = sv_math::randf(hash40("fighter"), 1.0);
        if rand > 0.75 {
            PLAY_SE(agent, Hash40::new("vc_edge_special_n06"));
        }
        else if rand > 0.5 {
            PLAY_SE(agent, Hash40::new("vc_edge_special_l01"));
        }
        else if rand > 0.25 {
            PLAY_SE(agent, Hash40::new("vc_edge_winged03"));
        }
        else {
            PLAY_SE(agent, Hash40::new("vc_edge_special_h05"));
        }
        PLAY_SE(agent, Hash40::new("se_edge_special_n05_01"));
    }
}

//Teraflare Expression
unsafe extern "C" fn ssbexo_edge_teraflare_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackss"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackss"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackss"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackss"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 45.0);
    for _ in 0..5 {
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 8.0);
    }
    frame(lua_state, 85.0);
    for _ in 0..15 {
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 6.0);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 6.0);
    }
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnstartwing", ssbexo_edge_grounded_teraflare_acmd, Low)
    .acmd("game_specialairnstartwing", ssbexo_edge_aerial_teraflare_acmd, Low)
    .acmd("effect_specialnstartwing", ssbexo_edge_teraflare_effect, Low)
    .acmd("effect_specialairnstartwing", ssbexo_edge_teraflare_effect, Low)
    .acmd("sound_specialnstartwing", ssbexo_edge_teraflare_sound, Low)
    .acmd("sound_specialairnstartwing", ssbexo_edge_teraflare_sound, Low)
    .acmd("expression_specialnstartwing", ssbexo_edge_teraflare_expression, Low)
    .acmd("expression_specialairnstartwing", ssbexo_edge_teraflare_expression, Low)
    .install()
    ;
}