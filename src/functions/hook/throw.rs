use super::*;

//Fix throws not respecting the cstick, especially dk cargo throw
#[skyline::hook(replace = L2CFighterCommon_IsThrowStick)]
unsafe extern "C" fn is_throw_stick(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut out = fighter.local_func__fighter_status_catch_1();
    let stick_x = fighter.stick_x() * PostureModule::lr(fighter.boma());
    let stick_y = fighter.stick_y();
    if stick_x > fighter.get_param_float("common", "attack_lw3_stick_x") {
        out["f"] = true.into();
    } else if stick_x < -fighter.get_param_float("common", "attack_lw3_stick_x") {
        out["b"] = true.into();
    }
    if stick_y > fighter.get_param_float("common", "attack_hi4_stick_y") {
        out["hi"] = true.into();
    } else if stick_y < fighter.get_param_float("common", "attack_lw4_stick_y") {
        out["lw"] = true.into();
    }
    out
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(is_throw_stick);
    }
}

pub fn install() {
	skyline::nro::add_hook(nro_hook).unwrap();
}