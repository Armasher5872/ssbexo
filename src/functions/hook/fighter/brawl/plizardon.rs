use super::*;

const PLIZARDON_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xf93940; //Charizard only
const PLIZARDON_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PLIZARDON_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf93b20; //Charizard only
const PLIZARDON_VTABLE_RESPAWN_INITIALIZATION_OFFSET: usize = 0xf96330; //Shared

//Charizard Startup Initialization
#[skyline::hook(offset = PLIZARDON_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn plizardon_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    agent.global_table[THROW_HI_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
    original!()(vtable, fighter)
}

//Charizard Reset Initialization
#[skyline::hook(offset = PLIZARDON_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn plizardon_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PLIZARDON as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Charizard Death Initialization
#[skyline::hook(offset = PLIZARDON_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn plizardon_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Charizard Respawn Initialization
#[skyline::hook(offset = PLIZARDON_VTABLE_RESPAWN_INITIALIZATION_OFFSET)]
unsafe extern "C" fn plizardon_respawn_initialization(_vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PLIZARDON as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
}

pub fn install() {
    skyline::install_hooks!(
        plizardon_start_initialization,
        plizardon_reset_initialization,
        plizardon_death_initialization,
        plizardon_respawn_initialization
    );
}