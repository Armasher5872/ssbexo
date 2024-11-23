use super::*;

const METAKNIGHT_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const METAKNIGHT_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const METAKNIGHT_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xd12b90; //Meta Knight only
const METAKNIGHT_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xd12be0; //Meta Knight only

//Meta Knight Startup Initialization
#[skyline::hook(offset = METAKNIGHT_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn metaknight_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_METAKNIGHT as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_initialization_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S);
    }
    original!()(vtable, fighter)
}

//Meta Knight Reset Initialization
#[skyline::hook(offset = METAKNIGHT_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn metaknight_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_METAKNIGHT as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S);
    }
    original!()(vtable, fighter)
}

//Meta Knight Death Initialization
#[skyline::hook(offset = METAKNIGHT_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn metaknight_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S);
    original!()(vtable, fighter)
}

//Meta Knight Once Per Fighter Frame
#[skyline::hook(offset = METAKNIGHT_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn metaknight_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if [*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, *FIGHTER_STATUS_KIND_GLIDE_END].contains(&status_kind) { 
        SoundModule::stop_se(boma, Hash40::new("se_metaknight_glide_start"), 0);
        SoundModule::stop_se(boma, Hash40::new("se_metaknight_glide_loop"), 0);
    };
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        metaknight_start_initialization,
        metaknight_reset_initialization,
        metaknight_death_initialization,
        metaknight_opff
    );
}