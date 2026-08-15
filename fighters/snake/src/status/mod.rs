use {
    exo_var::{
        globals::*,
        snake::*,
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
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod attack_s4;
mod special_hi_hang;
mod special_hi;
mod special_n;

pub fn install() {
    attack_s4::install();
    special_hi_hang::install();
    special_hi::install();
    special_n::install();
}