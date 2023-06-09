use {
    crate::functions::var::globals::*,
    smash::{
        app::lua_bind::*,
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        }
    }
};

mod itemthrow;

pub fn install() {
    itemthrow::install();
}