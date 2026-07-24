use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
            weapon::*,
        },
        fighter::{
            krool::*,
            springtrap::*,
        },
        structs::{
            collision_struct::*,
            getter_funcs::*,
            module_init::*,
        }
    },
    exo_var::{
        consts::*,
        globals::*,
        krool::*,
        springtrap::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
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