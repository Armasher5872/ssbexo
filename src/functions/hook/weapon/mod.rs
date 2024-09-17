use {
    crate::functions::{
        ext::utility::boma_ext::*,
        var::ness::*,
    },
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
    },
};

mod common;
mod smash_4;

pub fn install() {
    common::install();
    smash_4::install();
}