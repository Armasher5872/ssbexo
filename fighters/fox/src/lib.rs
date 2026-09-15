use {
    exo_utils::{
        common::{
            status_end_control::*,
            var_reset::*,
        },
        structs::getter_funcs::*,
    },
    exo_var::{
        consts::*,
        fox::*,
        globals::*,
        wolf::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::*,
    },
    smashline::*,
};

mod acmd;
mod opff;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
}