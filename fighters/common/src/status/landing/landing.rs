/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Landing_param)]
unsafe extern "C" fn status_pre_landing_param(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue, param_5: L2CValue, param_6: L2CValue) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&prev_status_kind) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP); //Makes wavedashes edge cancelable
    }
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), param_5.get_i32(), *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, param_2.get_i32(), param_3.get_i32(), param_4.get_i32(), param_6.get_i32());
    FighterStatusModuleImpl::set_fighter_status_data(boma, true, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, 0, 0);
    0.into()
}

//Landing Main Sub
#[skyline::hook(replace = L2CFighterCommon_status_Landing_MainSub)]
unsafe extern "C" fn status_landing_main_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let flick_y = fighter.global_table[FLICK_Y].get_i32();
    let boma = fighter.module_accessor;
    let pass_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("pass_flick_y"));
    let get_have_item_kind = ItemModule::get_have_item_kind(boma, 0);
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&prev_status_kind) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH);
        ControlModule::clear_command(boma, true);
        ControlModule::reset_main_stick_x(boma);
        if GroundModule::is_passable_ground(boma) && flick_y < pass_flick_y {
            fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
            return 1.into();
        }
    }
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
        ControlModule::clear_command(boma, true);
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET)
    || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER)
    || get_have_item_kind == *ITEM_KIND_ASSIST {
        if MotionModule::is_end(boma) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
        return 0.into();
    }
    if fighter.sub_landing_uniq_check_strans().get_bool() {
        return 1.into();
    }
    if fighter.sub_landing_ground_check_common().get_bool() {
        return 1.into();
    }
    fighter.sub_landing_cancel_check_damage_face()
}


fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_landing_param,
            status_landing_main_sub
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}