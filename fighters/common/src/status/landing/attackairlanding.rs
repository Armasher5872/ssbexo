use super::*;

//Status End Landing Attack Air
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_LandingAttackAir)]
unsafe fn status_end_landingattackair(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
    fighter.sub_landing_cancel_damage_face();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_end_landingattackair
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}