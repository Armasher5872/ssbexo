use {
    exo_utils::{
        common::{
            status_end_control::*,
            var_reset::*,
        },
        structs::ui_manager::*,
    },
    exo_var::globals::*,
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
    },
  smashline::*,
};

mod acmd;
mod opff;

pub fn install() {
    acmd::install();
    opff::install();
}