use {
    exo_var::{
        fox::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smashline::*,
    smash_script::*,
};

mod special_lw_end;
mod special_lw;
mod special_s;

pub fn install() {
    special_lw_end::install();
    special_lw::install();
    special_s::install();
}