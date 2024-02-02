use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Squat_param)]
unsafe fn status_pre_squat_param(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue, param_5: L2CValue) -> L2CValue {
    if !fighter.sub_pre_fall().get_bool() {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
            let get_have_item_kind = ItemModule::get_have_item_kind(fighter.module_accessor, 0);
            if get_have_item_kind == *ITEM_KIND_ASSIST {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST);
                return 1.into();
            }
            StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), param_4.get_i32(), *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, param_1.get_i32(), param_2.get_i32(), param_3.get_i32(), param_5.get_i32());
            FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, 0, (*FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32, 0, 0);
            return 0.into();
        }
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_HAMMER_WAIT);
    }
    1.into()
}
   
fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_squat_param
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}