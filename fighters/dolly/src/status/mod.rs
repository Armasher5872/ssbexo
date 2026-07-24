use {
    exo_utils::structs::vector::*,
    exo_var::{
        consts::*,
        dolly::*,
        globals::*,
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
        lua2cpp::L2CFighterCommon,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

mod guard_off;
mod special_n;
mod super_special2;

pub fn install() {
    guard_off::install();
    special_n::install();
    super_special2::install();
}