use super::*;

//Removes the ungrabbability of the mistech state
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_DownDamage)]
unsafe fn status_pre_down_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_DOWN_DAMAGE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_DOWN_DAMAGE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_DOWN_DAMAGE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, 0, (*FIGHTER_STATUS_ATTR_SLOPE_TOP_UNLIMIT | *FIGHTER_STATUS_ATTR_DAMAGE) as u32, 0, 0);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_down_damage
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}