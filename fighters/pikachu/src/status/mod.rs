use {
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
        phx::Hash40
    },
    smashline::*,
};

mod attack;
mod special_s_attack;
mod special_s_end;
mod special_s_hold;
mod special_s_weak;
mod special_s;

pub fn install() {
    attack::install();
    special_s_attack::install();
    special_s_end::install();
    special_s_hold::install();
    special_s_weak::install();
    special_s::install();
}