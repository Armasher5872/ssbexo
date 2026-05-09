use {
    exo_utils::{
        common::{
            status_end_control::*,
            var_reset::*,
        },
        structs::{
            getter_funcs::*,
            ui_manager::*,
        }
    },
    exo_var::{
        globals::*,
        mariod::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
    },
    smash_script::*,
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