use {
    exo_utils::{
        status::knockback_func::*,
        structs::vector::*,
    },
    exo_var::{
        consts::*,
        globals::*,
    },
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
    },
    smash_script::{
        macros::*,
        *
    },
};

mod damage;
mod damagefly;
mod damageflyroll;
mod shield_break_fall;
mod shield_break_fly;

pub fn install() {
    damage::install();
    damagefly::install();
    damageflyroll::install();
    shield_break_fall::install();
    shield_break_fly::install();
}