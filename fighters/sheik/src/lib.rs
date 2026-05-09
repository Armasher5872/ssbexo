use {
    exo_utils::{
        common::{
            check_attack::*,
            var_reset::*,
        },
        fighter::sheik::*,
        status::damage::*,
        structs::getter_funcs::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        sheik::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smash_script::macros::*,
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