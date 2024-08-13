use super::*;

//Removes the ungrabbability of the mistech state
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_DownWaitContinue)]
unsafe fn status_pre_down_wait_continue(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_DOWN_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_DOWN_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_DOWN_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, 0, *FIGHTER_STATUS_ATTR_SLOPE_TOP_UNLIMIT as u32, 0, 0);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_down_wait_continue
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}