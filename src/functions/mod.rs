#![allow(dead_code, unused_must_use)]
use {
    crate::functions::ext::{
        fighter::common::*,
        utility::controls::*,
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
            StageManager
        },
        lua2cpp::L2CFighterCommon
    },
    skyline::nn::ro::LookupSymbol,
    std::sync::Once
};

pub mod ext;
pub mod hook;
pub mod offsets;
pub mod singletons;
pub mod util;
pub mod var;

pub fn install() {
    hook::install();
}