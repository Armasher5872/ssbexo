use super::*;

//Grab ACMD
#[acmd_script( agent = "samus", script = "game_catch", category = ACMD_GAME)]
unsafe fn ssbuexo_samus_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//Dash Grab ACMD
#[acmd_script( agent = "samus", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn ssbuexo_samus_dash_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//Pivot Grab ACMD
#[acmd_script( agent = "samus", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn ssbuexo_samus_pivot_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//Down Throw ACMD
#[acmd_script( agent = "samus", script = "game_throwlw", category = ACMD_GAME)]
unsafe fn ssbuexo_samus_down_throw_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 65, 75, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 17.0, 0.0);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(fighter.lua_state_agent, 25.0);
    macros::FT_MOTION_RATE(fighter, 0.85);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_ARTICLE_MOTION_RATE_SYNC);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_samus_grab_acmd,
        ssbuexo_samus_dash_grab_acmd,
        ssbuexo_samus_pivot_grab_acmd,
        ssbuexo_samus_down_throw_acmd
    );
}