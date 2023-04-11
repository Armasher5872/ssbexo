#![allow(unused_macros)]
use {
    crate::functions::variables::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
    },
    smashline::*,
    smash_script::*,
};

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn up_special_wall_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] += 1;
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_STATUS) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_wall_b"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_wall"), 0.0, 1.0, false, 0.0, false, false);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)*WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("speed_x_mul_on_stop_wall"))*-1.0;
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    let mut speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("speed_y_on_stop_wall"));
    if speed_y <  0.0{
        speed_y = 0.0;
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.sub_shift_status_main(L2CValue::Ptr(up_special_wall_main_loop as *const () as _))
}

pub unsafe fn up_special_wall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    false.into()
}

pub fn install() {
    install_status_scripts!(
        up_special_wall_status_main
    );
}