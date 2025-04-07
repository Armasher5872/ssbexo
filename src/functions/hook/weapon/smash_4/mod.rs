use {
    crate::functions::{
        ext::utility::misc::*,
        hook::weapon::common::*,
        var::reflet::*,
        util::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
};

mod koopajr;
mod reflet;
mod rockman;

pub fn install() {
    koopajr::install();
    reflet::install();
    rockman::install();
}