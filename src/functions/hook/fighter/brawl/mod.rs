use {
    crate::functions::{
        ext::{
            fighter::{
                common::*,
                link::*,
            },
            utility::{
                boma_ext::*,
                commandcat::*,
                misc::*,
            }
        },
        hook::attack::*,
        var::{
            consts::*,
            dedede::*,
            diddy::*,
            globals::*,
            ike::*,
            kirby::*,
            link::*,
            lucas::*,
            metaknight::*,
            murabito::*,
            pfushigisou::*,
            robot::*,
            sonic::*,
            variables::*,
        },
        util::*,
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
            Vector3f
        }
    },
    smash_script::*,
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
    pzenigame::install();
    robot::install();
    snake::install();
    sonic::install();
    szerosuit::install();
    toonlink::install();
    wario::install();
    wolf::install();
}