/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

unsafe extern "C" fn left_stick_y_flick_counter(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let left_stick_y = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_y(boma)} else {ControlModule::get_stick_y(boma)};
    let prev_left_stick_y = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_prev_y(boma)} else {ControlModule::get_stick_prev_y(boma)};
    if left_stick_y == 0.0 {
        WorkModule::set_int(boma, u8::MAX as i32-1, *FIGHTER_INSTANCE_WORK_ID_INT_LEFT_STICK_FLICK_Y);
    }
    else if left_stick_y.signum() != prev_left_stick_y.signum() || prev_left_stick_y == 0.0 {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_LEFT_STICK_FLICK_Y);
    }
    else {
        WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_LEFT_STICK_FLICK_Y);
    }
}

//Sys Line System Control Fighter. Reserved for common OPFF to be placed on exec status rather than main status
#[skyline::hook(replace = L2CFighterCommon_sys_line_system_control_fighter)]
unsafe fn sys_line_system_control_fighter(fighter: &mut L2CFighterCommon) -> L2CValue {
    left_stick_y_flick_counter(fighter);
    original!()(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(sys_line_system_control_fighter);
    }
}

pub fn install() {
	let _ = skyline::nro::add_hook(nro_hook);
}