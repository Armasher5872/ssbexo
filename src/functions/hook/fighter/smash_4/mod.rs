use {
    crate::functions::{
        ext::{
            fighter::murabito_shizue_common::*,
            utility::{
                boma_ext::*,
                misc::*,
                get_objects::*,
            }
        },
        var::{
            bayonetta::*,
            consts::*,
            link::*,
            littlemac::*,
            miiswordsman::*,
            murabito::*,
            rosetta::*,
            ryu::*,
            variables::*,
        }
    },
    smash::{
        app::{
            BattleObject,
            Fighter,
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        phx::{
            Hash40,
            Vector3f
        }
    }
};

mod bayonetta;
mod cloud;
mod duckhunt;
mod gekkouga;
mod kamui;
mod ken;
mod koopajr;
mod littlemac;
mod miifighter;
mod miigunner;
mod miiswordsman;
mod murabito;
mod pacman;
mod palutena;
mod reflet;
mod rockman;
mod rosetta;
mod ryu;
mod shulk;
mod wiifit;

pub fn install() {
    bayonetta::install();
    cloud::install();
    duckhunt::install();
    gekkouga::install();
    kamui::install();
    ken::install();
    koopajr::install();
    littlemac::install();
    miifighter::install();
    miigunner::install();
    miiswordsman::install();
    murabito::install();
    pacman::install();
    palutena::install();
    reflet::install();
    rockman::install();
    rosetta::install();
    ryu::install();
    shulk::install();
    wiifit::install();
}