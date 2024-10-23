use {
    crate::functions::var::{
        consts::*,
        globals::*,
    },
    smash::{
        app::lua_bind::{
            PostureModule,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Vector3f
    },
    smash_script::*,
};

pub mod attack_xx4;
pub mod attack;
pub mod damage;