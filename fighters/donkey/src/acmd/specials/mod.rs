use {
    exo_var::donkey::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::{
        *,
        macros::*
    },
    smashline::{
        *,
        Priority::Low
    },
};

mod barrel_break;
mod barrel_roll;
mod special_hi;
mod special_lw;
mod special_s;

pub fn install() {
    barrel_break::install();
    barrel_roll::install();
    special_hi::install();
    special_lw::install();
    special_s::install();
}