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
        hook::ui::ui_manager::*,
        var::{
            chrom::*,
            consts::*,
            globals::*,
            marth::*,
            mewtwo::*,
            pichu::*,
            roy::*,
            sheik::*,
            variables::*,
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
    },
    smash_script::*,
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