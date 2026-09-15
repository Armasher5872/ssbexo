use {
    exo_utils::structs::buttons::*,
    exo_var::{
        consts::*,
        globals::*,
    },
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

mod is_dive;

pub fn install() {
    is_dive::install();
}