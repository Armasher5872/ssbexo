use {
    crate::functions::{
        ext::{
            fighter::link::*,
            utility::{
                boma_ext::*,
                commandcat::*,
            }
        },
        var::{
            consts::*,
            dedede::*,
            diddy::*,
            ike::*,
            kirby::*,
            link::*,
            metaknight::*,
            murabito::*,
            pfushigisou::*,
            robot::*,
            sonic::*,
            variables::*,
        }
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
            Vector3f
        }
    }
};

mod dedede;
mod diddy;
mod ike;
mod lucario;
mod lucas;
mod metaknight;
mod pfushigisou;
mod pikmin;
mod pit;
mod pitb;
mod plizardon;
mod ptrainer;
mod pzenigame;
mod robot;
mod snake;
mod sonic;
mod szerosuit;
mod toonlink;
mod wario;
mod wolf;

pub fn install() {
    dedede::install();
    diddy::install();
    ike::install();
    lucario::install();
    lucas::install();
    metaknight::install();
    pfushigisou::install();
    pikmin::install();
    pit::install();
    pitb::install();
    plizardon::install();
    ptrainer::install();
    pzenigame::install();
    robot::install();
    snake::install();
    sonic::install();
    szerosuit::install();
    toonlink::install();
    wario::install();
    wolf::install();
}