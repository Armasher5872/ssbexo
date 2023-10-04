use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_landing_attack_air_init)]
unsafe fn sub_landing_attack_air_init(fighter: &mut L2CFighterCommon, aerial_motion_kind: L2CValue, aerial_landing_lag: L2CValue, kind: L2CValue) {
    let mot = aerial_motion_kind.get_int();
    let mut motion_rate: f32 = 1.0;
    let mut landing_lag = WorkModule::get_param_float(fighter.module_accessor, aerial_landing_lag.get_int(), 0)+kind.get_f32();
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LAG_REDUCTION) {
        landing_lag -= 3.0;
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_LAG_REDUCTION);
    }
    if landing_lag != 0.0 {
        motion_rate = fighter.sub_get_landing_motion_rate(mot.into(), landing_lag.into()).get_f32();
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, motion_rate, false, 0.0, false, false);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(sub_landing_attack_air_init);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}