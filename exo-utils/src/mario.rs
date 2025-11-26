
use super::*;

pub unsafe extern "C" fn mario_change_angle(fighter: &mut L2CFighterCommon, current_degree: f32, max_degree: f32, motion_kind_max: &str, motion_kind_min: &str) {
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind_2nd = MotionModule::motion_kind_2nd(fighter.module_accessor);
    let rate = MotionModule::rate(fighter.module_accessor);
    let motion = if current_degree <= 0.0 {hash40(motion_kind_min)} else {hash40(motion_kind_max)};
    if motion_kind_2nd != motion {
        if current_degree <= 0.0 {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new(motion_kind_min), frame, rate, true, -(current_degree/max_degree));
            MotionModule::set_weight(fighter.module_accessor, 1.0+(current_degree/max_degree), true);
        }
        else {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new(motion_kind_max), frame, rate, true, current_degree/max_degree);
            MotionModule::set_weight(fighter.module_accessor, 1.0-(current_degree/max_degree), true);
        }
    }
    else {
        if current_degree < 0.0 {
            MotionModule::set_weight(fighter.module_accessor, 1.0+(current_degree/max_degree), true);
        }
        else if current_degree > 0.0 {
            MotionModule::set_weight(fighter.module_accessor, 1.0-(current_degree/max_degree), true);
        }
        else {
            MotionModule::set_weight(fighter.module_accessor, 1.0, true);
        }
    }
}