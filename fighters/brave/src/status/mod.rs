use {
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        lua2cpp::*,
    },
    smashline::*,
};

mod special_lw;

pub fn install() {
    special_lw::install();
}