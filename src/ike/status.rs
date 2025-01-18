use super::*;

unsafe extern "C" fn ike_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(ike_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn ike_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    if !ike_attack_air_main_common(fighter).get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOP].get_bool() {
            smash::app::FighterUtil::check_cloud_through_out(module_accessor);
        }
    }
    0.into()
}

unsafe extern "C" fn ike_attack_air_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let ike_bound_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_BOUND_ANGLE);
    let ike_x_check = WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_X_CHECK);
    let ike_y_check = WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_Y_CHECK);
    let data_0 = AttackModule::attack_data(fighter.module_accessor, 0, false);
    let data_1 = AttackModule::attack_data(fighter.module_accessor, 1, false);
    let data_2 = AttackModule::attack_data(fighter.module_accessor, 2, false);
    let data_3 = AttackModule::attack_data(fighter.module_accessor, 3, false);
    let data_4 = AttackModule::attack_data(fighter.module_accessor, 4, false);
    let ike_damage_0: f32 = (*data_0).power;
    let ike_damage_1: f32 = (*data_1).power;
    let ike_damage_2: f32 = (*data_2).power;
    let ike_damage_3: f32 = (*data_3).power;
    let ike_damage_4: f32 = (*data_4).power;
    let vector = Vector2f{x: 6.0*lr, y: 0.0};
    let ray_check = GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: pos_y}, &Vector2f{x: ike_x_check, y: ike_y_check}, &mut Vector2f::zero(), true);
    let wall_check = GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x+ike_x_check, y: pos_y+ike_y_check}, &vector, true) == 1
        || GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x+(ike_x_check*0.9), y: pos_y+(ike_y_check*0.9)}, &vector, true) == 1
        || GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x+(ike_x_check*0.8), y: pos_y+(ike_y_check*0.8)}, &vector, true) == 1
        || GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x+(ike_x_check*0.7), y: pos_y+(ike_y_check*0.7)}, &vector, true) == 1
        || GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x+(ike_x_check*0.6), y: pos_y+(ike_y_check*0.6)}, &vector, true) == 1
        || GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x+(ike_x_check*0.5), y: pos_y+(ike_y_check*0.5)}, &vector, true) == 1
        || GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x+(ike_x_check*0.4), y: pos_y+(ike_y_check*0.4)}, &vector, true) == 1
        || GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x+(ike_x_check*0.3), y: pos_y+(ike_y_check*0.3)}, &vector, true) == 1
        || GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x+(ike_x_check*0.2), y: pos_y+(ike_y_check*0.2)}, &vector, true) == 1
        || GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x+(ike_x_check*0.1), y: pos_y+(ike_y_check*0.1)}, &vector, true) == 1;
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if !fighter.attack_air_common_strans().get_bool() {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
            }
        }
        if prev_status_kind == *FIGHTER_STATUS_KIND_PASS {
            if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
        }
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP);
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return true.into();
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_CAN_BOUND) && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) || ray_check || wall_check) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_CAN_BOUND);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE) {
            let mut bound_speed_x_0 = ike_bound_angle.to_radians().sin()*(ike_damage_0/5.0);
            let mut bound_speed_y_0 = ike_bound_angle.to_radians().cos()*(ike_damage_0/5.0);
            let mut bound_speed_x_1 = ike_bound_angle.to_radians().sin()*(ike_damage_1/5.0);
            let mut bound_speed_y_1 = ike_bound_angle.to_radians().cos()*(ike_damage_1/5.0);
            let mut bound_speed_x_2 = ike_bound_angle.to_radians().sin()*(ike_damage_2/5.0);
            let mut bound_speed_y_2 = ike_bound_angle.to_radians().cos()*(ike_damage_2/5.0);
            let mut bound_speed_x_3 = ike_bound_angle.to_radians().sin()*(ike_damage_3/5.0);
            let mut bound_speed_y_3 = ike_bound_angle.to_radians().cos()*(ike_damage_3/5.0);
            let mut bound_speed_x_4 = ike_bound_angle.to_radians().sin()*(ike_damage_4/5.0);
            let mut bound_speed_y_4 = ike_bound_angle.to_radians().cos()*(ike_damage_4/5.0);
            if ike_bound_angle > 180.0 && ike_bound_angle < 0.0 {
                bound_speed_x_0 = -(ike_bound_angle.to_radians().sin()*(ike_damage_0/5.0));
                bound_speed_y_0 = -(ike_bound_angle.to_radians().cos()*(ike_damage_0/5.0));
                bound_speed_x_1 = -(ike_bound_angle.to_radians().sin()*(ike_damage_1/5.0));
                bound_speed_y_1 = -(ike_bound_angle.to_radians().cos()*(ike_damage_1/5.0));
                bound_speed_x_2 = -(ike_bound_angle.to_radians().sin()*(ike_damage_2/5.0));
                bound_speed_y_2 = -(ike_bound_angle.to_radians().cos()*(ike_damage_2/5.0));
                bound_speed_x_3 = -(ike_bound_angle.to_radians().sin()*(ike_damage_3/5.0));
                bound_speed_y_3 = -(ike_bound_angle.to_radians().cos()*(ike_damage_3/5.0));
                bound_speed_x_4 = -(ike_bound_angle.to_radians().sin()*(ike_damage_4/5.0));
                bound_speed_y_4 = -(ike_bound_angle.to_radians().cos()*(ike_damage_4/5.0));
            }
            let bound_speed_x = (bound_speed_x_0+bound_speed_x_1+bound_speed_x_2+bound_speed_x_3+bound_speed_x_4)/5.0;
            let bound_speed_y = (bound_speed_y_0+bound_speed_y_1+bound_speed_y_2+bound_speed_y_3+bound_speed_y_4)/5.0;
            if motion_kind == hash40("attack_air_b") {
                macros::SET_SPEED_EX(fighter, bound_speed_x/4.0, bound_speed_y/2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return false.into();
    }
    true.into()
}

unsafe extern "C" fn ike_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn ike_special_n_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

unsafe extern "C" fn ike_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_start"), L2CValue::Hash40s("special_air_n_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(ike_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn ike_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn ike_special_n_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ike_special_n_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ike_special_n_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ike_slash_shoot_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn ike_slash_shoot_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_slash"), hash40("life"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_slash"), hash40("speed_max"));
    let lr = PostureModule::lr(weapon.module_accessor);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    ModelModule::set_scale(weapon.module_accessor, 0.001);
    weapon.clear_lua_stack();
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, 0.0);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, 0.0);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

unsafe extern "C" fn ike_slash_shoot_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_slash"), hash40("speed_max"));
    let lr = PostureModule::lr(weapon.module_accessor);
    ReflectorModule::set_status(weapon.module_accessor, *WEAPON_IKE_SLASH_REFLECTOR_KIND_REFLECTOR, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    if GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, 0.0);
        sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, 0.0);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    }
    if should_remove_projectile(weapon) {
        slash_removal(weapon);
    }
    weapon.fastshift(L2CValue::Ptr(ike_slash_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn ike_slash_shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = weapon.global_table[PREV_SITUATION_KIND].get_i32();
    if should_remove_projectile(weapon) || (situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR && WorkModule::is_flag(owner_boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N)) {
        slash_removal(weapon);
    }
    0.into()
}

unsafe extern "C" fn ike_slash_shoot_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn ike_slash_shoot_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    0.into()
}

unsafe extern "C" fn ike_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn ike_special_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_n_start_spd_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n") as u64, hash40("special_n_start_spd_x_mul") as u64);
    let special_n_brake_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n") as u64, hash40("special_n_brake_spd_x") as u64);
    let x_mul = get_sum_speed_x*special_n_start_spd_x_mul;
    let mut y_mul = get_sum_speed_y;
    if situation_kind != *SITUATION_KIND_AIR {
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, x_mul, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_SITUATION_PREV);
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, x_mul, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_n_brake_spd_x, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if y_mul < 0.0 {
            y_mul = 0.0;
        }
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, y_mul, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_SITUATION_PREV);
    }
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
    0.into()
}

unsafe extern "C" fn ike_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CHARGE_MAX);
    fun_7100020e80(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(ike_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100020e80(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT){
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
    }
    0.into()
}

unsafe extern "C" fn ike_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP.into(), true.into());
        }
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            fun_7100020e80(fighter);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            fun_7100020e80(fighter);
        }
    }
    0.into()
}

unsafe extern "C" fn ike_special_lw_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let situation_prev = WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_SITUATION_PREV);
    if situation_kind != situation_prev {
        let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let special_n_brake_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n") as u64, hash40("special_n_brake_spd_x") as u64);
        if situation_kind != *SITUATION_KIND_AIR {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, get_sum_speed_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, module_accessor);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_SITUATION_PREV);
        }
        else {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, get_sum_speed_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_n_brake_spd_x, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, get_sum_speed_y, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_SITUATION_PREV);
        }
    }
    0.into()
}

unsafe extern "C" fn ike_special_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP {
        effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("ike_volcano_hold"), false, true);
        fighter.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn ike_special_lw_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ike_special_lw_end_max_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            if DamageModule::damage(opponent_boma, 0) >= 150.0 && fighter.global_table[CURRENT_FRAME].get_f32() < 11.0 {
                StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_DEAD, false);
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("ike")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, ike_attack_air_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_special_n_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_special_n_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_special_n_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_special_n_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_special_n_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, ike_special_n_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_special_lw_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_special_lw_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, ike_special_lw_exit_status)
    .status(CheckAttack, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX, ike_special_lw_end_max_check_attack_status)
    .install()
    ;
    Agent::new("ike_slash")
    .status(Pre, *WEAPON_IKE_SLASH_STATUS_KIND_SHOOT, ike_slash_shoot_pre_status)
    .status(Init, *WEAPON_IKE_SLASH_STATUS_KIND_SHOOT, ike_slash_shoot_init_status)
    .status(Main, *WEAPON_IKE_SLASH_STATUS_KIND_SHOOT, ike_slash_shoot_main_status)
    .status(Exec, *WEAPON_IKE_SLASH_STATUS_KIND_SHOOT, ike_slash_shoot_exec_status)
    .status(End, *WEAPON_IKE_SLASH_STATUS_KIND_SHOOT, ike_slash_shoot_end_status)
    .install()
    ;
}