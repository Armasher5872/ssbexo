use super::*;

//Shadow Flare Cancel ACMD
unsafe extern "C" fn ssbexo_edge_shadow_flare_cancel_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 19.0);
    if ArticleModule::is_exist(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FLAREDUMMY) {
        let flaredummy_boma = get_article_boma(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FLAREDUMMY);
        WorkModule::on_flag(flaredummy_boma, *WEAPON_EDGE_FLARE2_INSTANCE_WORK_ID_FLAG_OWNER_CANCELED);
        if ArticleModule::is_exist(flaredummy_boma, *WEAPON_EDGE_FLAREDUMMY_GENERATE_ARTICLE_FLARE2) {
            let flare2_count = ArticleModule::get_active_num(flaredummy_boma, *WEAPON_EDGE_FLAREDUMMY_GENERATE_ARTICLE_FLARE2);
            for idx in 0..flare2_count {
                let flare2_article = get_article_from_no(flaredummy_boma, *WEAPON_EDGE_FLAREDUMMY_GENERATE_ARTICLE_FLARE2, idx as i32);
                let flare2_battle_object_id = lua_bind::Article::get_battle_object_id(flare2_article);
                if flare2_battle_object_id as i32 != *BATTLE_OBJECT_ID_INVALID {
                    let flare2_boma = sv_battle_object::module_accessor(flare2_battle_object_id as u32);
                    WorkModule::on_flag(flare2_boma, *WEAPON_EDGE_FLARE2_INSTANCE_WORK_ID_FLAG_OWNER_CANCELED);
                }
            }
        }
    }
}

//Shadow Flare Cancel Effect
unsafe extern "C" fn ssbexo_edge_shadow_flare_cancel_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_shadowflare_hold"), Hash40::new("handr"), 1.7, 0, 1.3, 0, 0, 0, 0.6, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_shadowflare_hold"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire3_hold_flash"), Hash40::new("handr"), 1.7, 0.2, 0.6, 0, 0, 0, 0.3, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Shadow Flare Cancel Sound
unsafe extern "C" fn ssbexo_edge_shadow_flare_cancel_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let rand = sv_math::randf(hash40("fighter"), 1.0);
    frame(lua_state, 17.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_edge_special_s02"));
    }
    if rand > 0.8 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_h04"));
        }
    }
    else if rand > 0.7 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_l02"));
        }
    }
    else if rand > 0.6 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_n04"));
        }
    }
    else if rand > 0.5 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_n05"));
        }
    }
    else if rand > 0.4 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_s03"));
        }
    }
    else if rand > 0.3 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_s02"));
        }
    }
    else if rand > 0.2 {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_edge_special_s01"));
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_s03_finger"));
    }
}

//Shadow Flare Cancel Expression
unsafe extern "C" fn ssbexo_edge_shadow_flare_cancel_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attacks"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialscancel", ssbexo_edge_shadow_flare_cancel_acmd, Low)
    .acmd("game_specialairscancel", ssbexo_edge_shadow_flare_cancel_acmd, Low)
    .acmd("effect_specialscancel", ssbexo_edge_shadow_flare_cancel_effect, Low)
    .acmd("effect_specialairscancel", ssbexo_edge_shadow_flare_cancel_effect, Low)
    .acmd("sound_specialscancel", ssbexo_edge_shadow_flare_cancel_sound, Low)
    .acmd("sound_specialairscancel", ssbexo_edge_shadow_flare_cancel_sound, Low)
    .acmd("expression_specialscancel", ssbexo_edge_shadow_flare_cancel_expression, Low)
    .acmd("expression_specialairscancel", ssbexo_edge_shadow_flare_cancel_expression, Low)
    .install()
    ;
}