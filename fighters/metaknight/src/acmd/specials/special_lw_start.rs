use super::*;

unsafe extern "C" fn ssbexo_metaknight_down_special_start_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_lw_start"), false, -1.0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_FLAG_VIS_OFF);
        JostleModule::set_status(agent.module_accessor, false);
        ShieldModule::set_status(agent.module_accessor, *FIGHTER_METAKNIGHT_SHIELD_KIND_SPECIAL_LW_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_METAKNIGHT_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_START_FLAG_VIS_OFF);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_lw_end"), false, -1.0);
        ShieldModule::set_status(agent.module_accessor, *FIGHTER_METAKNIGHT_SHIELD_KIND_SPECIAL_LW_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_METAKNIGHT_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn ssbexo_metaknight_down_special_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_mantle_start"), Hash40::new("top"), 0, 6, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_mantle_end"), Hash40::new("top"), 0, 8, 0, 0, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ssbexo_metaknight_down_special_start_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_metaknight_special_l01"));
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_special_l01"));
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_special_l02"));
    }
}

unsafe extern "C" fn ssbexo_metaknight_down_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_none") as i64);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), false);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_none") as i64);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), false);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), true);
    }
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwstart", ssbexo_metaknight_down_special_start_acmd, Low)
    .game_acmd("game_specialairlwstart", ssbexo_metaknight_down_special_start_acmd, Low)
    .effect_acmd("effect_speciallwstart", ssbexo_metaknight_down_special_start_effect, Low)
    .effect_acmd("effect_specialairlwstart", ssbexo_metaknight_down_special_start_effect, Low)
    .sound_acmd("sound_speciallwstart", ssbexo_metaknight_down_special_start_sound, Low)
    .sound_acmd("sound_specialairlwstart", ssbexo_metaknight_down_special_start_sound, Low)
    .expression_acmd("expression_speciallwstart", ssbexo_metaknight_down_special_start_expression, Low)
    .expression_acmd("expression_specialairlwstart", ssbexo_metaknight_down_special_start_expression, Low)
    .install()
    ;
}