use super::*;

//Ike Reset Initialization
unsafe extern "C" fn ike_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
}

//Ike Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_IKE, 7, false, false))]
unsafe extern "C" fn ike_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
    original!()(vtable, fighter)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_IKE, 4, false, true)).data(ike_reset_initialization as *const () as *const u64);
	skyline::install_hook!(ike_death_initialization);
}