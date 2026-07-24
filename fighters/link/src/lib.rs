use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
            weapon::*,
        },
        fighter::link::*,
        status::damage::*,
        structs::{
            collision_struct::*,
            getter_funcs::*,
            module_init::*,
            ui_manager::*,
        }
    },
    exo_var::{
        consts::*,
        edge::*,
        globals::*,
        link::*,
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
        phx::*,
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