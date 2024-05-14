use {
    crate::functions::var::{
        globals::*,
        variables::*,
        wolf::*,
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
        phx::Vector3f
    },
    smash_script::*,
    smashline::*,
};

mod special_hi_rush;
mod special_s;
mod special_s_rush;
mod special_s_end;

pub fn install() {
    special_hi_rush::install();
    special_s::install();
    special_s_rush::install();
    special_s_end::install();
}