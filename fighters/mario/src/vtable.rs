use super::*;

const MARIO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const MARIO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcb9730; //Mario only
const MARIO_VTABLE_ON_ATTACK_OFFSET: usize = 0x68d7e0; //Shared

//Mario Reset Initialization
#[skyline::hook(offset = MARIO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mario_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARIO as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        mario_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Mario Death Initialization
#[skyline::hook(offset = MARIO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mario_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    mario_var(&mut *boma);
    original!()(vtable, fighter)
}

//Mario On Attack
#[skyline::hook(offset = MARIO_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn mario_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARIO as u32 {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP].contains(&status_kind) {
            WorkModule::on_flag(boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT);
        }
    }
    call_original!(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        mario_reset_initialization,
        mario_death_initialization,
        mario_on_attack
    );
}