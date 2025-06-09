use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_set_cliff_xlu_frame)]
unsafe extern "C" fn set_cliff_xlu_frame(fighter: &mut L2CFighterCommon, _motion_kind: &mut L2CValue) {
    WorkModule::set_float(fighter.module_accessor, 30.0, *FIGHTER_STATUS_CLIFF_WORK_FLOAT_HIT_NORMAL_FRAME);
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffEscape)]
unsafe extern "C" fn status_end_cliffescape(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            set_cliff_xlu_frame,
            status_end_cliffescape
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}