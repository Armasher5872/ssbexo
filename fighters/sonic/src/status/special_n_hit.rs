use super::*;

unsafe extern "C" fn sonic_special_n_hit_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rand_num_8 = sv_math::rand(hash40("fighter"), 8);
    let sonic_new_animation_hash = Hash40::new(match rand_num_8 {1|2 => "special_n_hit", 3..=4 => "special_n_hit_1", 5..=6 => "special_n_hit_2", _ => "special_n_hit_3"});
    let special_n_hit_accel_y_mul_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_hit_accel_y_mul_frame"));
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
    WorkModule::set_int(fighter.module_accessor, special_n_hit_accel_y_mul_frame, *FIGHTER_SONIC_STATUS_SPECIAL_N_HIT_WORK_INT_ADVANCE_COUNTER);
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_N_HIT_WORK_INT_ADVANCE_COUNTER);
    if !StopModule::is_stop(fighter.module_accessor) {
        sonic_special_n_hit_substatus(fighter);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(sonic_special_n_hit_substatus as *const () as _));
    MotionModule::change_motion(fighter.module_accessor, sonic_new_animation_hash, 0.0, 1.0, false, 0.0, false, false);
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
    let mut ivar4 = 0;
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    || fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || !fighter.sub_air_check_fall_common().get_bool() {
        ivar4 = 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_LANDING.into(), false.into());
        ivar4 = 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        ivar4 = 0.into();
    }
    ivar4.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, sonic_special_n_hit_main_status)
    .install()
    ;
}