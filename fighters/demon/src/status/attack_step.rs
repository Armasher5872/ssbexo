use super::*;

unsafe extern "C" fn demon_attack_step_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let boma = fighter.module_accessor;
    let move_mul = WorkModule::get_param_float(boma, hash40("param_attack_step"), hash40("move_mul"));
    let cancel_frame = WorkModule::get_param_int(boma, hash40("param_attack_step"), hash40("cancel_frame"));
    MotionModule::change_motion(boma, Hash40::new("attack_step"), 0.0, 1.0, false, 0.0, false, false);
    FighterControlModuleImpl::delete_command(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_COMMAND_623NB);
    FighterControlModuleImpl::delete_command(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_DASH);
    fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_DASH));
    ControlModule::reset_flick_x(boma);
    WorkModule::set_int(boma, cancel_frame, *FIGHTER_DEMON_STATUS_ATTACK_STEP_WORK_INT_CANCEL_FRAME);
    if !StopModule::is_stop(boma) {
        fun_710002f880(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710002f880 as *const () as _));
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
    FighterSpecializer_Demon::add_attack_log(&mut *global_fighter, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_11, false);
    sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, move_mul);
    MotionModule::set_trans_move_speed_no_scale(boma, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_step_main_loop as *const () as _))
}

unsafe extern "C" fn fun_710002f880(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    if bool_check.get_bool() {
        WorkModule::count_down_int(boma, *FIGHTER_DEMON_STATUS_ATTACK_STEP_WORK_INT_CANCEL_FRAME, 0);
    }
    0.into()
}

unsafe extern "C" fn demon_attack_step_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat4 = fighter.global_table[CMD_CAT4].get_i32();
    let boma = fighter.module_accessor;
    let rage_system = WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM);
    let special_command_neutral_threshold = WorkModule::get_param_float(boma, hash40("common"), hash40("special_command_neutral_threshold"));
    let step2f_frame = WorkModule::get_param_int(boma, hash40("param_attack_step"), hash40("step2f_frame"));
    let vec = fighter.Vector2__create(stick_x.into(), stick_y.into());
    let len = fighter.Vector2__length(vec).get_f32();
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623BLONG != 0 {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S.into(), true.into()); //Spinning Demon to Left Hook
    }
    if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG != 0 {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0 {
            if rage_system {
                WorkModule::set_flag(boma, rage_system, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE.into(), true.into()); //Rage Drive
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into()); //Heaven's Door
            }
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ELECTRIC_DRAGON_UPPERCUT);
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L.into(), true.into()); //Dragon Uppercut
        }
    }
    if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_323CATCH != 0 {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_CATCH_COMMAND.into(), true.into()); //Gates of Hell
        return 0.into();
    }
    if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A != 0 {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0 {
            if rage_system {
                WorkModule::set_flag(boma, rage_system, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE.into(), true.into()); //Rage Drive
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into()); //Heaven's Door
            }
        }
        else {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2.into(), true.into()); //Wind God Fist
            return 0.into();
        }
    }
    if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT != 0 {
        if frame <= step2f_frame {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F.into(), true.into()); //I'm not gonna sugarcoat it
            return 0.into();
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_NEUTRAL) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6 != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1.into(), true.into()); //Left Splits Kick
            return 0.into();
        }
    }
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_NEUTRAL) {
        if len <= special_command_neutral_threshold {
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_NEUTRAL);
        }
    }
    if 0 < WorkModule::get_int(boma, *FIGHTER_DEMON_STATUS_ATTACK_STEP_WORK_INT_CANCEL_FRAME) {
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_NEUTRAL) {
            if fighter.sub_transition_group_check_ground_jump().get_bool() {
                return 0.into();
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
                    fun_7100030c20(fighter);
                    fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
                }
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) {
                    fun_7100030c20(fighter);
                    fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
                }
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN != 0 {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN) {
                    fun_7100030c20(fighter);
                    fighter.change_status(FIGHTER_STATUS_KIND_TURN.into(), true.into());
                }
            }
            if fighter.sub_check_command_walk().get_bool() {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK) {
                    fun_7100030c20(fighter);
                    fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), true.into());
                }
            }
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn fun_7100030c20(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    if 3 <= FighterControlModuleImpl::special_command_623_step(boma) {
        FighterControlModuleImpl::reset_special_command_individual(boma, *FIGHTER_PAD_CMD_CAT4_SPECIAL_HI_COMMAND);
    }
    if 3 <= FighterControlModuleImpl::special_command_step(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_623NB) {
        FighterControlModuleImpl::reset_special_command_individual(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_623NB);
    }
    if 3 <= FighterControlModuleImpl::special_command_step(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_623STRICT) {
        FighterControlModuleImpl::reset_special_command_individual(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_623STRICT);
    }
    if 3 <= FighterControlModuleImpl::special_command_step(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_623ALONG) {
        FighterControlModuleImpl::reset_special_command_individual(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_623ALONG);
    }
    if 3 <= FighterControlModuleImpl::special_command_step(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_623BLONG) {
        FighterControlModuleImpl::reset_special_command_individual(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_623BLONG);
    }
    if 3 <= FighterControlModuleImpl::special_command_step(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_623A) {
        FighterControlModuleImpl::reset_special_command_individual(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_623A);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP, demon_attack_step_main_status)
    .install()
    ;
}