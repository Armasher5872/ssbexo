use super::*;

//Palutena Reset Initialization
unsafe extern "C" fn palutena_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Palutena Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_PALUTENA, 7, false, false))]
unsafe extern "C" fn palutena_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    HitModule::set_status_joint_default(boma, Hash40::new("virtualshield"), HitStatus(*HIT_STATUS_OFF), 0);
    hit_module_reset_status_all(boma, 0);
    WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_MENU_MOTION);
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_PALUTENA, 4, false, true)).data(palutena_reset_initialization as *const () as u64);
    skyline::install_hook!(palutena_death_initialization);
}