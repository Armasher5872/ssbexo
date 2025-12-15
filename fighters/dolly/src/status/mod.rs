use {
    exo_var::{
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
        lua2cpp::L2CFighterCommon
    },
    smash_script::*,
    smashline::*,
};

mod special_n;
mod super_special2;

pub fn install() {
    special_n::install();
    super_special2::install();
}