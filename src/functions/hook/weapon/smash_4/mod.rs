use {
    crate::functions::{
        ext::utility::misc::*,
        hook::weapon::common::*,
        var::{
            palutena::*,
            reflet::*,
        },
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
mod palutena;
mod reflet;
mod rockman;

pub fn install() {
    koopajr::install();
    palutena::install();
    reflet::install();
    rockman::install();
}