use {
    crate::functions::var::consts::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
    },
    smash_script::*,
    smashline::*,
};

mod grounded;

pub fn install() {
    grounded::install();
}