use {
    exo_var::krool::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::frame,
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

mod special_lw_charge;
mod special_lw_launch;
mod special_lw_start;

pub fn install() {
    special_lw_charge::install();
    special_lw_launch::install();
    special_lw_start::install();
}