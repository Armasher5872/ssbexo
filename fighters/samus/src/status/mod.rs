use {
    exo_utils::status::attack_dash::*,
    exo_var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::{
            L2CFighterCommon,
            *
        }
    },
    smashline::*,
};

mod attack_dash;
mod attack_hi3;
mod special_s1a;
mod special_s2a;

pub fn install() {
    attack_dash::install();
    attack_hi3::install();
    special_s1a::install();
    special_s2a::install();
}