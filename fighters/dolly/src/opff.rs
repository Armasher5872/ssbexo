use super::*;

unsafe extern "C" fn dolly_check_special_hi_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat4 = fighter.global_table[CMD_CAT4].get_i32();
    let boma = fighter.module_accessor;
    if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND != 0 && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].clone()).get_bool() {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
            WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
        }
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn dolly_check_super_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat4 = fighter.global_table[CMD_CAT4].get_i32();
    let boma = fighter.module_accessor;
    let is_attack = cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 |*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0;
    let cancel_statuses = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&prev_status_kind);
    let is_feint = cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0 && is_attack && !cancel_statuses;
    let opplr = WorkModule::get_float(boma, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    WorkModule::set_int(boma, fighter.global_table[CMD_CAT4].get_i32(), *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_CAT4_SPECIAL_COMMAND);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0
        && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
            if is_feint {
                WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
            return true.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
        && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL) {
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(), true.into());
            return true.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND != 0
        && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
            if opplr != 0.0 {
                PostureModule::reverse_lr(boma);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
            return true.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND != 0
        && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL) {
            if opplr != 0.0 {
                PostureModule::reverse_lr(boma);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(), true.into());
            return true.into();
        }
    }
    false.into()
}

unsafe extern "C" fn dolly_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat4 = fighter.global_table[CMD_CAT4].get_i32();
    let boma = fighter.module_accessor;
    let is_attack = cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 |*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0;
    let cancel_statuses = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&prev_status_kind);
    let is_feint = cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0 && is_attack && !cancel_statuses;
    if dolly_check_super_special_command(fighter).get_bool() {
        return true.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && dolly_check_special_hi_command(fighter).get_bool() {
        return true.into();
    }
    if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_LW_UNIQ].clone()).get_bool() {
        if is_feint {
            WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
        }
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND.into(), true.into());
        return true.into();
    }
    if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
    && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND.into(), true.into());
        return true.into();
    }
    if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
            if is_feint {
                WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND.into(), true.into());
        }
        return true.into();
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 && is_attack && !cancel_statuses {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), false.into());
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn dolly_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(dolly_check_special_command as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    set_command_input_button(boma, 0x0, 2);
    set_command_input_button(boma, 0x2, 2);
    set_command_input_button(boma, 0x3, 2);
    set_command_input_button(boma, 0x7, 2);
    set_command_input_button(boma, 0x8, 2);
    set_command_input_button(boma, 0x9, 2);
    set_command_input_button(boma, 0xA, 2);
    set_command_input_button(boma, 0xB, 2);
}

pub fn install() {
    Agent::new("dolly")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(dolly_on_start)
    .install()
    ;
}