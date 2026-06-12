use super::*;

unsafe extern "C" fn demon_attack_combo_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fun_7100031280(fighter, 0.into());
    MotionModule::set_trans_move_speed_no_scale(boma, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_combo_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100031280(fighter: &mut L2CFighterCommon, combo_index: L2CValue) {
    let boma = fighter.module_accessor;
    let motion_kind;
    WorkModule::set_int(boma, combo_index.get_i32(), *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_COMBO);
    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_NEXT_STATUS);
    if combo_index.get_i32() != 0 {
        if combo_index.get_i32() != 1 {
            if combo_index.get_i32() != 2 {
                if combo_index.get_i32() != 3 {
                    if combo_index.get_i32() != 4 {
                        if combo_index.get_i32() != 5 {
                            if combo_index.get_i32() != 6 {
                                if combo_index.get_i32() != 7 {
                                    if combo_index.get_i32() != 8 {
                                        motion_kind = Hash40::new("attack_110");
                                    }
                                    else {
                                        motion_kind = Hash40::new("attack_19");
                                    }
                                }
                                else {
                                    motion_kind = Hash40::new("attack_18");
                                }
                            }
                            else {
                                motion_kind = Hash40::new("attack_17");
                            }
                        }
                        else {
                            motion_kind = Hash40::new("attack_16");
                        }
                    }
                    else {
                        motion_kind = Hash40::new("attack_15");
                    }
                }
                else {
                    motion_kind = Hash40::new("attack_14");
                }
            }
            else {
                motion_kind = Hash40::new("attack_13");
            }
        }
        else {
            motion_kind = Hash40::new("attack_12_combo");
        }
    }
    else {
        motion_kind = Hash40::new("attack_11_combo");
    }
    MotionModule::change_motion(boma, motion_kind, 0.0, 1.0, false, 0.0, false, false);
}

unsafe extern "C" fn demon_attack_combo_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat4 = fighter.global_table[CMD_CAT4].get_i32();
    let boma = fighter.module_accessor;
    let combo_count = WorkModule::get_int(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_COMBO);
    let mut next_status = WorkModule::get_int(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_NEXT_STATUS);
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if next_status == *FIGHTER_STATUS_KIND_NONE || (next_status != *FIGHTER_STATUS_KIND_NONE && next_status == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2) {
        if combo_count == 4 || (combo_count != 4 && (combo_count == 6 || combo_count == 7)) {
            if next_status != *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2 {
                if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0 {
                    next_status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S;
                }
                else {
                    if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT != 0 {
                        next_status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F;
                    }
                    else {
                        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_323CATCH != 0 {
                            next_status = *FIGHTER_DEMON_STATUS_KIND_CATCH_COMMAND;
                        }
                        else {
                            if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A != 0 {
                                next_status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2;
                            }
                            else {
                                if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6AB != 0 {
                                    next_status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1;
                                }
                                else {
                                    next_status = *FIGHTER_STATUS_KIND_NONE;
                                }
                            }
                        }
                    }
                }
            }
            else {
                if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG != 0 {
                    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM) {
                        next_status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L;
                    }
                    else {
                        next_status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE;
                    }
                }
                if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623BLONG != 0 {
                    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM) {
                        next_status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE;
                    }
                }
            }
        }
        if next_status == *FIGHTER_STATUS_KIND_NONE {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                    if !StatusModule::is_changing(boma) {
                        next_status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO;
                    }
                }
            }
        }
        WorkModule::set_int(boma, next_status, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_NEXT_STATUS);
    }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
        if next_status == *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO {
            fun_7100031280(fighter, (combo_count+1).into());
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_16);
            return 1.into();
        }
    }
    if MotionModule::is_end(boma) {
        if next_status != *FIGHTER_STATUS_KIND_NONE {
            if next_status != *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO.into(), true.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO, demon_attack_combo_main_status)
    .install()
    ;
}