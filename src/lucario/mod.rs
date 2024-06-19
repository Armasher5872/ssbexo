use {
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
    acmd::install();
    opff::install();
}