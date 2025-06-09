use super::*;

/*   NEUTRAL SPECIAL STATUS SCRIPTS   */

unsafe extern "C" fn littlemac_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2.into(), false.into());
    0.into()
}

unsafe extern "C" fn littlemac_special_n2_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

/*   DOWN SPECIAL STATUS SCRIPTS   */

unsafe extern "C" fn littlemac_special_lw_hit_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut attack_power;
    let x = 0.0;
    let y = 0.0;
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let get_sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_lw_attack_power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_WORK_FLOAT_ATTACK_POWER);
    let special_lw_attack_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_attack_mul"));
    let special_lw_attack_power_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_attack_power_limit"));
    let special_lw_attack_max_for_enemy = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_attack_max_for_enemy"));
    let special_lw_attack_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_attack_max"));
    let special_lw_start_mul_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_start_mul_spd_x"));
    let special_lw_start_air_acl_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_start_air_acl_x"));
    let special_lw_attack_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_attack_acl_y"));
    let special_lw_attack_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_attack_max_y"));
    vector["x"].assign(&L2CValue::F32(get_sum_speed));
    vector["y"].assign(&L2CValue::F32(get_sum_speed));
    if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if status_kind != *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT {
            return 0.into();
        }
        attack_power = special_lw_attack_power*special_lw_attack_mul;
        if attack_power < special_lw_attack_power_limit {
            attack_power = special_lw_attack_power_limit;
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_IS_ATTACK_ENEMY) {
            if special_lw_attack_max_for_enemy < attack_power {
                attack_power = special_lw_attack_max_for_enemy;
            }
        }
        else {
            if special_lw_attack_max < attack_power {
                attack_power = special_lw_attack_max;
            }
        }
        WorkModule::set_float(fighter.module_accessor, attack_power, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_WORK_FLOAT_ATTACK_POWER);
    }
    else {
        if situation_kind != *SITUATION_KIND_AIR {
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_WORK_INT_SITUATION_PREV);
        }
        else {
            vector["x"].assign(&L2CValue::F32(vec_x*special_lw_start_mul_spd_x));
            vector["y"].assign(&L2CValue::F32(0.0));
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, vec_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_start_air_acl_x, 0.0);
            sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, vec_y, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_lw_attack_acl_y);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_lw_attack_max_y);
            sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_WORK_INT_SITUATION_PREV);
        }
    }
    0.into()
}

unsafe extern "C" fn littlemac_special_lw_hit_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let special_lw_feint_x_speed_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_feint_x_speed_"));
    let special_air_lw_feint_x_speed_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_air_lw_feint_x_speed_"));
    let special_air_lw_feint_x_speed = special_air_lw_feint_x_speed_*lr;
    let special_lw_feint_x_speed = special_lw_feint_x_speed_*lr;
    if current_frame >= 1.0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        if situation_kind != *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_air_lw_feint_x_speed, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_air_lw_feint_x_speed, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        else {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_feint_x_speed, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_feint_x_speed, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(littlemac_special_lw_hit_main_loop as *const () as _))
    }
    else {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        if situation_kind != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
        }
        0.into()
    }
}

unsafe extern "C" fn littlemac_special_lw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_air_lw_move_after_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_air_lw_move_after_accel_y"));
    let special_air_lw_move_after_speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_air_lw_move_after_speed_y_max"));
    let special_air_lw_counter_speed_x_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_air_lw_counter_speed_x_"));
    let special_air_lw_counter_brake_x_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_air_lw_counter_brake_x_"));
    let special_lw_counter_speed_x_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_counter_speed_x_"));
    let special_air_lw_attack_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_air_lw_attack_end_speed_x_mul"));
    let special_air_lw_move_after_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_air_lw_move_after_brake_x"));
    let special_lw_attack_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_attack_end_speed_x_mul"));
    let special_lw_counter_brake_x_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_counter_brake_x_"));
    let special_lw_counter_speed_x = special_lw_counter_speed_x_*lr;
    let special_air_lw_counter_speed_x = special_air_lw_counter_speed_x_*lr;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if !fighter.sub_air_check_fall_common().get_bool() {
                if MotionModule::is_end(fighter.module_accessor) {
                    if situation_kind != *SITUATION_KIND_GROUND {
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                        return 0.into();
                    }
                    else {
                        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                        return 0.into();
                    }
                }
            }
        }
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ENABLE_GRAVITY) {
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_air_lw_move_after_accel_y);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_air_lw_move_after_speed_y_max );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ENABLE_GRAVITY);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_START) {
        if situation_kind != *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_air_lw_counter_speed_x, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_air_lw_counter_speed_x, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_air_lw_counter_brake_x_, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        else {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_counter_speed_x, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_counter_speed_x, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_counter_brake_x_, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_START);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_END) {
        if situation_kind != *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR_BRAKE_ALWAYS, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x*special_air_lw_attack_end_speed_x_mul, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_air_lw_move_after_brake_x, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        else {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x*special_lw_attack_end_speed_x_mul, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_counter_brake_x_, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_END);
    }
    if prev_situation_kind != *SITUATION_KIND_GROUND {
        if situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_hit"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
            }
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_counter_brake_x_, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    else {
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            if situation_kind != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_hit"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
                }
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
                sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_lw_counter_brake_x_, 0.0);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
        }
    }
    fun_7100018140(fighter);
    0.into()
}

unsafe extern "C" fn fun_7100018140(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut box_num = -1;
    let next_box_num = box_num+1;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let attack_power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_WORK_FLOAT_ATTACK_POWER);
    if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if status_kind != *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT {
            return 0.into();
        }
        if attack_power > 0.0 {
            let part_size = (AttackModule::part_size(fighter.module_accessor) as i32) - 1;
            if -1 < part_size {
                while box_num <= part_size {
                    if AttackModule::is_attack(fighter.module_accessor, next_box_num, false) {
                        let get_power = AttackModule::get_power(fighter.module_accessor, next_box_num, false, 1.0, false);
                        if 0.0 < get_power {
                            AttackModule::set_power(fighter.module_accessor, next_box_num, attack_power, false);
                        }
                    }
                    box_num += 1;
                }
            }
        }
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD_CHK) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD) {
                return 0.into();
            }
            ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_LITTLEMAC_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD_CHK);
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD) {
                return 0.into();
            }
            ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_LITTLEMAC_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD_CHK);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, littlemac_special_n_main_status)
    .status(Pre, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, littlemac_special_n2_pre_status)
    .status(Init, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT, littlemac_special_lw_hit_init_status)
    .status(Main, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT, littlemac_special_lw_hit_main_status)
    .install()
    ;
}