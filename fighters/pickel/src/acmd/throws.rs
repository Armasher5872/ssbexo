use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_pickel_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch"), false, -1.0);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ArticleModule::set_visibility(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -4.0, -2.5, Some(0.0), Some(-4.8), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -4.0, -2.5, Some(0.0), Some(-4.8), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 1.8, 0.0, 7.0, 3.2, Some(0.0), Some(7.0), Some(10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 2, Hash40::new("top"), 3.6, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        GrabModule::clear(agent.module_accessor, 1);
        GrabModule::clear(agent.module_accessor, 2);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.0), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(-0.5), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(-0.5), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(0.0), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(0.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        ArticleModule::set_flag(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(agent.lua_state_agent, 45.0);
    FT_MOTION_RATE(agent, 2.0);
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_pickel_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch_dash"), false, -1.0);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ArticleModule::set_visibility(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.8), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.8), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 1.8, 0.0, 7.0, 3.2, Some(0.0), Some(7.0), Some(10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 2, Hash40::new("top"), 3.6, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        GrabModule::clear(agent.module_accessor, 1);
        GrabModule::clear(agent.module_accessor, 2);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.0), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(-0.5), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(-0.5), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(0.0), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(0.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);   
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        ArticleModule::set_flag(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(agent.lua_state_agent, 51.0);
    FT_MOTION_RATE(agent, 2.0);
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_pickel_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch_turn"), false, -1.0);
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ArticleModule::set_visibility(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.8), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, 2.5, Some(0.0), Some(-4.8), Some(2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 1.8, 0.0, 7.0, -3.2, Some(0.0), Some(7.0), Some(-10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 2, Hash40::new("top"), 3.6, 0.0, 7.0, -5.0, Some(0.0), Some(7.0), Some(-9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        GrabModule::clear(agent.module_accessor, 1);
        GrabModule::clear(agent.module_accessor, 2);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.0), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(-0.5), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(-0.5), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 0.0, 0, 0, 0, 0, 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(0.0), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        CATCH(agent, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(0.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);   
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        ArticleModule::set_flag(agent.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(agent.lua_state_agent, 52.0);
    FT_MOTION_RATE(agent, 2.0);
}

pub fn install() {
    Agent::new("pickel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catch", ssbexo_pickel_grab_acmd, Low)
    .game_acmd("game_catchdash", ssbexo_pickel_dash_grab_acmd, Low)
    .game_acmd("game_catchturn", ssbexo_pickel_pivot_grab_acmd, Low)
    .install()
    ;
}