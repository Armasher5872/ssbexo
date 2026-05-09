use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
            vtable_funcs::*,
        },
        fighter::cloud::*,
        status::damage::*,
        structs::{
            collision_struct::*,
            getter_funcs::*,
            shielddata_struct::*,
            ui_manager::*,
            vector::*,
        }
    },
    exo_var::{
        cloud::*,
        consts::*,
        globals::*,
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
        lua2cpp::L2CFighterCommon,
        phx::*,
    },
    smashline::*,
    smash_script::macros::*,
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