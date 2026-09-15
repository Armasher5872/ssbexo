use {
    exo_utils::structs::vector::*,
    exo_var::{
        globals::*,
        wolf::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smashline::*,
    smash_script::*
};

mod special_s_end;
mod special_s_rush;
mod special_s;

pub fn install() {
    special_s_end::install();
    special_s_rush::install();
    special_s::install();
}