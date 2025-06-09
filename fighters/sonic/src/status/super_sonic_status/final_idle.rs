use super::*;

unsafe extern "C" fn supersonic_final_idle_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_SONIC_SUPERSONIC_INSTANCE_WORK_ID_FLOAT_X_ACCEL);
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_SONIC_SUPERSONIC_INSTANCE_WORK_ID_FLOAT_Y_ACCEL);
    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 4.0, 4.0);
    0.into()
}

unsafe extern "C" fn supersonic_final_idle_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("final_idle"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(supersonic_final_idle_main_loop as *const () as _))
}

unsafe extern "C" fn supersonic_final_idle_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let timer = WorkModule::get_int(owner_boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_FINAL_SMASH_TIMER);
    let stick_x = ControlModule::get_stick_x(weapon.module_accessor);
    let lr = PostureModule::lr(weapon.module_accessor);
    WorkModule::inc_int(owner_boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_FINAL_SMASH_TIMER);
    if timer >= 900 {
        weapon.change_status(WEAPON_SONIC_SUPERSONIC_STATUS_KIND_PRE_END.into(), false.into());
        StatusModule::change_status_request_from_script(owner_boma, *FIGHTER_SONIC_STATUS_KIND_FINAL_END, false);
    }
    if stick_x*lr >= 0.8 {
        weapon.change_status(WEAPON_SONIC_SUPERSONIC_STATUS_KIND_MOVE_BALL_START.into(), false.into());
    }
    if stick_x*lr <= -0.6 {
        weapon.change_status(WEAPON_SONIC_SUPERSONIC_STATUS_KIND_TURN.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn supersonic_final_idle_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_TYPE_NORMAL);
    let get_speed_y = sv_kinetic_energy::get_speed_y(weapon.lua_state_agent);
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_TYPE_NORMAL);
    let get_speed_x = sv_kinetic_energy::get_speed_x(weapon.lua_state_agent);
    let accel_x = WorkModule::get_float(weapon.module_accessor, *WEAPON_SONIC_SUPERSONIC_INSTANCE_WORK_ID_FLOAT_X_ACCEL);
    let accel_y = WorkModule::get_float(weapon.module_accessor, *WEAPON_SONIC_SUPERSONIC_INSTANCE_WORK_ID_FLOAT_Y_ACCEL);
    let stick_y = ControlModule::get_stick_y(weapon.module_accessor);
    if stick_y > 0.3 {
        WorkModule::set_float(weapon.module_accessor, 0.2, *WEAPON_SONIC_SUPERSONIC_INSTANCE_WORK_ID_FLOAT_Y_ACCEL);
    }
    if stick_y < -0.3 {
        WorkModule::set_float(weapon.module_accessor, -0.2, *WEAPON_SONIC_SUPERSONIC_INSTANCE_WORK_ID_FLOAT_Y_ACCEL);
    }
    if stick_y < 0.3 && stick_y > -0.3 {
        WorkModule::set_float(weapon.module_accessor, accel_y*0.5, *WEAPON_SONIC_SUPERSONIC_INSTANCE_WORK_ID_FLOAT_Y_ACCEL);
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, get_speed_x*0.95, get_speed_y*0.95);
        if accel_y < 0.005 && accel_y > -0.005 {
            WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_SONIC_SUPERSONIC_INSTANCE_WORK_ID_FLOAT_Y_ACCEL);
        }
        if get_speed_y < 0.005 && get_speed_y > -0.005 {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, get_speed_x, 0.0);
        }
    }
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, accel_y);
    0.into()
}

unsafe extern "C" fn supersonic_final_idle_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let status_kind = weapon.global_table[STATUS_KIND].get_i32();
    if ![*WEAPON_SONIC_SUPERSONIC_STATUS_KIND_IDLE, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_TURN, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_MOVE_BALL_START, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_MOVE_BALL, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_MOVE_BALL_END].contains(&status_kind) {
        WorkModule::set_int(owner_boma, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_FINAL_SMASH_TIMER);
    }
    0.into()
}

pub fn install() {
    Agent::new("sonic_supersonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_IDLE, supersonic_final_idle_init_status)
    .status(Main, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_IDLE, supersonic_final_idle_main_status)
    .status(Exec, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_IDLE, supersonic_final_idle_exec_status)
    .status(End, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_IDLE, supersonic_final_idle_end_status)
    .install()
    ;
}