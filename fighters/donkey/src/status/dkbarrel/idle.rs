use super::*;

//Barrel Idle Pre Status
unsafe extern "C" fn donkey_barrel_idle_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

//Barrel Idle Init Status
unsafe extern "C" fn donkey_barrel_idle_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

//Barrel Idle Main Status
unsafe extern "C" fn donkey_barrel_idle_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("stop"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_idle_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_idle_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let gravity = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("gravity"));
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let hp = WorkModule::get_float(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y-4.0, z: pos_z});
    if should_remove_barrel(weapon) {
        barrel_removal(weapon);
    }
    if life <= 40 || hp <= 0.0 {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK.into(), false.into());
    }
    if !GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_BOUND_COUNT);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2f89bbb63a));
        weapon.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity);
    }
    if MotionModule::is_end(weapon.module_accessor) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("stop"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

//Barrel Idle Exec Status
unsafe extern "C" fn donkey_barrel_idle_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

//Barrel Idle End Status
unsafe extern "C" fn donkey_barrel_idle_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

//Barrel Idle Exit Status
unsafe extern "C" fn donkey_barrel_idle_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_pre_status)
    .status(Init, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_init_status)
    .status(Main, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_main_status)
    .status(Exec, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_exec_status)
    .status(End, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_end_status)
    .status(Exit, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_exit_status)
    .install()
    ;
}