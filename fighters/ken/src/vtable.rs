use super::*;

const KEN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x10d4570; //Shared
const KEN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10d4620; //Shared
const KEN_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x10d5f30; //Shared

//Ken Reset Initialization
#[skyline::hook(offset = KEN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ken_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_KEN as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        let control_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        *(control_energy as *mut u8).add(0xa4) = 1;
        common_reset_variable_reset(&mut *boma);
        ken_var(agent);
    }
    original!()(vtable, fighter)
}

//Ken Death Initialization
#[skyline::hook(offset = KEN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ken_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_KEN as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_death_variable_reset(&mut *boma);
        ken_var(agent);
    }
    original!()(vtable, fighter)
}

//Ken Once Per Fighter Frame
#[skyline::hook(offset = KEN_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn ken_opff(vtable: u64, fighter: &mut Fighter) {
    original!()(vtable, fighter);
    if fighter.battle_object.kind == *FIGHTER_KIND_KEN as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        let status_kind = agent.global_table[STATUS_KIND].get_i32();
        let situation_kind = agent.global_table[SITUATION_KIND].get_i32();
        let command_input_timer = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
        let attack_command1_counter = WorkModule::get_int(boma, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
        let stick_direction = get_command_stick_direction(&mut *boma, 0.2);
        if situation_kind == *SITUATION_KIND_GROUND
        && status_kind != *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1 {
            if command_input_timer != 0 {
                WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
            }
            else {
                WorkModule::set_int(boma, 0, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
            }
            if stick_direction == 6
            && attack_command1_counter == 0 {
                WorkModule::set_int(boma, 10, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
                WorkModule::inc_int(boma, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
            }
            if stick_direction == 3
            && attack_command1_counter == 1 {
                WorkModule::set_int(boma, 10, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
                WorkModule::inc_int(boma, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
            }
            if stick_direction == 2
            && attack_command1_counter == 2 {
                WorkModule::set_int(boma, 10, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
                WorkModule::inc_int(boma, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
            }
            if stick_direction == 1
            && attack_command1_counter == 3 {
                WorkModule::set_int(boma, 10, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
                WorkModule::inc_int(boma, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
            }
            if stick_direction == 4
            && attack_command1_counter == 4 {
                WorkModule::set_int(boma, 10, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
                WorkModule::inc_int(boma, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
                WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            }
            if stick_direction == 4
            && (attack_command1_counter >= 2 && attack_command1_counter < 5) {
                WorkModule::set_int(boma, 10, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
                WorkModule::set_int(boma, 5, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
            }
            if attack_command1_counter == 5
            && (
                ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) 
                || ControlModule::check_button_release(boma, *CONTROL_PAD_BUTTON_ATTACK) 
                || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)
            ) {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                agent.change_status(FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1.into(), true.into());
            }
        }
        else {
            if command_input_timer != 0 {
                WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER);
            }
            WorkModule::set_int(boma, 0, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        ken_reset_initialization,
        ken_death_initialization,
        ken_opff
    );
}