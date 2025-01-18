use super::*;

unsafe extern "C" fn koopajr_attack_lw4_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT, (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION));
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_LW4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32, 0);
    0.into()
}

unsafe extern "C" fn koopajr_attack_lw4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    fighter.attack_lw4_mtrans();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.status_ThrowKirby_Uniq(L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_status_ThrowKirby_Uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(koopajr_attack_lw4_main_loop as *const () as _))
}

unsafe extern "C" fn koopajr_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND)
    && !MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_status_uniq_process_ThrowKirby_execFixPos();
        return 0.into()
    }
    fighter.status_AttackLw4_Main_param(FIGHTER_STATUS_KIND_WAIT.into());
    0.into()
}

unsafe extern "C" fn koopajr_attack_lw4_map_correction_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let frame = MotionModule::frame(fighter.module_accessor);
    let prev_frame = MotionModule::prev_frame(fighter.module_accessor);
    let start_air_frame = 5.0;
    let fall_start_frame = 9.0;
    let fall_stop_frame = 10.0;
    let landing_frame = 11.0;
    if frame <= fall_start_frame {
        return 0.into()
    }
    if prev_frame < start_air_frame  && frame >= start_air_frame {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if prev_frame < fall_stop_frame && frame >= fall_stop_frame {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -6.0);
            sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            MotionModule::set_frame(fighter.module_accessor, fall_stop_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
    }
    else {
        if frame < landing_frame {
            if stick_y <= pass_stick_y {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
            if GroundModule::is_passable_check(fighter.module_accessor) && GroundModule::is_passable_ground(fighter.module_accessor) {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
                MotionModule::set_frame(fighter.module_accessor, landing_frame, true);
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn koopajr_float_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

unsafe extern "C" fn koopajr_float_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
    WorkModule::set_int(fighter.module_accessor, 60, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    0.into()
}

unsafe extern "C" fn koopajr_float_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("clownshaft3"), 0, 0, 0, 0, 0, 0, 0.7, true);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.5, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(koopajr_float_main_loop as *const () as _))
}

unsafe extern "C" fn koopajr_float_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let float_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_attack().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    if fighter.global_table[CURRENT_FRAME].get_f32() == 3.0 {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.5, false, 0.0, false, false);
    }
    if float_time <= 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn koopajr_float_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_erace_smoke"), false, true);
    0.into()
}

unsafe extern "C" fn koopajr_cannonball_shoot_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    0.into()
}

unsafe extern "C" fn koopajr_cannonball_shoot_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let float_charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cannonball"), hash40("speed_min"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cannonball"), hash40("speed_max"));
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_cannonball"), hash40("life"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let lerp_speed = weapon.lerp(speed_min.into(), speed_max.into(), float_charge.into());
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 28, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_FREEZE_FRAME);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, lerp_speed.get_f32()*lr, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn koopajr_cannonball_shoot_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let float_charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let attack_mul_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cannonball"), hash40("attack_mul_min"));
    let attack_mul_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cannonball"), hash40("attack_mul_max"));
    let gravity_start_frame_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cannonball"), hash40("gravity_start_frame_min"));
    let gravity_start_frame_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cannonball"), hash40("gravity_start_frame_max"));
    let attack_mul = attack_mul_max-attack_mul_min;
    let attack_charge = attack_mul*float_charge;
    let lerp_gravity = weapon.lerp(gravity_start_frame_min.into(), gravity_start_frame_max.into(), 0.0f32.into());
    AttackModule::set_power_mul(weapon.module_accessor, attack_mul_min+attack_charge);
    ModelModule::set_scale(weapon.module_accessor, 2.0);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int(weapon.module_accessor, lerp_gravity.get_i32(), *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
    if !StopModule::is_stop(weapon.module_accessor) {
        fun_71000196c0(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_71000196c0 as *const () as _));
    weapon.fastshift(L2CValue::Ptr(koopajr_cannonball_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn fun_71000196c0(weapon: &mut L2CWeaponCommon, is_stop: L2CValue) -> L2CValue {
    if is_stop.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn koopajr_cannonball_shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let float_charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let freeze_frame = WorkModule::get_int(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_FREEZE_FRAME);
    let gravity_frame = WorkModule::get_int(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
    let brake_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cannonball"), hash40("brake_x"));
    let gravity = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cannonball"), hash40("gravity"));
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cannonball"), hash40("speed_min"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cannonball"), hash40("speed_max"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
    let owner_boma = get_owner_boma(weapon);
    let owner_stick_x = ControlModule::get_stick_x(owner_boma)*PostureModule::lr(owner_boma);
    let owner_stick_y = ControlModule::get_stick_y(owner_boma);
    let lerp_speed;
    if life > 0 {
        if freeze_frame > 0 {
            WorkModule::dec_int(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_FREEZE_FRAME);
            if owner_stick_x > 0.7 {
                lerp_speed = weapon.lerp(1.4f32.into(), 3.5f32.into(), float_charge.into());
            }
            else if owner_stick_x < -0.7 {
                lerp_speed = weapon.lerp(0.7f32.into(), 1.8f32.into(), float_charge.into());
            }
            else {
                lerp_speed = weapon.lerp(speed_min.into(), speed_max.into(), float_charge.into());
            }
            if owner_stick_y > 0.5 {
                WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_IS_UP_INPUT);
            }
            else {
                WorkModule::off_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_IS_UP_INPUT);
            }
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, lerp_speed.get_f32()*lr, 0.0);
        }
        if freeze_frame == 0 {
            KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        }
        if gravity_frame == 0 {
            if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_IS_UP_INPUT) {
                sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -brake_x*lr, gravity);
                MotionModule::change_motion(weapon.module_accessor, Hash40::new("rise"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -brake_x*lr, -gravity);
                MotionModule::change_motion(weapon.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
            }
            WorkModule::set_int(weapon.module_accessor, -1, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
        }
        if [hash40("fall"), hash40("rise")].contains(&motion_kind) {
            if MotionModule::is_end(weapon.module_accessor) {
                MotionModule::set_rate(weapon.module_accessor, 0.0);
            }
        }
        if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) || GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            weapon.clear_lua_stack();
            lua_args!(weapon, *MA_MSC_CMD_EFFECT_EFFECT, hash40("sys_bomb_b"), hash40("rot"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
            sv_module_access::effect(weapon.lua_state_agent);
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            return 1.into();
        }
    }
    else {
        weapon.clear_lua_stack();
        lua_args!(weapon, *MA_MSC_CMD_EFFECT_EFFECT, hash40("sys_erace_smoke"), hash40("rot"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
        sv_module_access::effect(weapon.lua_state_agent);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn koopajr_cannonball_shoot_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_FREEZE_FRAME);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_IS_UP_INPUT);
    0.into()
}

pub fn install() {
    Agent::new("koopajr")
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_LW4, koopajr_attack_lw4_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, koopajr_attack_lw4_main_status)
    .status(MapCorrection, *FIGHTER_STATUS_KIND_ATTACK_LW4, koopajr_attack_lw4_map_correction_status)
    .status(Pre, *FIGHTER_KOOPAJR_STATUS_KIND_FLOAT, koopajr_float_pre_status)
    .status(Init, *FIGHTER_KOOPAJR_STATUS_KIND_FLOAT, koopajr_float_init_status)
    .status(Main, *FIGHTER_KOOPAJR_STATUS_KIND_FLOAT, koopajr_float_main_status)
    .status(End, *FIGHTER_KOOPAJR_STATUS_KIND_FLOAT, koopajr_float_end_status)
    .install()
    ;
    Agent::new("koopajr_cannonball")
    .status(Pre, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, koopajr_cannonball_shoot_pre_status)
    .status(Init, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, koopajr_cannonball_shoot_init_status)
    .status(Main, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, koopajr_cannonball_shoot_main_status)
    .status(End, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, koopajr_cannonball_shoot_end_status)
    .install()
    ;
}