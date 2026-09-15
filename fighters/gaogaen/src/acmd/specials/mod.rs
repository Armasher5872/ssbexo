use {
    exo_var::gaogaen::*,
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

mod special_hi_cancel;
mod special_hi;
mod special_n;

pub fn install() {
    special_hi_cancel::install();
    special_hi::install();
    special_n::install();
}