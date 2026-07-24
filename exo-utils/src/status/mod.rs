#![allow(internal_features)]
use {
    crate::{
        common::{
            battle_object::*,
            extern_func::*,
            hook::*,
        },
        structs::{
            getter_funcs::*,
            knockback_calc_context::*,
            vector::*,
        }
    },
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
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smash_script::{
        *,
        macros::*
    }
};

pub mod appeal;
pub mod attack_dash;
pub mod attack_xx4;
pub mod attack;
pub mod catch;
pub mod damage;
pub mod glide;
pub mod knockback_func;