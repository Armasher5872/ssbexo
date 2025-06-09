use super::*;

unsafe extern "C" fn dolly_on_start(fighter: &mut L2CFighterCommon) {
    set_command_input_button(fighter.module_accessor, 0x0, 2);
    set_command_input_button(fighter.module_accessor, 0x2, 2);
    set_command_input_button(fighter.module_accessor, 0x3, 2);
    set_command_input_button(fighter.module_accessor, 0x7, 2);
    set_command_input_button(fighter.module_accessor, 0x8, 2);
    set_command_input_button(fighter.module_accessor, 0x9, 2);
    set_command_input_button(fighter.module_accessor, 0xA, 2);
    set_command_input_button(fighter.module_accessor, 0xB, 2);
}

pub fn install() {
    Agent::new("dolly")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(dolly_on_start)
    .install()
    ;
}