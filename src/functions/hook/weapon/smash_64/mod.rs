use {
    crate::functions::{
        ext::utility::boma_ext::*,
        var::ness::*,
    },
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
    }
};

mod ness;
mod samusd;

pub fn install() {
    ness::install();
    samusd::install();
}