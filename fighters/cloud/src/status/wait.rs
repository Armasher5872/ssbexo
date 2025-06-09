use super::*;

unsafe extern "C" fn cloud_wait_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_wait_common();
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        fighter.sub_wait_motion_mtrans();
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("punish_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_wait_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
        fighter.change_status(FIGHTER_STATUS_KIND_HAMMER_WAIT.into(), false.into());
    }
    else {
        if !fighter.sub_wait_common_Main().get_bool() {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
                fighter.sub_wait_motion(true.into());
            }
            else {
                if MotionModule::is_end(fighter.module_accessor) {
                    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 0.0, true, false, false);
                }
            }
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAMON_EXHAUST) {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_METAMON_OUT.into(), false.into());
        }
    }
    1.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_WAIT, cloud_wait_main_status)
    .install()
    ;
}