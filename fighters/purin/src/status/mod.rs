use {
    exo_var::globals::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod attack_lw4;
mod special_hi;
mod special_lw;

pub fn install() {
    attack_lw4::install();
    special_hi::install();
    special_lw::install();
}