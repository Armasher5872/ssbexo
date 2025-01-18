use super::*;

const KOOPA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const KOOPA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xbc1dd0; //Bowser only
const KOOPA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xbc1e00; //Bowser only

//Bowser Startup Initialization
#[skyline::hook(offset = KOOPA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopa_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_KOOPA as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Bowser Reset Initialization
#[skyline::hook(offset = KOOPA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopa_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Bowser Death Initialization
#[skyline::hook(offset = KOOPA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopa_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        koopa_start_initialization,
        koopa_reset_initialization,
        koopa_death_initialization
    );
}