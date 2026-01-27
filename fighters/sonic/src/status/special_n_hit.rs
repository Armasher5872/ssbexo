use super::*;

unsafe extern "C" fn sonic_special_n_hit_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_NONE | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn sonic_special_n_hit_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn sonic_special_n_hit_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_n_hit_accel_y_mul_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_hit_accel_y_mul_frame"));
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
    WorkModule::set_int(fighter.module_accessor, special_n_hit_accel_y_mul_frame, *FIGHTER_SONIC_STATUS_SPECIAL_N_HIT_WORK_INT_ADVANCE_COUNTER);
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_N_HIT_WORK_INT_ADVANCE_COUNTER);
    if !StopModule::is_stop(fighter.module_accessor) {
        sonic_special_n_hit_substatus(fighter);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(sonic_special_n_hit_substatus as *const () as _));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hit"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_n_hit_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_n_hit_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_N_HIT_WORK_INT_ADVANCE_COUNTER);
    let advance_counter = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_N_WORK_FLOAT_ADVANCE_COUNTER);
    if advance_counter < 0.0 {
        fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::I32(0));
        fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
    }
    0.into()
}

unsafe extern "C" fn sonic_special_n_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_LANDING.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn sonic_special_n_hit_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn sonic_special_n_hit_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL);
    }
    WorkModule::set_int(fighter.module_accessor, 120, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN_TIMER);
    0.into()
}

unsafe extern "C" fn sonic_special_n_hit_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 120, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN_TIMER);
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, sonic_special_n_hit_pre_status)
    .status(Init, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, sonic_special_n_hit_init_status)
    .status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, sonic_special_n_hit_main_status)
    .status(Exec, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, sonic_special_n_hit_exec_status)
    .status(End, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, sonic_special_n_hit_end_status)
    .status(Exit, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, sonic_special_n_hit_exit_status)
    .install()
    ;
}