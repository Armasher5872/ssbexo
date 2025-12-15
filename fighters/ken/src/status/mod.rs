use {
    exo_var::{
        consts::*,
        globals::*,
        ken::*,
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
    },
    smash_script::{
        *,
        macros::*
    },
    smashline::*,
};

mod attack_dash;
mod attack_s3;

pub fn install() {
    attack_dash::install();
    attack_s3::install();
}