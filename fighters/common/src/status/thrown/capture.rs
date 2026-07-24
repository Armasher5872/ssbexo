use super::*;

//Fighter Status Capture Set Invalid Capture. The function that handles grab immunity
#[skyline::hook(replace = L2CFighterCommon_FighterStatusCapture_set_invalid_capture)]
unsafe extern "C" fn fighter_status_capture_set_invalid_capture(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let frame = WorkModule::get_param_int(boma, hash40("common"), hash40("invalid_capture_frame"));
    let invalid_capture_frame = if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_COOLDOWN) <= 0 {frame} else {frame*WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_TIMER_MULTIPLIER)};
    WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_TIMER_MULTIPLIER);
    WorkModule::set_int(boma, 180, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_COOLDOWN);
    WorkModule::set_int(boma, invalid_capture_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_FRAME);
    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CHECK_CATCH);
    EffectModule::req_common(boma, Hash40::new("invalid_capture"), 0.0);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(fighter_status_capture_set_invalid_capture);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}