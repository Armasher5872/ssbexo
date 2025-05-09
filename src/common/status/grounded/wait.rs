use super::*;

//Pre Wait Main Param
#[skyline::hook(replace = L2CFighterCommon_status_pre_Wait_main_param)]
unsafe fn status_pre_wait_main_param(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, param_1.get_i32());
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, 0, (*FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST | *FIGHTER_STATUS_ATTR_INTO_DOOR | *FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT) as u32, 0, 0);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_wait_main_param
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}