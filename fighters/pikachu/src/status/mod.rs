use {
    exo_utils::{
        common::check_attack::*,
        structs::getter_funcs::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        pikachu::*,
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
        phx::*
    },
    smashline::*,
    smash_script::macros::*
};

mod special_hi;
mod special_s;

pub fn install() {
    special_hi::install();
    special_s::install();
}