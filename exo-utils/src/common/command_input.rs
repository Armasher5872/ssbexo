//The functions here are credited to WuBoytH
use super::*;

//Enables characters having command inputs buttons to work properly, and assigns them to a single button type. 0 = Both, 1 = Attack, 2 = Special
pub unsafe extern "C" fn set_command_input_button(boma: *mut BattleObjectModuleAccessor, command: usize, buttons: u8) {
    let buttons = InputAllow::from_bits(buttons).unwrap();
    let control_module = *(boma as *const *const u64).add(0x48/8);
    let command_input = *control_module.add((0x7f0+(command*8))/8) as *mut CommandInputState;
    (*command_input).input_allow = buttons;
}

//Clones a command input to another cat4 flag
pub unsafe extern "C" fn clone_command_input(boma: *mut BattleObjectModuleAccessor, command: usize, replace_command: usize) {
    let control_module = *(boma as *const *const u64).add(0x48/8);
    let original = *control_module.add((0x7f0+(command*8))/8) as *mut CommandInputState;
    let replace = *control_module.add((0x7f0+(replace_command*8))/8) as *mut CommandInputState;
    *replace = *original.clone();
}

//Used to handle custom command inputs
pub unsafe extern "C" fn get_command_stick_direction(boma: &mut BattleObjectModuleAccessor, stick_sensitivity: f32) -> i32 {
    let status_kind = StatusModule::status_kind(boma);
    let stick_x = if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
    let stick_y = if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_y(boma)} else {ControlModule::get_stick_y(boma)};
    let mut lr_stick_x = stick_x*PostureModule::lr(boma);
    if status_kind == *FIGHTER_STATUS_KIND_TURN_RUN {
        lr_stick_x *= -1.0;
    }
    if lr_stick_x >= stick_sensitivity {
        if stick_y <= -stick_sensitivity {
            return 3;
        }
        else if stick_y >= stick_sensitivity {
            return 9;
        }
        else {
            return 6;
        }
    }
    else if lr_stick_x <= -stick_sensitivity {
        if stick_y <= -stick_sensitivity {
            return 1;
        }
        else if stick_y >= stick_sensitivity {
            return 7;
        }
        else {
            return 4;
        }
    }
    else {
        if stick_y <= -stick_sensitivity {
            return 2;
        }
        else if stick_y >= stick_sensitivity {
            return 8;
        }
        else {
            return 5;
        }
    }
}