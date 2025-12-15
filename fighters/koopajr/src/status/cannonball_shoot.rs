use super::*;

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
    Agent::new("koopajr_cannonball")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, koopajr_cannonball_shoot_pre_status)
    .status(Init, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, koopajr_cannonball_shoot_init_status)
    .status(Main, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, koopajr_cannonball_shoot_main_status)
    .status(End, *WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT, koopajr_cannonball_shoot_end_status)
    .install()
    ;
}