use {
    crate::functions::ext::utility::misc::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::{
            L2CAgent,
            L2CValue,
            lua_const::*,
        }
    },
    smash_script::*,
};

mod snake;

pub fn install() {
    snake::install();
}