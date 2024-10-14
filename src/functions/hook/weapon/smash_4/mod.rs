use {
    crate::functions::{
        hook::weapon::common::*,
        var::{
            palutena::*,
            reflet::*,
        },
        util::*,
    },
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        phx::Hash40
    },
    smash_script::*,
};

mod palutena;
mod reflet;
mod rockman;

pub fn install() {
    palutena::install();
    reflet::install();
    rockman::install();
}