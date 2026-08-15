use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            }
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::{
        Priority::Low,
        *
    },
};

mod appeal_hi;
mod attach_wall_climb;
mod attach_wall;
mod escape_air;
mod mortal_draw_attack;
mod mortal_draw_loop;

pub fn install() {
    appeal_hi::install();
    attach_wall_climb::install();
    attach_wall::install();
    escape_air::install();
    mortal_draw_attack::install();
    mortal_draw_loop::install();
}