use {
    exo_utils::structs::vector::*,
    exo_var::{
        consts::*,
        ganon::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait_loop_sync_mot,
                wait
            },
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::{
        *,
        Priority::Low
    },
};

mod appeal_hi;
mod appeal_s_attack;
mod appeal_s;
mod dash;
mod escape_air;
mod escape_b;
mod guard_off;
mod guard_on;
mod run;
mod turn_dash;
mod walk_fast;
mod walk_middle;
mod walk_slow;
mod win_1_wait;
mod win_1;

pub fn install() {
    appeal_hi::install();
    appeal_s_attack::install();
    appeal_s::install();
    dash::install();
    escape_air::install();
    escape_b::install();
    guard_off::install();
    guard_on::install();
    run::install();
    turn_dash::install();
    walk_fast::install();
    walk_middle::install();
    walk_slow::install();
    win_1_wait::install();
    win_1::install();
}