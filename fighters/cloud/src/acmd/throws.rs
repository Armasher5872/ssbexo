use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_cloud_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        MotionModule::set_rate(agent.module_accessor, 1.3);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_cloud_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        MotionModule::set_rate(agent.module_accessor, 1.2);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_cloud_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        MotionModule::set_rate(agent.module_accessor, 1.18);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Forward Throw ACMD
unsafe extern "C" fn ssbexo_cloud_forward_throw_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 40, 125, 0, 48, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footl"), 4.0, 45, 120, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        CHECK_FINISH_CAMERA(agent, 15, 4);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catch", ssbexo_cloud_grab_acmd, Low)
    .game_acmd("game_catchdash", ssbexo_cloud_dash_grab_acmd, Low)
    .game_acmd("game_catchturn", ssbexo_cloud_pivot_grab_acmd, Low)
    .game_acmd("game_throwf", ssbexo_cloud_forward_throw_acmd, Low)
    .install()
    ;
}