use super::*;

pub unsafe extern "C" fn miiswordsman_special_lw1_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

pub unsafe extern "C" fn miiswordsman_special_lw1_start_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x = 0.0;
    let y = 0.0;
    let mut power;
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let lw1_start_mul_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_start_mul_spd_x") as u64);
    let lw1_attack_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_attack_max_y") as u64);
    let lw1_attack_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_attack_acl_y") as u64);
    let lw1_start_air_acl_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_start_air_acl_x") as u64);
    let lw1_attack_power_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_attack_power_limit") as u64);
    let lw1_attack_max_for_enemy = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_attack_max_for_enemy") as u64);
    let lw1_attack_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_attack_max") as u64);
    let lw1_attack_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_attack_mul") as u64);
    let get_sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    vector["x"].assign(&L2CValue::F32(get_sum_speed));
    vector["y"].assign(&L2CValue::F32(get_sum_speed));
    if ![*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT].contains(&status_kind) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_IS_ATTACK_ENEMY) {
            power = lw1_attack_max_for_enemy;
        }
        else {
            power = lw1_attack_max;
        }
        if power < lw1_attack_power_limit {
            WorkModule::set_float(fighter.module_accessor, lw1_attack_power_limit*lw1_attack_mul, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_FLOAT_ATTACK_POWER);
        }
        if lw1_attack_max < power
        || lw1_attack_max_for_enemy < power {
            power = lw1_attack_max;
        }
        WorkModule::set_float(fighter.module_accessor, power*lw1_attack_mul, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_FLOAT_ATTACK_POWER);
    }
    else {
        if situation_kind != *SITUATION_KIND_AIR {
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
        }
        else {
            vector["x"].assign(&L2CValue::F32(lw1_start_mul_spd_x));
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, vec_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, lw1_start_air_acl_x);
            sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, vec_y, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -lw1_attack_acl_y);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, lw1_attack_max_y);
            sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
        }
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_lw1_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

pub unsafe extern "C" fn miiswordsman_special_lw1_start_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x = 0.0;
    let y = 0.0;
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let situation_prev = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
    let lw1_attack_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_attack_max_y") as u64);
    let lw1_attack_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_attack_acl_y") as u64);
    let lw1_start_air_acl_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw1_start_air_acl_x") as u64);
    let get_sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    vector["x"].assign(&L2CValue::F32(get_sum_speed));
    vector["y"].assign(&L2CValue::F32(get_sum_speed));
    if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT
    && situation_prev != situation_kind {
        if situation_kind != *SITUATION_KIND_AIR {
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
        }
        else {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, vec_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, lw1_start_air_acl_x);
            sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, vec_y, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -lw1_attack_acl_y);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, lw1_attack_max_y);
            sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_INT_SITUATION_PREV);
        }
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_lw1_start_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

pub unsafe extern "C" fn miiswordsman_special_lw1_start_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD_CHK) {
        ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_MIISWORDSMAN_SHIELD_GROUP_KIND_COUNTER_GUARD);
    }
    0.into()
}