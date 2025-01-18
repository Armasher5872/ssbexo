use super::*;

const PURIN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xfdf970; //Jigglypuff only
const PURIN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PURIN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x68d620; //Jigglypuff only

//Jigglypuff Startup Initialization
#[skyline::hook(offset = PURIN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn purin_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_PURIN_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

//Jigglypuff Reset Initialization
#[skyline::hook(offset = PURIN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn purin_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PURIN as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_PURIN_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
    }
    original!()(vtable, fighter)
}

//Jigglypuff Death Initialization
#[skyline::hook(offset = PURIN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn purin_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_PURIN_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
}

pub fn install() {
	skyline::install_hooks!(
        purin_start_initialization,
        purin_reset_initialization,
        purin_death_initialization
    );
}