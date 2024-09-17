use {
    crate::functions::{
        var::reflet::*,
        util::*,
    },
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        phx::Hash40
    },
    smash_script::*,
};

mod reflet;

pub fn install() {
    reflet::install();
}