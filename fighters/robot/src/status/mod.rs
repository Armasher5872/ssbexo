use {
    smash::{
        app::lua_bind::*,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod special_hi_attack;
mod special_hi;

pub fn install() {
    special_hi_attack::install();
    special_hi::install();
}