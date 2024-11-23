use super::*;

const KEN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const KEN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x10d4570; //Shared
const KEN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10d4620; //Shared

//Ken Startup Initialization
#[skyline::hook(offset = KEN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ken_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_KEN as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_initialization_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_CAN_KARA_CANCEL);
    }
    original!()(vtable, fighter)
}

//Ken Reset Initialization
#[skyline::hook(offset = KEN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ken_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_KEN as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_CAN_KARA_CANCEL);
    }
    original!()(vtable, fighter)
}

//Ken Death Initialization
#[skyline::hook(offset = KEN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ken_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_KEN as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_CAN_KARA_CANCEL);
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        ken_start_initialization,
        ken_reset_initialization,
        ken_death_initialization
    );
}