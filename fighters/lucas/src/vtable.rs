use super::*;

//Lucas Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_LUCAS, 4, false, false))]
unsafe extern "C" fn lucas_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_LASSO_HANG_DATA);
}

//Lucas Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_LUCAS, 7, false, false))]
unsafe extern "C" fn lucas_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        lucas_reset_initialization,
        lucas_death_initialization
    );
}