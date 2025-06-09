use super::*;

pub unsafe extern "C" fn fun_7100021780(fighter: &mut L2CFighterCommon) {
    let catch_motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(catch_motion_kind), 0.0, 1.0, false, 0.0, false, false);
}