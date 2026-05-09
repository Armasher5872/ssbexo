use super::*;

const KIRBY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xb96350; //Kirby only
const KIRBY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xb96f70; //Kirby only
const KIRBY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xb971d0; //Kirby only

//Kirby Reset Initialization
#[skyline::hook(offset = KIRBY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kirby_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    kirby_var(&mut *boma);
    original!()(vtable, fighter)
}

//Kirby Death Initialization
#[skyline::hook(offset = KIRBY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kirby_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    kirby_var(&mut *boma);
    original!()(vtable, fighter)
}

//Kirby Once Per Fighter Frame
#[skyline::hook(offset = KIRBY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn kirby_once_per_fighter_frame(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    //Kirby Stuff
    if ![
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, 
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].contains(&status_kind) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        kirby_reset_initialization,
        kirby_death_initialization,
        kirby_once_per_fighter_frame
    );
}