use {
    exo_var::consts::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::frame,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
    },
    smash_script::macros::*,
    smashline::{
        *,
        Priority::Low
    },
};

mod grounded;
mod smashes;

pub fn install() {
    grounded::install();
    smashes::install();
}