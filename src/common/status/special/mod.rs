use {
    crate::functions::{
        ext::*,
        var::{
            globals::*,
            variables::*,
        }
    },
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon
    }
};

mod specialhi;

pub fn install() {
    specialhi::install();
}