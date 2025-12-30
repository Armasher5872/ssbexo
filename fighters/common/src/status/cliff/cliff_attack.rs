use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_end_CliffAttack)]
unsafe extern "C" fn status_end_cliffattack(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y);
    FighterUtil::set_pickelblock_mode_normal(fighter.module_accessor);
    GroundModule::set_ignore_boss(fighter.module_accessor, false);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(
            status_end_cliffattack
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}