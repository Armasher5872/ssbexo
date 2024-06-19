use {
    crate::functions::{
        ext::utility::{
            boma_ext::*,
            commandcat::*,
            misc::*,
        },
        var::{
            captain::*,
            consts::*,
            donkey::*,
            kirby::*,
            link::*,
            luigi::*,
            pikachu::*,
            purin::*,
            samusd::*,
            variables::*,
        }
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
        lib::lua_const::*,
        phx::{
            Hash40,
            Vector3f,
            Vector4f
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