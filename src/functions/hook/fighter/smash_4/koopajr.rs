use super::*;

const KOOPAJR_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xbe37b0; //Bowser Jr only
const KOOPAJR_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xbe37e0; //Bowser Jr only
const KOOPAJR_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xbe3830; //Bowser Jr only
const KOOPAJR_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xbe3d20; //Bowser Jr only

unsafe extern "C" fn koopajr_check_air_jump_aerial_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_aerial = fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) 
    || WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) {
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT) {
            let mut allow_float = false;
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
                    allow_float = !is_aerial;
                }
            }
            if allow_float {
                fighter.change_status(FIGHTER_KOOPAJR_STATUS_KIND_FLOAT.into(), true.into());
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn koopajr_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    }
    0.into()
}

unsafe extern "C" fn koopajr_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
    WorkModule::set_int(boma, 0, FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
}

//Bowser Jr Startup Initialization
#[skyline::hook(offset = KOOPAJR_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopajr_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    koopajr_var(&mut *boma);
    agent.global_table[CHECK_AIR_JUMP_AERIAL_UNIQ].assign(&L2CValue::Ptr(koopajr_check_air_jump_aerial_uniq as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(koopajr_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Bowser Jr Reset Initialization
#[skyline::hook(offset = KOOPAJR_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopajr_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    koopajr_var(&mut *boma);
    original!()(vtable, fighter)
}

//Bowser Jr Death Initialization
#[skyline::hook(offset = KOOPAJR_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopajr_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    koopajr_var(&mut *boma);
    original!()(vtable, fighter)
}

//Bowser Jr Once Per Fighter Frame
#[skyline::hook(offset = KOOPAJR_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn koopajr_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) && WorkModule::is_flag(boma, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END, false);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        koopajr_start_initialization,
        koopajr_reset_initialization,
        koopajr_death_initialization,
        koopajr_opff
    );
}