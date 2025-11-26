use {
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
    }
};

mod cliff;

pub fn install() {
    cliff::install();
}