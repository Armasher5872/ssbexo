use {
    crate::functions::{
        ext::utility::misc::*,
        var::ike::*,
    },
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        phx::Hash40
    }
};

mod ike;

pub fn install() {
    ike::install();
}