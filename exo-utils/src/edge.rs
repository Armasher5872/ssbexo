use super::*;

pub unsafe extern "C" fn edge_fire_fly_sub(weapon: &mut L2CWeaponCommon, status: L2CValue) -> L2CValue {
    let current_frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let facing = PostureModule::lr(weapon.module_accessor);
    let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_m"))*facing;
    if life <= 0 || (WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL) && current_frame <= 2.0) {
        weapon.change_status(status, false.into());
        return 1.into();
    }
    else {
        if weapon.sub_ground_module_is_touch_all_consider_speed().get_bool() {
            if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
                sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, 1.0);
                sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y*-1.0);
                return 0.into()
            }
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL);
            if current_frame > 1.0 {
                weapon.change_status(status, false.into());
                return 1.into()
            }
            StopModule::set_other_stop(weapon.module_accessor, 2, StopOtherKind(0));
        }
    }
    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_ATTACK) {
        return 0.into()
    }
    weapon.change_status(status, false.into());
    1.into()
}

pub unsafe extern "C" fn edge_special_kinetic_handler(fighter: &mut L2CFighterCommon, param_1: bool) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if !param_1 && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            return 0.into();
        }
        sv_kinetic_energy!(set_needs_set_param, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, false);
        if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            sv_kinetic_energy!(set_needs_set_param, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, false);
        }
    }
    0.into()
}