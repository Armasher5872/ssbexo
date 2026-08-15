use {
    exo_utils::{
        common::check_attack::*,
        structs::getter_funcs::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        sheik::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smash_script::macros::*,
    smashline::*,
};

mod attack_air;
mod special_s;

pub fn install() {
    attack_air::install();
    special_s::install();
}