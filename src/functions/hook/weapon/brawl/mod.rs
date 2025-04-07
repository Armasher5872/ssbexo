use {
    crate::functions::{
        ext::utility::misc::*,
        hook::misc::*,
        util::*,
        var::ike::*,
    },
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        phx::Hash40
    },
    smash_script::*,
};

mod ike;

pub fn install() {
    ike::install();
}