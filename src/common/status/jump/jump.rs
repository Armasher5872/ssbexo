use super::*;

//Status End Jump
#[skyline::hook(replace = L2CFighterCommon_status_end_Jump)]
unsafe fn status_end_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_end_jump
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}