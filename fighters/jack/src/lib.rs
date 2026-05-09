use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
            waza_customize::*,
        },
        structs::{
            collision_struct::*,
            getter_funcs::*,
        }
    },
    exo_var::globals::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon
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