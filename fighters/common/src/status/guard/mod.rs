use {
    exo_utils::{
        common::{
            hook::*,
            vtable_funcs::*,
        },
        structs::{
            attackinfo_struct::*,
            vector::*,
        }
    },
    exo_var::{
        consts::*,
        globals::*,
        variables::*,
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

mod guard_damage;
mod guard_off;
mod guard_on;
mod guard;

pub fn install() {
    guard_damage::install();
    guard_off::install();
    guard_on::install();
    guard::install();
}