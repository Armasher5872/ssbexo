use {
    crate::functions::var::consts::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod grounded;
mod throws;

pub fn install() {
    grounded::install();
    throws::install();
}