#![allow(dead_code, unused_must_use)]
use {
    crate::functions::{
        ext::utility::{
            boma_ext::*,
            controls::*,
        },
        var::{
            armstrong::*,
            captain::*,
            consts::*,
            dedede::*,
            dolly::*,
            edge::*,
            globals::*,
            ike::*,
            kirby::*,
            link::*,
            littlemac::*,
            luigi::*,
            marth::*,
            metaknight::*,
            mewtwo::*,
            miiswordsman::*,
            murabito::*,
            pfushigisou::*,
            pikachu::*,
            purin::*,
            robot::*,
            rosetta::*,
            ryu::*,
            samusd::*,
            sheik::*,
            sonic::*,
            variables::*,
        }
    },
    smash::{
        app::{
            BattleObject,
            BattleObjectWorld,
            BattleObjectSlow,
            BattleObjectManager,
            BattleObjectModuleAccessor,
            BossManager,
            FighterBayonettaFinalModule,
            FighterCutInManager,
            FighterManager,
            FighterParamAccessor2,
            FighterPitBFinalModule,
            GimmickEventPresenter,
            ItemManager,
            ItemParamAccessor,
            lua_bind::{
                PostureModule,
                *
            },
            StageManager,
            sv_battle_object
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smashline::*,
    skyline::nn::ro::LookupSymbol,
    std::sync::Once
};

pub mod ext;
pub mod hook;
pub mod offsets;
pub mod params;
pub mod reset;
pub mod singletons;
pub mod util;
pub mod var;

pub fn install() {
    hook::install();
	params::install();
	reset::install();
}