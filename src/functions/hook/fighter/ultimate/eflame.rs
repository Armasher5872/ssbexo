use super::*;

const EFLAME_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xa0b890; //Pyra only
const EFLAME_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xa0b8a0; //Pyra only
const EFLAME_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xa0bce0; //Pyra only
const EFLAME_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xa0c010; //Pyra only
const EFLAME_VTABLE_ON_ATTACK_OFFSET: usize = 0xa0cec0; //Pyra only

//Pyra Startup Initialization
#[skyline::hook(offset = EFLAME_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn eflame_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

//Pyra Reset Initialization
#[skyline::hook(offset = EFLAME_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn eflame_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    original!()(vtable, fighter)
}

//Pyra Death Initialization
#[skyline::hook(offset = EFLAME_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn eflame_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    original!()(vtable, fighter)
}

//Pyra Once Per Fighter Frame
#[skyline::hook(offset = EFLAME_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn eflame_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
    && fighter.battle_object.is_cat_flag(Cat1::SpecialLw) {
        WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
    }
    original!()(vtable, fighter)
}

//Pyra On Attack
#[skyline::hook(offset = EFLAME_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn eflame_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    if (status_kind == *FIGHTER_STATUS_KIND_ATTACK && motion_kind == hash40("attack_13"))
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
    || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && [hash40("attack_air_f"), hash40("attack_air_b")].contains(&motion_kind)) {
        WorkModule::on_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    }
    call_original!(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        eflame_start_initialization,
        eflame_reset_initialization,
        eflame_death_initialization,
        eflame_opff,
        eflame_on_attack
    );
}