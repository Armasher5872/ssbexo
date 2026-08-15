use super::*;

unsafe extern "C" fn demon_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat4 = fighter.global_table[CMD_CAT4].get_i32();
    let boma = fighter.module_accessor;
    let is_attack = cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 |*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0;
    let rage_system = WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM);
    if situation_kind == *SITUATION_KIND_AIR {
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S)
    || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND /*623B*/ != 0 {
            if is_attack {
                if rage_system {
                    fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE.into(), true.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
                }
            }
            else {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2H.into(), true.into());
            }
            return 1.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623BLONG != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S.into(), true.into());
            return 1.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG != 0 {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0 && rage_system {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE.into(), true.into());
            }
            else {
                WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ELECTRIC_DRAGON_UPPERCUT);
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L.into(), true.into());
            }
            return 1.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F.into(), true.into());
            return 1.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2.into(), true.into());
            return 1.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623NB != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB) {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP.into(), false.into());
                return 1.into();
            }
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6AB != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND) {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1.into(), true.into());
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn demon_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    demon_var(&mut *boma);
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(demon_check_special_command as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(demon_on_start)
    .install()
    ;
}