use {
    crate::functions::{
        ext::utility::misc::*,
        var::variables::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
    },
    smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
    acmd::install();
    opff::install();
}