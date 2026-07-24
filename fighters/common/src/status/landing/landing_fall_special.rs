use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_landing_fall_special_common)]
unsafe extern "C" fn status_pre_landing_fall_special_common(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), fighter.sub_pre_landing_kinetic_type().get_i32(), *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, param_2.get_i32(), param_3.get_i32(), param_4.get_i32(), 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, true, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, 0, 0);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(status_pre_landing_fall_special_common);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}