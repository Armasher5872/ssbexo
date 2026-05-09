use {
    exo_utils::{
        common::{
            fighter_common::*,
            hook::*,
            var_reset::*,
        },
        fighter::sonic::*,
        status::damage::*,
        structs::{
            collision_struct::*,
            getter_funcs::*,
            ui_manager::*,
            vector::*,
        }
    },
    exo_var::{
        consts::*,
        globals::*,
        sonic::*,
        variables::*,
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
        phx::*
    },
    smashline::*,
    smash_script::{
        macros::*,
        *
    }
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