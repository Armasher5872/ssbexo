use {
    exo_utils::fighter::daisy::*,
    exo_var::globals::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod attack_air;
mod attack_s4_hold;
mod attack_s4;
mod special_hi;
mod special_lw;

pub fn install() {
    attack_air::install();
    attack_s4_hold::install();
    attack_s4::install();
    special_hi::install();
    special_lw::install();
}