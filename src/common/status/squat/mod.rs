use smash::{
    app::lua_bind::*,
    lib::{
        L2CValue,
        lua_const::*,
    },
    lua2cpp::L2CFighterCommon
};

mod squat;

pub fn install() {
    squat::install();
}