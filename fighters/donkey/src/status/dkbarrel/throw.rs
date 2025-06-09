use super::*;

//Barrel Throw Pre Status
unsafe extern "C" fn donkey_barrel_throw_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

//Barrel Throw Init Status
unsafe extern "C" fn donkey_barrel_throw_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    let owner_lr = PostureModule::lr(owner_boma);
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("speed_max"));
    let gravity = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("gravity"));
    let brake_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("brake_x"));
    let angle: f32 = match owner_status_kind {
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_F => 40.0,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_B => 100.0,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_HI => 80.0,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_LW => 50.0,
        _ => 0.0
    };
    let speed_x: f32 = match owner_status_kind {
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_F => angle.to_radians().sin()*speed_max,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_B => angle.to_radians().sin()*speed_max,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_HI => (angle.to_radians().sin()*speed_max)/4.0,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_LW => (angle.to_radians().sin()*speed_max)*2.0,
        _ => 0.0
    };
    let speed_y: f32 = match owner_status_kind {
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_F => angle.to_radians().cos()*speed_max,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_B => (angle.to_radians().cos()*speed_max)*2.0,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_HI => (angle.to_radians().cos()*speed_max)*5.0,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_LW => (angle.to_radians().cos()*speed_max)/4.0,
        _ => 0.0
    };
    let attack_angle: u64 = match owner_status_kind {
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_F => 45,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_B => 135,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_HI => 90,
        _ if owner_status_kind == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_LW => 15,
        _ => 361
    };
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*owner_lr, speed_y);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -brake_x*owner_lr, -gravity);
    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 3.0, 3.0);
    WorkModule::set_int(weapon.module_accessor, attack_angle as i32, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_THROW_ANGLE);
    WorkModule::set_int(weapon.module_accessor, -1, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    barrel_unlink(weapon);
    0.into()
}

//Barrel Throw Main Status
unsafe extern "C" fn donkey_barrel_throw_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ReflectorModule::set_status_all(weapon.module_accessor, ShieldStatus(*SHIELD_STATUS_NORMAL_GLOBAL), 0);
    ReflectorModule::set_no_team(weapon.module_accessor, true);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("roll"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_throw_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_throw_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let hp = WorkModule::get_float(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
    if should_remove_barrel(weapon) {
        barrel_removal(weapon);
    }
    if life <= 40 || hp <= 0.0 {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK.into(), false.into());
    }
    if GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2f89bbb63a));
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: PostureModule::pos_x(weapon.module_accessor), y: PostureModule::pos_y(weapon.module_accessor)-4.0, z: PostureModule::pos_z(weapon.module_accessor)});
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_BOUND.into(), false.into());
    }
    if MotionModule::is_end(weapon.module_accessor) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("roll"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

//Barrel Throw Exec Status
unsafe extern "C" fn donkey_barrel_throw_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(weapon.module_accessor);
    let gravity = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("gravity"));
    if speed_x*lr < 0.0 {
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity);
    }
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

//Barrel Throw End Status
unsafe extern "C" fn donkey_barrel_throw_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    barrel_unlink(weapon);
    0.into()
}

//Barrel Throw Exit Status
unsafe extern "C" fn donkey_barrel_throw_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    barrel_unlink(weapon);
    0.into()
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_pre_status)
    .status(Init, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_init_status)
    .status(Main, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_main_status)
    .status(Exec, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_exec_status)
    .status(End, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_end_status)
    .status(Exit, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_exit_status)
    .install()
    ;
}