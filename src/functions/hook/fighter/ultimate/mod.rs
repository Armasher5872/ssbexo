use {
    crate::{
        functions::{
            ext::{
                fighter::{
                    common::*,
                    murabito_shizue_common::*,
                },
                utility::{
                    boma_ext::*,
                    commandcat::*,
                }
            },
            var::{
                consts::*,
                demon::*,
                dolly::*,
                edge::*,
                element::*,
                gaogaen::*,
                globals::*,
                murabito::*,
                variables::*,
            },
            util::*,
        },
        jack::status::*,
    },
    smash::{
        app::{
            Fighter,
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
            Vector2f,
            Vector3f
        }
    },
    smash_script::*,
};

mod brave;
mod buddy;
mod demon;
mod dolly;
mod edge;
mod eflame;
mod elight;
mod gaogaen;
mod inkling;
mod jack;
mod krool;
mod master;
mod packun;
mod pickel;
mod richter;
mod ridley;
mod shizue;
mod simon;
mod tantan;
mod trail;

pub fn install() {
    brave::install();
    buddy::install();
    demon::install();
    dolly::install();
    edge::install();
    eflame::install();
    elight::install();
    gaogaen::install();
    inkling::install();
    jack::install();
    krool::install();
    master::install();
    packun::install();
    pickel::install();
    richter::install();
    ridley::install();
    shizue::install();
    simon::install();
    tantan::install();
    trail::install();
}