use {
    smash::{
        app::lua_bind::*,
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

mod shouldered_donkey;

pub fn install() {
    shouldered_donkey::install();
}