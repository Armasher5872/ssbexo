use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_Thrown)]
unsafe fn status_end_thrown(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.FighterStatusCapture_set_invalid_capture();
    LinkModule::set_constraint_translate_offset(fighter.module_accessor, &Vector3f::zero());
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_end_thrown
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}