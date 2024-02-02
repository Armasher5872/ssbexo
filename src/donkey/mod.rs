use {
    crate::functions::{
        ext::*,
        var::{
            consts::*,
            donkey::*,
            globals::*,
            variables::*,
        }
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

mod acmd;
mod hook;
mod opff;
mod status;

pub fn install() {
    acmd::install();
    hook::install();
    opff::install();
    status::install();
}