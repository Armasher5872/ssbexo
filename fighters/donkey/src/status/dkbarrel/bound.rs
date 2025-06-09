use super::*;

//Barrel Bound Pre Status
unsafe extern "C" fn donkey_barrel_bound_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

//Barrel Bound Init Status
unsafe extern "C" fn donkey_barrel_bound_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("speed_max"));
    let gravity = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("gravity"));
    let brake_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("brake_x"));
    let bound_count = WorkModule::get_int(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_BOUND_COUNT);
    let angle: f32 = 70.0;
    let speed_x = angle.to_radians().sin()*speed_max;
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, (speed_x*lr)/2.0, 2.5/(bound_count as f32));
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -brake_x, -gravity);
    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 3.0, 3.0);
    WorkModule::set_int(weapon.module_accessor, -1, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    barrel_unlink(weapon);
    0.into()
}

//Barrel Bound Main Status
unsafe extern "C" fn donkey_barrel_bound_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("roll"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_bound_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_bound_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let hp = WorkModule::get_float(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    let lr = PostureModule::lr(weapon.module_accessor);
    let bound_count = WorkModule::get_int(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_BOUND_COUNT);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y-4.0, z: pos_z});
    if should_remove_barrel(weapon) {
        barrel_removal(weapon);
    }
    if life <= 40 || hp <= 0.0 {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK.into(), false.into());
    }
    println!("Speed X: {}", KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
    println!("Speed Y: {}", KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
    if weapon.sub_ground_module_is_touch_all_consider_speed().get_bool() {
        if GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            if bound_count < 3 {
                WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
                notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2f89bbb63a));
                WorkModule::inc_int(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_BOUND_COUNT);
                sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, (speed_x*lr)/1.1, 2.5/(bound_count as f32));
            }
            else {
                sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, (speed_x*lr), 0.0);
                sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_ROLL.into(), false.into());
            }
        }
    }
    if MotionModule::is_end(weapon.module_accessor) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("roll"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

//Barrel Bound Exec Status
unsafe extern "C" fn donkey_barrel_bound_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

//Barrel Bound End Status
unsafe extern "C" fn donkey_barrel_bound_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

//Barrel Bound Exit Status
unsafe extern "C" fn donkey_barrel_bound_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_DONKEY_BARREL_STATUS_KIND_BOUND, donkey_barrel_bound_pre_status)
    .status(Init, *WEAPON_DONKEY_BARREL_STATUS_KIND_BOUND, donkey_barrel_bound_init_status)
    .status(Main, *WEAPON_DONKEY_BARREL_STATUS_KIND_BOUND, donkey_barrel_bound_main_status)
    .status(Exec, *WEAPON_DONKEY_BARREL_STATUS_KIND_BOUND, donkey_barrel_bound_exec_status)
    .status(End, *WEAPON_DONKEY_BARREL_STATUS_KIND_BOUND, donkey_barrel_bound_end_status)
    .status(Exit, *WEAPON_DONKEY_BARREL_STATUS_KIND_BOUND, donkey_barrel_bound_exit_status)
    .install()
    ;
}