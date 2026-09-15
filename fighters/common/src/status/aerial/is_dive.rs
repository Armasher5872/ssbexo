/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Sub Is Dive
#[skyline::hook(replace = L2CFighterCommon_sub_is_dive)]
unsafe extern "C" fn sub_is_dive(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let left_stick_y = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_y(boma)} else {ControlModule::get_stick_y(boma)};
    let left_flick_y = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_LEFT_STICK_FLICK_Y)} else {fighter.global_table[FLICK_Y].get_i32()};
    let get_sum_speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let cliff_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
    let cliff_dive_count_max = WorkModule::get_param_int(boma, hash40("common"), 0x189f0b0c96);
    let dive_speed_y = WorkModule::get_param_float(boma, hash40("dive_speed_y"), 0);
    let mut dive_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("dive_cont_value"));
    let mut dive_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("dive_flick_frame_value"));
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
        return false.into();
    }
    if [*FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY].contains(&status_kind_interrupt) {
        return false.into();
    }
    if cliff_count > cliff_dive_count_max {
        return false.into();
    }
    if !KineticModule::is_enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
        return false.into();
    }
    if KineticModule::is_suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
        return false.into();
    }
    if get_sum_speed_y >= 0.0 {
        return false.into();
    }
    if [*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&prev_status_kind) {
        dive_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("cliff_dive_cont_value"));
        dive_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("cliff_dive_flick_frame_value"));
    }
    if left_stick_y > dive_stick_y
    || left_flick_y >= dive_flick_y
    || fighter.global_table[FLICK_Y_DIR].get_i32() >= 0 {
        return false.into();
    }
    (get_sum_speed_y >= -dive_speed_y).into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(sub_is_dive);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}