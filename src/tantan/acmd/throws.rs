use super::*;

#[acmd_script( agent = "tantan", scripts = ["game_catchstart", "game_catchdashstart", "game_catchturnstart"], category = ACMD_GAME)]
unsafe fn ssbuexo_tantan_grab_start_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 3.5);
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -12.0, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("throw"), 1.4, 0.0, -1.0, -13.4, Some(0.0), Some(-1.0), Some(0.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -1.5, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("throw"), 1.4, 0.0, -1.0, -2.9, Some(0.0), Some(-1.0), Some(0.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    macros::game_CaptureCutCommon(fighter);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

#[acmd_script( agent = "tantan", scripts = ["game_catchend", "game_catchdashend", "game_catchturnend"], category = ACMD_GAME)]
unsafe fn ssbuexo_tantan_grab_end_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 3.5);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 3, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_NORMAL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_tantan_grab_start_acmd,
        ssbuexo_tantan_grab_end_acmd
    );
}