use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_LandingAttackAir)]
unsafe extern "C" fn status_pre_landing_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, true, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

//ECB related fixes
#[skyline::hook(replace = L2CFighterCommon_status_LandingAttackAirSub)]
unsafe extern "C" fn status_landing_attack_air_sub(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
        if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET) {
            if ItemModule::get_have_item_kind(boma, 0) == *ITEM_KIND_ASSIST {
                return;
            }
            if !StopModule::is_stop(boma) {
                fighter.sub_landing_uniq_check_attack_air();
            }
            fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_landing_uniq_check_attack_air as *const () as _));
        }
    }
    GroundModule::set_offset_y(boma, 0.0);
    GroundModule::set_rhombus_offset(boma, &Vector2f{x: 0.0, y: 0.0});
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_landing_attack_air,
            status_landing_attack_air_sub
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}