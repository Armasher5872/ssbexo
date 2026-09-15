use super::*;

//Ryu & Ken Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_RYU, 4, false, false))]
unsafe extern "C" fn ryu_ken_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    if kind == *FIGHTER_KIND_RYU {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI);
    }
    if kind == *FIGHTER_KIND_KEN {
        ken_var(agent);
    }
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Ryu & Ken Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_RYU, 7, false, false))]
unsafe extern "C" fn ryu_ken_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    if kind == *FIGHTER_KIND_RYU {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI);
    }
    if kind == *FIGHTER_KIND_KEN {
        ken_var(agent);
    }
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Ryu & Ken Once Per Fighter Frame
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_RYU, 13, false, false))]
unsafe extern "C" fn ryu_ken_opff(vtable: u64, fighter: &mut Fighter) {
    let ret = original!()(vtable, fighter);
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    if kind == *FIGHTER_KIND_KEN {
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
    ret
}

pub fn install() {
	skyline::install_hooks!(
        ryu_ken_reset_initialization,
        ryu_ken_death_initialization,
        ryu_ken_opff
    );
}