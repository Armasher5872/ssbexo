use {
    exo_utils::{
        extern_func::*,
        fighter_common::*,
    },
    exo_var::{
        globals::*,
        sonic::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod final_idle;
mod final_move_ball;
mod final_move_ball_end;
mod final_move_ball_start;
mod final_start;
mod final_turn;

pub fn install() {
    final_idle::install();
    final_move_ball::install();
    final_move_ball_end::install();
    final_move_ball_start::install();
    final_start::install();
    final_turn::install();
}