use {
    exo_utils::{
        damage::*,
        knockback::*,
        vector::*,
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
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
};

mod damage;
mod damageair;
mod damagefly;
mod damageflyroll;
mod damagesleepfall;

pub fn install() {
    damage::install();
    damageair::install();
    damagefly::install();
    damageflyroll::install();
    damagesleepfall::install();
}