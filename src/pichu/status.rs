use super::*;

unsafe extern "C" fn pichu_attack_hi4_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT, *FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_HI4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32, 0);
    0.into()
}

unsafe extern "C" fn pichu_attack_hi4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackHi4_common(hash40("attack_hi4").into());
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    attack_hi_set_kinetic(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(pichu_attack_hi4_main_loop as *const () as _))
}

unsafe extern "C" fn attack_hi_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
}

unsafe extern "C" fn pichu_attack_hi4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let frame = MotionModule::frame(fighter.module_accessor);
    if !StatusModule::is_changing(fighter.module_accessor) && StatusModule::is_situation_changed(fighter.module_accessor) {
        attack_hi_set_kinetic(fighter);
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
        if situation_kind == *SITUATION_KIND_AIR && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if (8.0..=12.0).contains(&frame) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    if MotionModule::trans_move_speed(fighter.module_accessor).value[1] < 0.0 && fighter.sub_transition_group_check_air_landing().get_bool() {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
        return 0.into();
    }
    if situation_kind == *SITUATION_KIND_AIR && frame >= 50.0 {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn pichu_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, (*FIGHTER_LOG_MASK_FLAG_THROW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_THROW as u32, 0);
        0.into()
    }
    else {
        fighter.status_pre_Throw()
    }
}

unsafe extern "C" fn pichu_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
        if situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        }
        pichu_throw_sub_status(fighter);
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_throw"), L2CValue::Hash40s("special_air_s_throw"), false.into());
        fighter.sub_shift_status_main(L2CValue::Ptr(pichu_throw_main_loop as *const () as _))
    }
    else {
        fighter.status_Throw()
    }
}

unsafe extern "C" fn pichu_throw_sub_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.ThrowUniq();
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_ThrowUniq as *const () as _));
    0.into()
}

unsafe extern "C" fn pichu_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_throw"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_throw"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn pichu_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_IS_VALID_NUZZLE);
    JostleModule::set_status(fighter.module_accessor, true);
    fighter.status_end_Throw()
}

unsafe extern "C" fn pichu_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_IS_VALID_NUZZLE);
    JostleModule::set_status(fighter.module_accessor, true);
    fighter.sub_throw_uniq_process_exit();
    0.into()
}

unsafe extern "C" fn pichu_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn pichu_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    0.into()
}

unsafe extern "C" fn pichu_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(pichu_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn pichu_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn pichu_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pichu_special_s_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pichu_special_s_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pichu_special_hi_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let quick_attack_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_COUNT);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, Hash40::new("special_air_hi_end"), false, -1.0);
    ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, 0.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    if quick_attack_count == 2 {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, module_accessor);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pichu_special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn pichu_special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_escape().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_AGILITY_CAN_CANCEL) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    }
    else {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    }
    if [hash40("special_hi_end"), 0x115ef899a8, 0x1129ffa93e, 0x11b79b3c9d].contains(&motion_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }
    if [hash40("special_air_hi_end"), 0x1528a56db7, 0x155fa25d21, 0x15c1c6c882].contains(&motion_kind) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("pichu")
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_HI4, pichu_attack_hi4_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4, pichu_attack_hi4_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_THROW, pichu_throw_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_THROW, pichu_throw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_THROW, pichu_throw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_THROW, pichu_throw_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, pichu_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, pichu_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, pichu_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, pichu_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, pichu_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, pichu_special_s_exit_status)
    .status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END, pichu_special_hi_end_main_status)
    .install()
    ;
}