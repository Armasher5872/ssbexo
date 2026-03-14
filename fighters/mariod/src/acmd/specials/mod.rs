use {
    exo_utils::fighter_common::*,
    exo_var::mariod::*,
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

mod drcapsule_regular;
mod special_lw;
mod special_s;

pub fn install() {
    drcapsule_regular::install();
    special_lw::install();
    special_s::install();
}