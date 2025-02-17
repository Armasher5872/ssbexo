use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_ShieldBreakFly)]
unsafe extern "C" fn status_pre_shieldbreakfly(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_DAMAGE_FLOAT, (*FS_SUCCEEDS_KEEP_SOUND | *FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_RUMBLE));
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0,  (*FIGHTER_STATUS_ATTR_DISABLE_CURRY_FACE | *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY) as u32, 0, 0);
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_ShieldBreakFly_Main)]
unsafe extern "C" fn status_shieldbreakfly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_STATUS_KIND_FURAFURA_END.into(), false.into());
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_shieldbreakfly,
            status_shieldbreakfly_main
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}