use {
    exo_utils::fighter_common::*,
    exo_var::ike::*,
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

mod slash_shoot;
mod special_lw_end_max;
mod special_lw_end_middle;
mod special_lw_loop;
mod special_lw;
mod special_n;

pub fn install() {
    slash_shoot::install();
    special_lw_end_max::install();
    special_lw_end_middle::install();
    special_lw_loop::install();
    special_lw::install();
    special_n::install();
}