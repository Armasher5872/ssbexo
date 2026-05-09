#![allow(improper_ctypes_definitions)] //Addresses warning: `extern` fn uses type `str`, which is not FFI-safe
use super::*;

//Handles angling of moves
pub unsafe extern "C" fn change_angle(boma: *mut BattleObjectModuleAccessor, current_degree: f32, max_degree: f32, motion_kind_max: &str, motion_kind_min: &str) {
    let frame = MotionModule::frame(boma);
    let motion_kind_2nd = MotionModule::motion_kind_2nd(boma);
    let rate = MotionModule::rate(boma);
    let motion = if current_degree <= 0.0 {hash40(motion_kind_min)} else {hash40(motion_kind_max)};
    if motion_kind_2nd != motion {
        if current_degree <= 0.0 {
            MotionModule::add_motion_2nd(boma, Hash40::new(motion_kind_min), frame, rate, true, -(current_degree/max_degree));
            MotionModule::set_weight(boma, 1.0+(current_degree/max_degree), true);
        }
        else {
            MotionModule::add_motion_2nd(boma, Hash40::new(motion_kind_max), frame, rate, true, current_degree/max_degree);
            MotionModule::set_weight(boma, 1.0-(current_degree/max_degree), true);
        }
    }
    else {
        if current_degree < 0.0 {
            MotionModule::set_weight(boma, 1.0+(current_degree/max_degree), true);
        }
        else if current_degree > 0.0 {
            MotionModule::set_weight(boma, 1.0-(current_degree/max_degree), true);
        }
        else {
            MotionModule::set_weight(boma, 1.0, true);
        }
    }
}

//Indicates when moves are off cooldown
pub unsafe extern "C" fn gimmick_flash(fighter: &mut L2CFighterCommon) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let offset = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
    if !sv_information::is_ready_go() {
        return;
    }
    FighterUtil::flash_eye_info(fighter.module_accessor);
    if WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    else {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0*lr, offset, 2, 0, 0, 0, 1.0, true);
    }
    LAST_EFFECT_SET_COLOR(fighter, 0.831, 0.686, 0.216);
}