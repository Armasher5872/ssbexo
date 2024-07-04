#![allow(unused_parens)]
use {
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        lua2cpp::*,
    },
    smashline::*,
};

mod status;

pub fn install() {
    status::install();
}