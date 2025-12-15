use {
    exo_utils::kirby::*,
    exo_var::{
        consts::*,
        globals::*,
        kirby::*,
    },
    smash::{
        app::{
        lua_bind::*,
        *
        },
        hash40,
        lib::{
        L2CValue,
        lua_const::*,
        },
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod special_hi2;
mod special_hi3;
mod special_hi4;
mod special_hi5;
mod special_lw;
mod special_s;

pub fn install() {
    special_hi2::install();
    special_hi3::install();
    special_hi4::install();
    special_hi5::install();
    special_lw::install();
    special_s::install();
}