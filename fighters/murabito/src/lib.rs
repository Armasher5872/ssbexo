use {
    exo_utils::{
        fighter_common::*,
        murabito_shizue_common::*,
        status_end_control::*,
    },
    exo_var::{
        globals::*,
        murabito::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
    },
    smashline::*,
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    status::install();
    vtable::install();
}