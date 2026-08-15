use {
    exo_utils::{
        structs::{
            getter_funcs::*,
            ui_manager::*,
        }
    },
    exo_var::mariod::*,
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

mod drcapsule_regular;
mod special_n;

pub fn install() {
    drcapsule_regular::install();
    special_n::install();
}