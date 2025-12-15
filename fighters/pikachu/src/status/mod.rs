use {
    exo_var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smashline::*,
};

mod special_s_attack;
mod special_s_end;
mod special_s_hold;
mod special_s_weak;
mod special_s;

pub fn install() {
    special_s_attack::install();
    special_s_end::install();
    special_s_hold::install();
    special_s_weak::install();
    special_s::install();
}