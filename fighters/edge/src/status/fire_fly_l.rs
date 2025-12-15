use super::*;

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

pub fn install() {
    Agent::new("edge_fire")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L, edge_fire_l_fly_main_status)
    .status(Exec, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L, edge_fire_l_fly_exec_status)
    .install()
    ;
}