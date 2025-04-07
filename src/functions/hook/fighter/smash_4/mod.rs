use {
    crate::{
        functions::{
            ext::{
                fighter::{
                    common::*,
                    ike::*,
                    metaknight::*,
                    murabito_shizue_common::*,
                    pfushigisou::*,
                    purin::*,
                },
                utility::{
                    boma_ext::*,
                    misc::*,
                    get_objects::*,
                }
            },
            hook::{
                misc::*,
                ui::ui_manager::UiManager
            },
            var::{
                bayonetta::*,
                cloud::*,
                consts::*,
                gekkouga::*,
                globals::*,
                ken::*,
                koopajr::*,
                link::*,
                littlemac::*,
                miifighter::*,
                miiswordsman::*,
                murabito::*,
                palutena::*,
                reflet::*,
                rockman::*,
                rosetta::*,
                ryu::*,
                variables::*,
            },
            util::*,
        },
        miifighter::status::*,
        miiswordsman::status::{
            special_n::{
                special_n1::{
                    special_n1::*,
                    special_n1_loop::*,
                    special_n1_attack::*,
                },
                special_n2::{
                    special_n2::*,
                    special_n2_hold::*,
                    special_n2_fire::*,
                },
                special_n3::{
                    special_n3::*,
                    special_n3_slash::*,
                }
            },
            special_s::{
                special_s1::{
                    special_s1_start::*,
                    special_s1::*,
                    special_s1_end::*,
                },
                special_s2::{
                    special_s2::*,
                    special_s2_attack_1::*,
                    special_s2_attack_2::*,
                    special_s2_attack_3::*,
                }
            }
        },
    },
    smash::{
        app::{
            BattleObject,
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