use super::*;

unsafe extern "C" fn ken_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND) 
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());  
        return true.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND) {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2.into(), true.into());  
            return true.into();
        }
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_N_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND.into(), true.into());  
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool()
    && FighterSpecializer_Ryu::check_special_air_s_command(module_accessor) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND.into(), true.into());  
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn ken_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    ken_var(fighter);
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(ken_check_special_command as *const () as _));
    set_command_input_button(boma, SPECIAL_N_COMMAND, 0);
    set_command_input_button(boma, SPECIAL_S_COMMAND, 2);
    set_command_input_button(boma, SPECIAL_HI_COMMAND, 0);
    clone_command_input(boma, SPECIAL_S_COMMAND /*FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND*/, SPECIAL_N2_COMMAND /*FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND*/);
    set_command_input_button(boma, SPECIAL_N2_COMMAND, 0); /*FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND*/
}

pub fn install() {
    Agent::new("ken")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(ken_on_start)
    .install()
    ;
}