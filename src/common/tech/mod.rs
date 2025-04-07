use {
    crate::functions::{
        ext::{
            fighter::common::*,
            utility::{
                commandcat::*,
                misc::*,
            }
        },
        var::{
            consts::*,
            globals::*,
            variables::*,
        }
    },
    smash::{
        app::{
            lua_bind::*,
            self,
            sv_information,
            *
        },
        hash40,
        lib::{
            L2CValue,
            L2CValueType,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    },
    smashline::*,
    smash_script::*,
    skyline::nn::ro::LookupSymbol
};

mod customgamemodes;
mod misc;

pub fn install() {
    customgamemodes::install();
    misc::install();
}