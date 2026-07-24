use super::*;

//Demon God Fist Pre Status
unsafe extern "C" fn demon_attack_stand_7_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32, 0);
    0.into()
}

//Demon God Fist Init Status
unsafe extern "C" fn demon_attack_stand_7_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Demon God Fist Main Status
unsafe extern "C" fn demon_attack_stand_7_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if !fun_710002aed0(fighter).get_bool() {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_DEMON_GOD_FIST_TURN);
        MotionModule::change_motion(boma, Hash40::new("attack_stand_7"), 0.0, 1.0, false, 0.0, false, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_09);
        MotionModule::set_trans_move_speed_no_scale(boma, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_stand_7_main_loop as *const () as _))
    }
    else {
        return 0.into();
    }
}

unsafe extern "C" fn demon_attack_stand_7_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let cmd_cat4 = fighter.global_table[CMD_CAT4].get_i32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let command_life_extend = WorkModule::get_param_int(boma, hash40("param_private"), 0x2b87c7acb0);
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_ATTACK) {
        return 1.into();
    }
    if stick_x*lr < -0.7 && WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_DEMON_GOD_FIST_TURN) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_DEMON_GOD_FIST_TURN);
        PostureModule::reverse_lr(boma);
        PostureModule::update_rot_y_lr(boma);
    }
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_SPECIAL_HI_COMMAND) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0 {
            fighter_control_module_set_command_life_count(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_SPECIAL_HI_COMMAND, command_life_extend);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_SPECIAL_HI_COMMAND);
        }
    }
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_COMMAND_623ALONG) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG != 0 {
            fighter_control_module_set_command_life_count(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG, command_life_extend);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_COMMAND_623ALONG);
        }
    }
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_COMMAND_623BLONG) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623BLONG != 0 {
            fighter_control_module_set_command_life_count(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623BLONG, command_life_extend);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_COMMAND_623BLONG);
        }
    }
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_COMMAND_623STRICT) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT != 0 {
            fighter_control_module_set_command_life_count(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT, command_life_extend);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_COMMAND_623STRICT);
        }
    }
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_COMMAND_623A) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A != 0 {
            fighter_control_module_set_command_life_count(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A, command_life_extend);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_COMMAND_623A);
        }
    }
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_COMMAND_623NB) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623NB != 0 {
            fighter_control_module_set_command_life_count(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623NB, command_life_extend);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_SQUAT_4_FLAG_EXTEND_COMMAND_623NB);
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Demon God Fist Exec Status
unsafe extern "C" fn demon_attack_stand_7_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Demon God Fist End Status
unsafe extern "C" fn demon_attack_stand_7_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Demon God Fist Exit Status
unsafe extern "C" fn demon_attack_stand_7_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_7, demon_attack_stand_7_pre_status)
    .status(Init, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_7, demon_attack_stand_7_init_status)
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_7, demon_attack_stand_7_main_status)
    .status(Exec, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_7, demon_attack_stand_7_exec_status)
    .status(End, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_7, demon_attack_stand_7_end_status)
    .status(Exit, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_7, demon_attack_stand_7_exit_status)
    .install()
    ;
}