use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CapturePulled_Main)]
unsafe fn status_capturepulled_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.CapturePulledCommon_Main();
    //Fighter Specific
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION) {
        WorkModule::sub_float(fighter.module_accessor, 34.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CAPTURE_WAIT) {
        if !fighter.is_capture_pulled_special_fighter().get_bool() {
            if !MotionModule::is_end(fighter.module_accessor) {
                if !fighter.is_capture_pulled_special_fighter().get_bool() {
                    return 0.into();
                }
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_MOTION_END_FLAG);
            }
            fighter.change_status(FIGHTER_STATUS_KIND_CAPTURE_WAIT.into(), false.into());
        }
        else {
            if !MotionModule::is_end(fighter.module_accessor)
            && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_MOTION_END_FLAG) {
                if !fighter.is_capture_pulled_special_fighter().get_bool() {
                    return 0.into();
                }
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_MOTION_END_FLAG);
            }
            fighter.change_status(FIGHTER_STATUS_KIND_CAPTURE_WAIT.into(), false.into());
        }
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CapturePulled)]
unsafe fn status_end_capturepulled(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    //Fighter Specific
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_capturepulled_main,
            status_end_capturepulled
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}