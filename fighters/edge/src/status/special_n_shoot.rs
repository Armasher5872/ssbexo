use super::*;

unsafe extern "C" fn edge_special_n_shoot_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bool_check = false;
    let charge_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    if charge_kind == *FIGHTER_EDGE_SPECIAL_N_S {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n1") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n1") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
    }
    if charge_kind == *FIGHTER_EDGE_SPECIAL_N_M {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n2") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n2") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
    }
    if charge_kind == *FIGHTER_EDGE_SPECIAL_N_L {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_CANCEL_SCREEN_EFFECT);
        bool_check = true;
    }
    let ground_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
    let air_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
    fighter.sub_change_motion_by_situation(Hash40::new_raw(ground_motion).into(), Hash40::new_raw(air_motion).into(), bool_check.into());
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED_END_INIT);
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_n_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bool_check = false;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let ground_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
    let air_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new_raw(ground_motion).into(), Hash40::new_raw(air_motion).into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_n").into());
        edge_special_kinetic_handler(fighter, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED_END_INIT) {
            if situation_kind == *SITUATION_KIND_AIR {
                bool_check = true;
            }
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if prev_situation_kind != *SITUATION_KIND_AIR {
                bool_check = true;
            }
        }
        if bool_check {
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED_END_INIT);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if stick_y >= 0.7 {
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_SPECIAL_N_DIRECTION);
        }
        else if stick_y <= -0.7 {
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_SPECIAL_N_DIRECTION);
        }
        else {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_SPECIAL_N_DIRECTION);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn edge_special_n_shoot_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_SPECIAL_N_DIRECTION);
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT, edge_special_n_shoot_main_status)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT, edge_special_n_shoot_end_status)
    .install()
    ;
}