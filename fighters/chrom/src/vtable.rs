use super::*;

const CHROM_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x10bb480; //Shared
const CHROM_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const CHROM_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10bb700; //Shared

//Chrom Startup Initialization
#[skyline::hook(offset = CHROM_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn chrom_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_CHROM as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Chrom Reset Initialization
#[skyline::hook(offset = CHROM_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn chrom_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_CHROM as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Chrom Death Initialization
#[skyline::hook(offset = CHROM_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn chrom_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_CHROM as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        chrom_start_initialization,
        chrom_reset_initialization,
        chrom_death_initialization
    );
}