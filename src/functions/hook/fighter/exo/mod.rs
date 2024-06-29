use {
    crate::functions::{
        ext::utility::boma_ext::*,
        var::{
            armstrong::*,
            consts::*,
            variables::*,
        }
    },
    smash::{
        app::{
            Fighter,
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        phx::{
            Hash40,
            Vector3f
        }
    }
};

mod armstrong;

pub fn install() {
    armstrong::install();
}