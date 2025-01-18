use super::*;

const RYU_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const RYU_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x10d4570; //Shared
const RYU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10d4620; //Shared

//Ryu Startup Initialization
#[skyline::hook(offset = RYU_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ryu_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_RYU as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Ryu Reset Initialization
#[skyline::hook(offset = RYU_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ryu_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_RYU as u32 {
        let boma = fighter.battle_object.module_accessor;
        let control_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        *(control_energy as *mut u8).add(0xa4) = 1;
        common_reset_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI);
    }
    original!()(vtable, fighter)
}

//Ryu Death Initialization
#[skyline::hook(offset = RYU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ryu_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_RYU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        ryu_start_initialization,
        ryu_reset_initialization,
        ryu_death_initialization
    );
}