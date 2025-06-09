use super::*;

//Neutral Special

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

//Side Special

unsafe extern "C" fn edge_special_s_charge_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_hold").into(), Hash40::new("special_s_air_hold").into(), false.into());
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_EDGE_STATUS_SPECIAL_S_INT_HOLD_FRAME);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_s_charge_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_s_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let hold_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("hold_max"));
    let hold_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_S_INT_HOLD_FRAME);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_hold").into(), Hash40::new("special_s_air_hold").into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
        edge_special_kinetic_handler(fighter, false);
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if 0.0 < current_frame {
            edge_special_s_reverse_function(fighter);
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT.into(), false.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if hold_frame >= 5 {
            if fighter.sub_check_command_guard().get_bool() {
                if situation_kind == *SITUATION_KIND_GROUND {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                    fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                    return 1.into()
                }
                else {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                        return 1.into()
                    }
                }
            }
            else {
                if situation_kind == *SITUATION_KIND_AIR {
                    if fighter.sub_check_jump_in_charging().get_bool() {
                        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                    }
                }
                else {
                    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0
                    || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0) && fighter.sub_check_button_frick().get_bool()) {
                        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                    }
                }
            }
        }
    }
    if hold_max <= hold_frame {
        edge_special_s_reverse_function(fighter);
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn edge_special_kinetic_handler(fighter: &mut L2CFighterCommon, param_1: bool) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if !param_1 && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            return 0.into();
        }
        sv_kinetic_energy!(set_needs_set_param, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, false);
        if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            sv_kinetic_energy!(set_needs_set_param, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, false);
        }
    }
    0.into()
}

unsafe extern "C" fn edge_special_s_reverse_function(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
    if stick_x <= turn_stick_x {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    0.into()
}

//Up Special

unsafe extern "C" fn edge_special_hi_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("edge_octaslash_line"), true, true);
    fun_7100014650(fighter);
    fun_71000147f0(fighter);
    0.into()
}

unsafe extern "C" fn fun_7100014650(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE);
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: 0 as u8}, MotionNodeRotateOrder{_address: 0 as u8});
    0.into()
}

unsafe extern "C" fn fun_71000147f0(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let charged_rush = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
    let landing_lag = edge_special_hi_param_int_helper(fighter, Hash40::new("landing_fix_frame").into(), charged_rush.into());
    if status_kind != *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        if status_kind != *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            return 0.into();
        }
    }
    WorkModule::set_float(fighter.module_accessor, landing_lag.get_f32(), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    0.into()
}

unsafe extern "C" fn edge_special_hi_param_int_helper(fighter: &mut L2CFighterCommon, hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let param = edge_special_hi_param_helper_inner(hash, charged_rush).get_u64();
    WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), param).into()
}

unsafe extern "C" fn edge_special_hi_param_helper_inner(hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let hash = hash.get_u64();
    if !charged_rush.get_bool() {
        return hash.into();
    }
    let new_hash = if hash == hash40("rot_decide_frame") {
        hash40("charged_rot_decide_frame")
    }
    else if hash == hash40("rot_end_frame") {
        hash40("charged_rot_end_frame")
    }
    else if hash == hash40("rush_frame") {
        hash40("charged_rush_frame")
    }
    else if hash == hash40("rush_speed") {
        hash40("charged_rush_speed")
    }
    else if hash == hash40("rush_brake_frame") {
        hash40("charged_rush_brake_frame")
    }
    else if hash == hash40("rush_brake") {
        hash40("charged_rush_brake")
    }
    else if hash == hash40("ground_speed_x_mul") {
        hash40("charged_ground_speed_x_mul")
    }
    else if hash == hash40("landing_speed_x_mul") {
        hash40("charged_landing_speed_x_mul")
    }
    else if hash == hash40("landing_brake_x") {
        hash40("charged_landing_brake_x")
    }
    else if hash == hash40("landing_fix_frame") {
        hash40("charged_landing_fix_frame")
    }
    else if hash == hash40("rotate_back_begin_frame") {
        hash40("charged_rotate_back_begin_frame")
    }
    else if hash == hash40("rotate_back_end_frame") {
        hash40("charged_rotate_back_end_frame")
    }
    else if hash == hash40("rush_end_speed_mul") {
        hash40("charged_rush_end_speed_mul")
    }
    else if hash == hash40("rush_end_brake_x") {
        hash40("charged_rush_end_brake_x")
    }
    else if hash == hash40("rush_end_gravity_accel") {
        hash40("charged_rush_end_gravity_accel")
    }
    else if hash == hash40("control_accel_x_mul") {
        hash40("charged_control_accel_x_mul")
    }
    else if hash == hash40("control_speed_x_max_mul") {
        hash40("charged_control_speed_x_max_mul")
    }
    else{
        hash
    };
    new_hash.into()
}

unsafe extern "C" fn edge_special_hi_landing_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("edge_octaslash_line"), true, true);
    0.into()
}

//Flare

unsafe extern "C" fn edge_fire_s_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_direction = WorkModule::get_int(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_SPECIAL_N_DIRECTION);
    let lr = PostureModule::lr(weapon.module_accessor);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_fire"), hash40("life_s"));
    let speed_x_s = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("speed_x_s"));
    let accel_x_s = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_s"));
    let max_speed_x_s = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("max_speed_x_s"));
    let angle: f32 = 20.0;
    let speed_x = angle.to_radians().sin()*speed_x_s*lr;
    let speed_y = angle.to_radians().cos()*speed_x_s;
    if WorkModule::is_flag(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if owner_direction == 2 {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_s*lr, -accel_x_s);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_s, max_speed_x_s);
        }
        else if owner_direction == 1 {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_s*lr, accel_x_s);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_s, max_speed_x_s);
        }
        else {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x_s*lr, 0.0);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_s*lr, 0.0);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_s, -1.0);
        }
    }
    else {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x_s*lr, 0.0);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_s*lr, 0.0);
        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_s, -1.0);
    }
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_n1"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(edge_fire_s_fly_main_loop as *const () as _))
}

unsafe extern "C" fn edge_fire_s_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    edge_fire_fly_sub(weapon, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_S.into());
    return 0.into()
}

unsafe extern "C" fn edge_fire_s_fly_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    let owner_prev_status_kind = StatusModule::prev_status_kind(owner_boma, 0);
    if WorkModule::is_flag(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if owner_status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL
        && owner_prev_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            weapon.change_status(WEAPON_EDGE_FIRE_STATUS_KIND_BURST_S.into(), false.into());
        }
    }
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

//Megaflare

unsafe extern "C" fn edge_fire_m_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_direction = WorkModule::get_int(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_SPECIAL_N_DIRECTION);
    let lr = PostureModule::lr(weapon.module_accessor);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_fire"), hash40("life_m"));
    let speed_x_m = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("speed_x_m"));
    let accel_x_m = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_m"));
    let max_speed_x_m = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("max_speed_x_m"));
    let angle: f32 = 20.0;
    let speed_x = angle.to_radians().sin()*speed_x_m*lr;
    let speed_y = angle.to_radians().cos()*speed_x_m;
    if WorkModule::is_flag(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if owner_direction == 2 {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_m*lr, -accel_x_m);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_m, max_speed_x_m);
        }
        else if owner_direction == 1 {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_m*lr, accel_x_m);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_m, max_speed_x_m);
        }
        else {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x_m*lr, 0.0);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_m*lr, 0.0);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_m, -1.0);
        }
    }
    else {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x_m*lr, 0.0);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_m*lr, 0.0);
        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_m, -1.0);
    }
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_n2"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(edge_fire_m_fly_main_loop as *const () as _))
}

unsafe extern "C" fn edge_fire_m_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    edge_fire_fly_sub(weapon, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_M.into());
    return 0.into()
}

unsafe extern "C" fn edge_fire_m_fly_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    let owner_prev_status_kind = StatusModule::prev_status_kind(owner_boma, 0);
    if WorkModule::is_flag(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if owner_status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL
        && owner_prev_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            weapon.change_status(WEAPON_EDGE_FIRE_STATUS_KIND_BURST_M.into(), false.into());
        }
    }
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

//Gigaflare

unsafe extern "C" fn edge_fire_l_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_direction = WorkModule::get_int(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_SPECIAL_N_DIRECTION);
    let lr = PostureModule::lr(weapon.module_accessor);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_fire"), hash40("life_l"));
    let speed_x_l = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("speed_x_l"));
    let accel_x_l = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_l"));
    let max_speed_x_l = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("max_speed_x_l"));
    let angle: f32 = 20.0;
    let speed_x = angle.to_radians().sin()*speed_x_l*lr;
    let speed_y = angle.to_radians().cos()*speed_x_l;
    if WorkModule::is_flag(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if owner_direction == 2 {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_l*lr, -accel_x_l);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_l, max_speed_x_l);
        }
        else if owner_direction == 1 {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_l*lr, accel_x_l);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_l, max_speed_x_l);
        }
        else {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x_l*lr, 0.0);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_l*lr, 0.0);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_l, -1.0);
        }
    }
    else {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x_l*lr, 0.0);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_l*lr, 0.0);
        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_l, -1.0);
    }
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_n3"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(edge_fire_l_fly_main_loop as *const () as _))
}

unsafe extern "C" fn edge_fire_l_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    edge_fire_fly_sub(weapon, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_L.into());
    return 0.into()
}

unsafe extern "C" fn edge_fire_l_fly_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    let owner_prev_status_kind = StatusModule::prev_status_kind(owner_boma, 0);
    if WorkModule::is_flag(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if owner_status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL
        && owner_prev_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            weapon.change_status(WEAPON_EDGE_FIRE_STATUS_KIND_BURST_L.into(), false.into());
        }
    }
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn edge_fire_fly_sub(weapon: &mut L2CWeaponCommon, status: L2CValue) -> L2CValue {
    let current_frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let facing = PostureModule::lr(weapon.module_accessor);
    let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_m"))*facing;
    if life <= 0 || (WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL) && current_frame <= 2.0) {
        weapon.change_status(status, false.into());
        return 1.into();
    }
    else {
        if weapon.sub_ground_module_is_touch_all_consider_speed().get_bool() {
            if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
                sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, 1.0);
                sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y*-1.0);
                return 0.into()
            }
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL);
            if current_frame > 1.0 {
                weapon.change_status(status, false.into());
                return 1.into()
            }
            StopModule::set_other_stop(weapon.module_accessor, 2, StopOtherKind(0));
        }
    }
    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_ATTACK) {
        return 0.into()
    }
    weapon.change_status(status, false.into());
    1.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT, edge_special_n_shoot_main_status)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT, edge_special_n_shoot_end_status)
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE, edge_special_s_charge_main_status)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, edge_special_hi_end_end_status)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, edge_special_hi_landing_end_status)
    .install()
    ;
    Agent::new("edge_fire")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, edge_fire_s_fly_main_status)
    .status(Exec, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, edge_fire_s_fly_exec_status)
    .status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M, edge_fire_m_fly_main_status)
    .status(Exec, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M, edge_fire_m_fly_exec_status)
    .status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L, edge_fire_l_fly_main_status)
    .status(Exec, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L, edge_fire_l_fly_exec_status)
    .install()
    ;
}