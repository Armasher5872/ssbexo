use {
    crate::functions::var::{
        consts::*,
        dolly::*,
        edge::*,
        jack::*,
        murabito::*,
        variables::*,
    },
    smash::{
        app::{
            Fighter,
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    }
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