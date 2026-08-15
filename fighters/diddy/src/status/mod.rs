use {
    exo_var::{
        diddy::*,
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
        phx::Hash40
    },
    smashline::*,
};

mod special_lw_laugh;

pub fn install() {
    special_lw_laugh::install();
}