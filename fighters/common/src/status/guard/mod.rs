use {
    exo_var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
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
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
};

mod guard;
mod guard_on;
mod guard_damage;

pub fn install() {
    guard::install();
    guard_on::install();
    guard_damage::install();
}