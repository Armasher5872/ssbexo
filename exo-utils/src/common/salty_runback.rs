//Credit to HDR
use super::*;

pub unsafe fn salty_runback_check(fighter: &mut L2CFighterCommon) -> bool {
    let boma = fighter.module_accessor;
    let button = ControlModule::get_button(boma);
    if IS_SALTY_RUNBACK {
        return false;
    }
    if Buttons::from_bits_retain(button).intersects(Buttons::StockShare) && Buttons::from_bits_retain(button).intersects(Buttons::AppealHi) {
        FighterUtil::flash_eye_info(boma);
        EffectModule::req_follow(boma, Hash40::new("sys_assist_out"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.5, true, 0, 0, 0, 0, 0, false, false);
        trigger_match_reset();
        true
    } 
    else {
        false
    }
}

pub unsafe fn salty_quit_check(fighter: &mut L2CFighterCommon) -> bool {
    let boma = fighter.module_accessor;
    let button = ControlModule::get_button(boma);
    if IS_SALTY_MATCH_EXIT {
        return false;
    }
    if Buttons::from_bits_retain(button).intersects(Buttons::StockShare) && Buttons::from_bits_retain(button).intersects(Buttons::AppealLw) {
        MATCH_EXITING.store(true, std::sync::atomic::Ordering::Relaxed);
        FighterUtil::flash_eye_info(boma);
        EffectModule::req_follow(boma, Hash40::new("sys_assist_out"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.5, true, 0, 0, 0, 0, 0, false, false);
        trigger_match_exit();
        true
    } 
    else {
        false
    }
}