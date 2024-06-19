use {
    crate::functions::{
        ext::utility::{
            boma_ext::*,
            commandcat::*,
            misc::*,
        },
        var::{
            consts::*,
            mewtwo::*,
            sheik::*,
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
            Vector2f,
            Vector3f
        }
    }
};

mod chrom;
mod daisy;
mod falco;
mod gamewatch;
mod koopa;
mod lucina;
mod mariod;
mod marth;
mod mewtwo;
mod nana;
mod peach;
mod pichu;
mod popo;
mod roy;
mod sheik;
mod younglink;
mod zelda;

pub fn install() {
    chrom::install();
    daisy::install();
    falco::install();
    gamewatch::install();
    koopa::install();
    lucina::install();
    mariod::install();
    marth::install();
    mewtwo::install();
    nana::install();
    peach::install();
    pichu::install();
    popo::install();
    roy::install();
    sheik::install();
    younglink::install();
    zelda::install();
}