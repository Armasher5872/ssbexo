use {
    crate::functions::var::{
        chrom::*,
        consts::*,
        littlemac::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon
    }
};

mod capturepulled;

pub fn install() {
    capturepulled::install();
}