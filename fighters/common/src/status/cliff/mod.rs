use {
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
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::*,
        phx::{
            Hash40,
            Vector2f
        }
    },
    smash_script::*,
};

mod cliff_attack;
mod cliff_catch;
mod cliff_climb;
mod cliff_escape;
mod cliff_jump1;
mod cliff_jump2;
mod cliff_jump3;
mod cliff_robbed;
mod cliff;

pub fn install() {
    cliff_attack::install();
    cliff_catch::install();
    cliff_climb::install();
    cliff_escape::install();
    cliff_jump1::install();
    cliff_jump2::install();
    cliff_jump3::install();
    cliff_robbed::install();
    cliff::install();
}