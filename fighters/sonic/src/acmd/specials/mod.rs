use {
    exo_utils::vector::*,
    exo_var::sonic::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::frame,
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
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

mod homingtarget_homing;
mod special_n_hit;
mod special_n_homing_start;
mod special_n_homing;
mod special_n_start;
mod special_s_rush_end;
mod special_s_rush;
mod special_s_start;

pub fn install() {
    homingtarget_homing::install();
    special_n_hit::install();
    special_n_homing_start::install();
    special_n_homing::install();
    special_n_start::install();
    special_s_rush_end::install();
    special_s_rush::install();
    special_s_start::install();
}