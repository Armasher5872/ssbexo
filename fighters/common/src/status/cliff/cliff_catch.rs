use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_CliffCatch)]
unsafe fn status_pre_cliffcatch(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_ROB) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_CLIFF_ROBBED);
    }
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffCatch)]
unsafe fn status_end_cliffcatch(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CLIFF_CATCH {
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    }
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_cliffcatch,
            status_end_cliffcatch
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}