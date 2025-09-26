use super::*;

unsafe extern "C" fn link_swordbeam_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_motion_kind = MotionModule::motion_kind(owner_boma);
    let max_life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_swordbeam"), hash40("max_life"));
    let max_power = WorkModule::get_param_float(weapon.module_accessor, hash40("param_swordbeam"), hash40("max_power"));
    let max_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_swordbeam"), hash40("max_scale"));
    let max_speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_swordbeam"), hash40("max_speed_x"));
    let deccel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_swordbeam"), hash40("deccel_x"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let angle: f32 = 45.0;
    let speed_x = angle.to_radians().sin()*max_speed_x*lr;
    let speed_y = angle.to_radians().cos()*max_speed_x;
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int(weapon.module_accessor, max_life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, max_life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    if owner_motion_kind == hash40("attack_s3_s") {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, 0.0);
    }
    else if owner_motion_kind == hash40("attack_s3_hi") {
        PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 45.0}, 0);
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    }
    else {
        PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: -45.0}, 0);
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    }
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -deccel_x);
    AttackModule::set_power(weapon.module_accessor, 0, max_power, false);
    PostureModule::set_scale(weapon.module_accessor, max_scale, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        link_swordbeam_fly_sub_status(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(link_swordbeam_fly_sub_status as *const () as _));
    weapon.fastshift(L2CValue::Ptr(link_swordbeam_fly_main_loop as *const () as _))
}

unsafe extern "C" fn link_swordbeam_fly_sub_status(weapon: &mut L2CWeaponCommon, bool_check: L2CValue) -> L2CValue {
    if bool_check.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn link_swordbeam_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life > 0 {
        if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            weapon.change_status(WEAPON_LINK_SWORDBEAM_STATUS_KIND_HIT.into(), false.into());
        }
        else {
            if WorkModule::is_flag(weapon.module_accessor, *WEAPON_LINK_SWORDBEAM_INSTANCE_WORK_ID_FLAG_HIT) {
                weapon.change_status(WEAPON_LINK_SWORDBEAM_STATUS_KIND_HIT.into(), false.into());
            }
        }
    }
    else {
        weapon.change_status(WEAPON_LINK_SWORDBEAM_STATUS_KIND_VANISH.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("link_swordbeam")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WEAPON_LINK_SWORDBEAM_STATUS_KIND_FLY, link_swordbeam_fly_main_status)
    .install()
    ;
}