use {
    exo_utils::vector::*,
    exo_var::{
        consts::*,
        globals::*,
    },
    interpolation::Lerp,
    smash::{
        app::{
            lua_bind::{
                PostureModule,
                *
            },
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
            Vector2f,
            Vector3f
        }
    },
    smash_script::*,
};

mod escape_air;

pub fn install() {
    escape_air::install();
}