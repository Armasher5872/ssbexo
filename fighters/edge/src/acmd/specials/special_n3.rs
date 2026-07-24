use super::*;

//Gigaflare ACMD
unsafe extern "C" fn ssbexo_edge_gigaflare_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.6);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}

//Gigaflare Effect
unsafe extern "C" fn ssbexo_edge_gigaflare_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        EffectModule::req_screen(boma, Hash40::new("edge_fire3_screen1"), false, false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("edge_fire2_hold"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire3_hold_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("arml"), 3, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("arml"), 3, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire3_trigger"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_fire2_hold_aura"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("arml"), 3, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire2_hold_aura_end"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

//Gigaflare Sound
unsafe extern "C" fn ssbexo_edge_gigaflare_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_edge_rnd_attack_special_n03"));
        PLAY_SE(agent, Hash40::new("se_edge_special_n04_01"));
    }
}

//Gigaflare Expression
unsafe extern "C" fn ssbexo_edge_gigaflare_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    for _ in 0..3 {
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
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
    .acmd("game_specialn3", ssbexo_edge_gigaflare_acmd, Low)
    .acmd("game_specialairn3", ssbexo_edge_gigaflare_acmd, Low)
    .acmd("effect_specialn3", ssbexo_edge_gigaflare_effect, Low)
    .acmd("effect_specialairn3", ssbexo_edge_gigaflare_effect, Low)
    .acmd("sound_specialn3", ssbexo_edge_gigaflare_sound, Low)
    .acmd("sound_specialairn3", ssbexo_edge_gigaflare_sound, Low)
    .acmd("expression_specialn3", ssbexo_edge_gigaflare_expression, Low)
    .acmd("expression_specialairn3", ssbexo_edge_gigaflare_expression, Low)
    .install()
    ;
}