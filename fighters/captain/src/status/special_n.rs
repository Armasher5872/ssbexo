use super::*;

//Neutral Special Pre Status
unsafe extern "C" fn captain_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

//Neutral Special Init Status
unsafe extern "C" fn captain_special_n_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    WorkModule::set_float(boma, lr, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLOAT_FALCON_PUNCH_START_CHARA_DIR);
    0.into()
}

//Neutral Special Main Status
unsafe extern "C" fn captain_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int64(boma, hash40("special_n") as i64, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(boma, hash40("special_air_n") as i64, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_AIR_MOT);
    WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    WorkModule::set_int(boma, 0, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    WorkModule::set_int(boma, -1, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_PREV_AIR_PHASE);
    WorkModule::set_int(boma, 0, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_SPECIAL_N_EFFECT_HANDLE);
    if !StopModule::is_stop(boma) {
        fun_71000152d0(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_71000152d0 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(captain_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn fun_71000152d0(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let turn_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_stick_x"));
    if !bool_check.get_bool() {
        if WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN) {
            if !WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_CHECK_END) {
                if stick_x*lr <= turn_stick_x {
                    if !WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED) {
                        fighter.change_status(FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN.into(), true.into());
                    }
                }
                else {
                    WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_CHECK_END);
                }
            }
            else {
                WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_CHECK_END);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn captain_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let frame = MotionModule::frame(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let rate = MotionModule::rate(boma);
    let is_stored = WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if !StatusModule::is_changing(boma) {
        if StatusModule::is_situation_changed(boma) {
            fun_7100015170(fighter, false.into());
        }
    }
    else {
        fun_7100015170(fighter, false.into());
    }
    fun_7100014db0(fighter, WEAPON_CAPTAIN_FALCONPUNCH_STATUS_KIND_SPECIAL_N.into());
    if (30.0..=40.0).contains(&frame) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) 
        && !is_stored
        && motion_kind == hash40("special_n") {
            MotionModule::change_motion(boma, Hash40::new("special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if motion_kind == hash40("special_n_hold") {
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
            fun_7100015170(fighter, true.into());
        }
    }
    if is_stored {
        if frame >= 50.0 && rate != 1.0 {
            MotionModule::set_rate(boma, 1.0);
        }
    }
    if MotionModule::is_end(boma) {
        if motion_kind == hash40("special_n_hold") {
            fighter.change_status(FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_CHARGED.into(), false.into());
        }
        else {
            if situation_kind != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
    }
    0.into()
}

unsafe extern "C" fn fun_7100015170(fighter: &mut L2CFighterCommon, was_holding: L2CValue) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let is_stored = WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
    let ground_mot = WorkModule::get_int64(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_GROUND_MOT);
    let air_mot = WorkModule::get_int64(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_AIR_MOT);
    if situation_kind != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
            if is_stored {
                MotionModule::change_motion(boma, Hash40::new_raw(air_mot), 0.0, 4.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new_raw(air_mot), 0.0, 1.0, false, 0.0, false, false);
            }
            WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40::new_raw(air_mot), -1.0, 1.0, 0.0);
        }
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
            if was_holding.get_bool() {
                STOP_SE(fighter, Hash40::new("se_captain_boost_charge"));
                fighter.change_status(FIGHTER_STATUS_KIND_FURAFURA_END.into(), false.into());
            }
            else {
                if is_stored {
                    MotionModule::change_motion(boma, Hash40::new_raw(ground_mot), 0.0, 4.0, false, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(boma, Hash40::new_raw(ground_mot), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40::new_raw(ground_mot), -1.0, 1.0, 0.0);
        }
        if !MotionModule::is_end(boma) {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
        }
    }
}

unsafe extern "C" fn fun_7100014db0(fighter: &mut L2CFighterCommon, falcon_punch_status_kind: L2CValue) {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD) {
        ArticleModule::generate_article(boma, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, false, -1);
        if ArticleModule::is_exist(boma, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH) {
            ArticleModule::change_status_exist(boma, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, falcon_punch_status_kind.get_i32());
            WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_VISIBLE_BIRD);
            ArticleModule::set_visibility_whole(boma, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, true, ArticleOperationTarget(0));
        }
        WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
    }
    if ArticleModule::is_exist(boma, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH) {
        if WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_VISIBLE_BIRD) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, true, ArticleOperationTarget(0));
        }
        else {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, false, ArticleOperationTarget(0));
        }
    }
}

//Neutral Special Exec Status
unsafe extern "C" fn captain_special_n_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special End Status
unsafe extern "C" fn captain_special_n_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    ArticleModule::remove_exist(boma, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, ArticleOperationTarget(0));
    if status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_CHARGED {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
    }
    else {
        STOP_SE(fighter, Hash40::new("se_captain_boost_charge"));
        WorkModule::set_int(boma, 0, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_SPECIAL_N_EFFECT_HANDLE);
        if WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED) {
            WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
        }
    }
    0.into()
}

//Neutral Special Exit Status
unsafe extern "C" fn captain_special_n_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_CHARGED {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
    }
    else {
        STOP_SE(fighter, Hash40::new("se_captain_boost_charge"));
        WorkModule::set_int(boma, 0, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_SPECIAL_N_EFFECT_HANDLE);
        if WorkModule::is_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED) {
            WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, captain_special_n_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, captain_special_n_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, captain_special_n_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, captain_special_n_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, captain_special_n_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, captain_special_n_exit_status)
    .install()
    ;
}