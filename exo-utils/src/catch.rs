#![allow(improper_ctypes_definitions)] //Addresses `extern` fn uses type `str`, which is not FFI-safe
use super::*;

//Gets the boma of the grabbed opponent
pub unsafe fn get_grabbed_opponent_boma(attacker: *mut BattleObjectModuleAccessor) -> &'static mut BattleObjectModuleAccessor {
    let opponent_id = LinkModule::get_node_object_id(attacker, *LINK_NO_CAPTURE) as u32;
    let opponent_object = get_battle_object_from_id(opponent_id);
    return &mut *(*opponent_object).module_accessor
}

//Gets the necessary grab animation for throws
pub unsafe extern "C" fn grabbed_anim_selector(fighter: &mut L2CFighterCommon, anim_name: &str, set_frame: f32, mot_rate: f32) {
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if capture_id as i32 != *BATTLE_OBJECT_ID_INVALID {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        let motion_share = WorkModule::get_param_int(capture_boma, hash40("param_motion"), hash40("motion_share"));
        let mut motion = hash40(anim_name);
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_DX);
        }
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_GIRL);
        }
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_BIG);
        }
        MotionModule::change_motion(capture_boma, Hash40::new_raw(motion), set_frame, mot_rate, false, 0.0, false, false);
    }
}