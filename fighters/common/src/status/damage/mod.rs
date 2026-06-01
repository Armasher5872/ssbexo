use {
    exo_utils::status::knockback_func::*,
    exo_var::globals::*,
    skyline::hooks::InlineCtx,
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
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::*,
    }
};

mod damage;
mod damagefly;
mod damageflyroll;

pub fn install() {
    damage::install();
    damagefly::install();
    damageflyroll::install();
}