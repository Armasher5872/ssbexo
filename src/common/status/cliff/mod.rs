use {
    crate::functions::var::consts::*,
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

mod cliff_attack;
mod cliff_catch_move;
mod cliff_catch;
mod cliff_climb;
mod cliff_escape;
mod cliff_jump1;
mod cliff_jump2;
mod cliff_jump3;
mod cliff_robbed;
mod cliff_wait;

pub fn install() {
    cliff_attack::install();
    cliff_catch_move::install();
    cliff_catch::install();
    cliff_climb::install();
    cliff_escape::install();
    cliff_jump1::install();
    cliff_jump2::install();
    cliff_jump3::install();
    cliff_robbed::install();
    cliff_wait::install();
}