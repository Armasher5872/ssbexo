use {
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

mod special_hi;
mod special_lw_hit;
mod special_n_turn;
mod special_s;

pub fn install() {
    special_hi::install();
    special_lw_hit::install();
    special_n_turn::install();
    special_s::install();
}