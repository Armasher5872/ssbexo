use super::*;

const FOX_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xa617c0; //Shared
const FOX_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xa62210; //Shared
const FOX_VTABLE_ON_ATTACK_OFFSET: usize = 0xa64ce0; //Shared

//Fox Reset Initialization
#[skyline::hook(offset = FOX_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn fox_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_FOX as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT);
    }
    original!()(vtable, fighter)
}

//Fox Death Initialization
#[skyline::hook(offset = FOX_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn fox_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_FOX as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT);
    }
    original!()(vtable, fighter)
}

//Fox On Attack
#[skyline::hook(offset = FOX_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn fox_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_FOX as u32 {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            WorkModule::on_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT);
        }
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
    skyline::install_hooks!(
        fox_reset_initialization,
        fox_death_initialization,
        fox_on_attack
    );
}