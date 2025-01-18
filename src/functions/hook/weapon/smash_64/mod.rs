/*
use {
    crate::functions::{
        ext::utility::misc::*,
        hook::weapon::common::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
    }
};
*/

mod samusd;

pub fn install() {
    samusd::install();
}