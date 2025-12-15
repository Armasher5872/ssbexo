use super::*;

unsafe extern "C" fn koopa_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn koopa_special_n_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_STEP);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_PREV_STEP);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_PREV_GENERATE_KIND);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_SE1_HANDLE);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_SE2_HANDLE);
    if kind != *FIGHTER_KIND_KIRBY {
        if kind != *FIGHTER_KIND_KOOPAG {
            WorkModule::set_int64(fighter.module_accessor, hash40("se_koopa_special_n02") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
            WorkModule::set_int64(fighter.module_accessor, hash40("head") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
        }
        else {
            WorkModule::set_int64(fighter.module_accessor, hash40("se_koopag_special_n02") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
            WorkModule::set_int64(fighter.module_accessor, hash40("head") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
        }
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("se_koopa_special_n02") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
        WorkModule::set_int64(fighter.module_accessor, hash40("head") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
    0.into()
}

unsafe extern "C" fn koopa_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n"), L2CValue::Hash40s("special_air_n"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(koopa_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn koopa_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn koopa_special_n_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn koopa_special_n_exec_stop_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn koopa_special_n_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn koopa_special_n_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_exec_status)
    .status(ExecStop, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_exec_stop_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_exit_status)
    .install()
    ;
}