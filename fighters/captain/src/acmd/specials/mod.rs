use {
    exo_utils::vector::*,
    exo_var::captain::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            }
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
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

mod special_air_lw;
mod special_n_charged;
mod special_n_hold;
mod special_n;

pub fn install() {
    special_air_lw::install();
    special_n_charged::install();
    special_n_hold::install();
    special_n::install();
}