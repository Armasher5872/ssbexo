use {
    crate::functions::{
        ext::{
            fighter::common::*,
            utility::boma_ext::*,
        },
        hook::attack::*,
        var::{
            armstrong::*,
            consts::*,
            globals::*,
            variables::*,
        },
        util::*,
    },
    smash::{
        app::{
            Fighter,
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
    }
};

mod armstrong;

pub fn install() {
    armstrong::install();
}