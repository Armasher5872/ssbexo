use {
    crate::functions::{
        ext::{
            fighter::common::*,
            utility::{
                boma_ext::*,
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
            BattleObjectModuleAccessor,
            lua_bind::{
                PostureModule,
                *
            },
            self,
            sv_information
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
            Vector3f,
            Vector4f
        }
    },
    smashline::*,
    smash_script::*,
    skyline::nn::ro::LookupSymbol
};

mod airturn;
mod breverse;
mod customgamemodes;
mod misc;

pub fn install() {
    airturn::install();
    breverse::install();
    customgamemodes::install();
    misc::install();
}