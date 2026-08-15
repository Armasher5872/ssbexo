use {
    exo_utils::{
        common::weapon::*,
        fighter::pfushigisou::*,
    },
    exo_var::{
        globals::*,
        pfushigisou::*,
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
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod sludge_shoot;
mod special_n;

pub fn install() {
    sludge_shoot::install();
    special_n::install();
}