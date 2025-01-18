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

mod brawl;
pub mod common;
mod melee;
mod smash_4;
mod smash_64;

pub fn install() {
    brawl::install();
    common::install();
    melee::install();
    smash_4::install();
    smash_64::install();
}