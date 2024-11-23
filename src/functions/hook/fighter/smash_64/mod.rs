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
            captain::*,
            consts::*,
            donkey::*,
            globals::*,
            kirby::*,
            link::*,
            luigi::*,
            ness::*,
            pikachu::*,
            purin::*,
            samusd::*,
        },
        util::*,
    },
    smash::{
        app::{
            Fighter,
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
            Vector3f
        }
    }
};

mod captain;
mod donkey;
mod fox;
mod kirby;
mod link;
mod luigi;
mod mario;
mod ness;
mod pikachu;
mod purin;
mod samus;
mod samusd;
mod yoshi;

pub fn install() {
    captain::install();
    donkey::install();
    fox::install();
    kirby::install();
    link::install();
    luigi::install();
    mario::install();
    ness::install();
    pikachu::install();
    purin::install();
    samus::install();
    samusd::install();
    yoshi::install();
}