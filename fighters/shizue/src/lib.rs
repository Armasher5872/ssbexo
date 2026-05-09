use {
    exo_utils::{
        common::{
            status_end_control::*,
            var_reset::*,
        },
        fighter::{
            murabito_shizue_common::*,
            shizue::*,
        }
    },
    exo_var::globals::*,
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
mod opff;
mod vtable;
  
pub fn install() {
    acmd::install();
    opff::install();
    vtable::install();
}