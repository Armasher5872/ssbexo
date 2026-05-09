use {
    exo_utils::{
        common::{
            hook::*,
            var_reset::*,
        },
        fighter::luigi::*,
        status::damage::*,
        structs::collision_struct::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        luigi::*,
    },
    smash::{
            app::{
            lua_bind::*,
            *
        },
        hash40,
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