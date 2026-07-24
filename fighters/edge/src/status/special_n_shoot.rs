use super::*;

unsafe extern "C" fn edge_special_n_shoot_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let is_wing = WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
    let charge_kind = WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    let mut bool_check = false;
    if charge_kind == *FIGHTER_EDGE_SPECIAL_N_S {
        WorkModule::set_int64(boma, hash40("special_n1") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
        WorkModule::set_int64(boma, hash40("special_air_n1") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
    }
    if charge_kind == *FIGHTER_EDGE_SPECIAL_N_M {
        WorkModule::set_int64(boma, hash40("special_n2") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
        WorkModule::set_int64(boma, hash40("special_air_n2") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
    }
    if charge_kind == *FIGHTER_EDGE_SPECIAL_N_L {
        if is_wing {
            WorkModule::set_int64(boma, hash40("special_n3") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
            WorkModule::set_int64(boma, hash40("special_air_n3") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
        }
        else {
            WorkModule::set_int64(boma, hash40("special_n_start") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
            WorkModule::set_int64(boma, hash40("special_air_n_start") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
            bool_check = true;
        }
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_CANCEL_SCREEN_EFFECT);
    }
    if charge_kind == *FIGHTER_EDGE_SPECIAL_N_XL {
        WorkModule::set_int64(boma, hash40("special_n_start_wing") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
        WorkModule::set_int64(boma, hash40("special_air_n_start_wing") as i64, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_CANCEL_SCREEN_EFFECT);
        bool_check = true;
    }
    let ground_motion = WorkModule::get_int64(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
    let air_motion = WorkModule::get_int64(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
    fighter.sub_change_motion_by_situation(Hash40::new_raw(ground_motion).into(), Hash40::new_raw(air_motion).into(), bool_check.into());
    WorkModule::off_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    WorkModule::off_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED_END_INIT);
    ItemModule::set_have_item_visibility(boma, false, 0);
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_n_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);
    let ground_motion = WorkModule::get_int64(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_MOTION);
    let air_motion = WorkModule::get_int64(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_SHOOT_AIR_MOTION);
    let mut bool_check = false;
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(boma) {
        fighter.sub_change_motion_by_situation(Hash40::new_raw(ground_motion).into(), Hash40::new_raw(air_motion).into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_n").into());
        edge_special_kinetic_handler(fighter, false);
    }
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED) {
        if !WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED_END_INIT) {
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
        WorkModule::off_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED_END_INIT);
    }
    if MotionModule::is_end(boma) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT, edge_special_n_shoot_main_status)
    .install()
    ;
}