use super::*;

pub unsafe extern "C" fn miiswordsman_special_hi1_jump_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    original_status(Pre, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_JUMP)(fighter)
}

pub unsafe extern "C" fn miiswordsman_special_hi1_jump_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable") as u64, 0);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul") as u64, 0);
    let hi1_jump_mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi") as u64, hash40("hi1_jump_mul_x") as u64);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(mul_x_speed_max, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, (air_accel_x_mul*hi1_jump_mul_x));
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 100.0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi1_jump_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    original_status(Main, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_JUMP)(fighter)
}

pub unsafe extern "C" fn miiswordsman_special_hi1_jump_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let hi1_air_start_mul_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi") as u64, hash40("hi1_air_start_mul_y") as u64);
    let hi1_jump_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi") as u64, hash40("hi1_jump_speed_mul") as u64);
    let get_speed_y = {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)
    };
    let mut speed_mul: f32 = 1.0;
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_ROKET_UNDER_THRUST_START_SITUATION);
    if start_situation == *SITUATION_KIND_AIR {
        if 0.0 < get_speed_y {
            speed_mul = hi1_air_start_mul_y*hi1_jump_speed_mul;
        }
        sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_ROKET_UNDER_DISABLE_CONTROL_X) {
        sv_kinetic_energy!(clear_speed_ex, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_ROKET_UNDER_DISABLE_CONTROL_X);
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi1_jump_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    original_status(End, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_JUMP)(fighter)
}

pub unsafe extern "C" fn miiswordsman_special_hi1_jump_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_LOOP, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_END].contains(&status_kind) {
        EffectModule::kill_all(fighter.module_accessor, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP as u32, false, true);
    }
    0.into()
}