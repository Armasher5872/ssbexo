use super::*;

pub unsafe extern "C" fn taunt_hold(fighter: &mut L2CFighterCommon, motion: u64, restart_frame: f32) {
    let boma = fighter.module_accessor;
    let hi_check_on = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI);
    let lw_check_on = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW);
    let hi_check_off = ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_HI);
    let lw_check_off = ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_LW);
    let motion_kind = MotionModule::motion_kind(boma);
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_APPEAL_ENABLE_LOOP) {
        if motion == hash40("appeal_hi_l") {
            if hi_check_on {
                MotionModule::change_motion(boma, Hash40::new("appeal_hi_l_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("appeal_hi_r") {
            if hi_check_on {
                MotionModule::change_motion(boma, Hash40::new("appeal_hi_r_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("appeal_s_l") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                MotionModule::change_motion(boma, Hash40::new("appeal_s_l_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("appeal_s_r") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                MotionModule::change_motion(boma, Hash40::new("appeal_s_r_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("appeal_lw_l") {
            if lw_check_on {
                MotionModule::change_motion(boma, Hash40::new("appeal_lw_l_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("appeal_lw_r") {
            if lw_check_on {
                MotionModule::change_motion(boma, Hash40::new("appeal_lw_r_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_APPEAL_ENABLE_LOOP);
    }
    if motion_kind == hash40("appeal_hi_l_loop") {
        if hi_check_off {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
    if motion_kind == hash40("appeal_hi_r_loop") {
        if hi_check_off {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
    if motion_kind == hash40("appeal_s_l_loop") {
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
    if motion_kind == hash40("appeal_s_r_loop") {
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
    if motion_kind == hash40("appeal_lw_l_loop") {
        if lw_check_off {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
    if motion_kind == hash40("appeal_lw_r_loop") {
        if lw_check_off {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
}