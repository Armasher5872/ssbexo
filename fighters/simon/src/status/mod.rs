use {
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod attack_air;
mod special_lw;

pub fn install() {
    attack_air::install();
    special_lw::install();
}