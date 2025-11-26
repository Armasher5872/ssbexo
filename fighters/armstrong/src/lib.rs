#![allow(unused_assignments)]
use {
    exo_utils::{
        armstrong::*,
        catch::*,
        fighter_common::*,
        hook::*,
        status_end_control::*,
        vtable_funcs::*,
    },
    exo_var::{
        armstrong::*,
        consts::*,
        ganon::*,
        globals::*,
    },
    param_config::*,
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
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash2::app::{
        LinkEvent,
        LinkEventCapture
    },
    smash_script::*,
    smashline::*,
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("walk_accel_mul"), 0, 0.06));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("walk_accel_add"), 0, 0.01));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("walk_speed_max"), 0, 0.64));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("ground_brake"), 0, 0.15));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("dash_speed"), 0, 1.443));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("run_accel_mul"), 0, 0.015));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("run_accel_add"), 0, 0.15));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("run_speed_max"), 0, 1.36));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("jump_speed_x"), 0, 0.788));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("jump_speed_x_max"), 0, 1.443));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("jump_initial_y"), 0, 21.0));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("jump_y"), 0, 32.0));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("mini_jump_y"), 0, 15.0));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("jump_aerial_y"), 0, 30.35));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("air_accel_x_mul"), 0, 0.02));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("air_accel_x_add"), 0, 0.05));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("air_speed_x_stable"), 0, 0.875));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("air_brake_x"), 0, 0.01));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("air_accel_y"), 0, 0.13));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("air_speed_y_stable"), 0, 1.85));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("damage_fly_top_air_accel_y"), 0, 0.13));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("damage_fly_top_speed_y_stable"), 0, 1.85));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("dive_speed_y"), 0, 3.17));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("weight"), 0, 116.0));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("landing_attack_air_frame_n"), 0, 10.0));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("landing_attack_air_frame_f"), 0, 13.0));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("landing_attack_air_frame_b"), 0, 11.0));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("landing_attack_air_frame_hi"), 0, 12.0));
    update_float_2(*FIGHTER_KIND_GANON, vec![8, 9, 10, 11, 12, 13, 14, 15].clone(), (hash40("landing_attack_air_frame_lw"), 0, 16.0));
    acmd::install();
    status::install();
    vtable::install();
}