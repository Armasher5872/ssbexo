use {
    exo_utils::{
        miifighter::*,
        vector::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        miifighter::*,
    },
    smash::{
        app::{
        lua_bind::*,
        *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Vector3f
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

pub mod special_lw1_attack;
pub mod special_lw1_charge;
pub mod special_lw1;
mod special_lw3_throw;
pub mod special_n3_dive;
pub mod special_n3_land;
pub mod special_n3_rise;
pub mod special_n3;
pub mod special_s1_end;
pub mod special_s1;

pub fn install() {
    special_lw3_throw::install();
}