use super::*;

unsafe extern "C" fn ken_on_start(fighter: &mut L2CFighterCommon) {
    set_command_input_button(fighter.module_accessor, SPECIAL_N_COMMAND, 0);
    set_command_input_button(fighter.module_accessor, SPECIAL_S_COMMAND, 2);
    set_command_input_button(fighter.module_accessor, SPECIAL_HI_COMMAND, 0);
    clone_command_input(fighter.module_accessor, SPECIAL_S_COMMAND /*FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND*/, SPECIAL_N2_COMMAND /*FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND*/);
    set_command_input_button(fighter.module_accessor, SPECIAL_N2_COMMAND, 0); /*FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND*/
}

pub fn install() {
    Agent::new("ken")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(ken_on_start)
    .install()
    ;
}