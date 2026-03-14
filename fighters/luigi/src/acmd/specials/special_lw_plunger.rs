use super::*;

//Down Special Plunger ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_plunger_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        SEARCH(agent, 0, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(agent.lua_state_agent, 3.0);
    game_CaptureCutCommon(agent);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
            let plunger_boma = get_article_boma(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER);
            let pos_x = PostureModule::pos_x(agent.module_accessor);
            let pos_y = PostureModule::pos_y(agent.module_accessor);
            let pos_z = PostureModule::pos_z(agent.module_accessor);
            let lr = PostureModule::lr(agent.module_accessor);
            PostureModule::set_pos(plunger_boma, &Vector3f{x: pos_x+(5.0*lr), y: pos_y+6.5, z: pos_z});
        }
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    wait(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Down Special Plunger Effect
unsafe extern "C" fn ssbexo_luigi_down_special_plunger_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Special Plunger Sound
unsafe extern "C" fn ssbexo_luigi_down_special_plunger_sound(_agent: &mut L2CAgentBase) {}

//Down Special Plunger Pull Expression
unsafe extern "C" fn ssbexo_luigi_down_special_plunger_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwplunger", ssbexo_luigi_down_special_plunger_acmd, Low)
    .effect_acmd("effect_speciallwplunger", ssbexo_luigi_down_special_plunger_effect, Low)
    .sound_acmd("sound_speciallwplunger", ssbexo_luigi_down_special_plunger_sound, Low)
    .expression_acmd("expression_speciallwplunger", ssbexo_luigi_down_special_plunger_expression, Low)
    .game_acmd("game_specialairlwplunger", ssbexo_luigi_down_special_plunger_acmd, Low)
    .effect_acmd("effect_specialairlwplunger", ssbexo_luigi_down_special_plunger_effect, Low)
    .sound_acmd("sound_specialairlwplunger", ssbexo_luigi_down_special_plunger_sound, Low)
    .expression_acmd("expression_specialairlwplunger", ssbexo_luigi_down_special_plunger_expression, Low)
    .install()
    ;
}