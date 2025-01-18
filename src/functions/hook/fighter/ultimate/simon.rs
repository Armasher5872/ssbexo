use super::*;

const SIMON_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x1194120; //Shared
const SIMON_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1194130; //Shared
const SIMON_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11944e0; //Shared

//Simon Startup Initialization
#[skyline::hook(offset = SIMON_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn simon_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_SIMON as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Simon Reset Initialization
#[skyline::hook(offset = SIMON_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn simon_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_SIMON as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma); 
    }
    original!()(vtable, fighter)
}

//Simon Death Initialization
#[skyline::hook(offset = SIMON_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn simon_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_SIMON as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma); 
    }
    original!()(vtable, fighter)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x1195224).data(0x7100001F); //Credited to HDR, makes it so Simon/Richter only enter the Cross Catch animation if they are idle
	skyline::install_hooks!(
        simon_start_initialization,
        simon_reset_initialization,
        simon_death_initialization
    );
}