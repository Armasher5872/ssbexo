use super::*;

unsafe extern "C" fn ssbexo_metaknight_down_special_end_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_lw_b"), false, -1.0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_BEAM, false, -1);
    }
}

unsafe extern "C" fn ssbexo_metaknight_down_special_end_effect(agent: &mut L2CAgentBase) {
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("metaknight_mantle_airb"), Hash40::new("top"), 0, -3.7, -17.5, 0, 0, 0, 1, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("metaknight_mantle_airb"), Hash40::new("top"), 0, 0, -18, 0, 0, 0, 1, true);
            EffectModule::set_disable_render_offset_last(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_mantle_end"), false, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
}

unsafe extern "C" fn ssbexo_metaknight_down_special_end_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_special_l03"));
    }
}

unsafe extern "C" fn ssbexo_metaknight_down_special_end_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_none") as i64);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwend", ssbexo_metaknight_down_special_end_acmd, Low)
    .game_acmd("game_specialairlwend", ssbexo_metaknight_down_special_end_acmd, Low)
    .effect_acmd("effect_speciallwend", ssbexo_metaknight_down_special_end_effect, Low)
    .effect_acmd("effect_specialairlwend", ssbexo_metaknight_down_special_end_effect, Low)
    .sound_acmd("sound_speciallwend", ssbexo_metaknight_down_special_end_sound, Low)
    .sound_acmd("sound_specialairlwend", ssbexo_metaknight_down_special_end_sound, Low)
    .expression_acmd("expression_speciallwend", ssbexo_metaknight_down_special_end_expression, Low)
    .expression_acmd("expression_specialairlwend", ssbexo_metaknight_down_special_end_expression, Low)
    .install()
    ;
}