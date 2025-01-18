use {
    crate::functions::var::{
        link::*,
        variables::*,
    },
    smash::{
        app::{
            BattleObjectModuleAccessor,
            FighterUtil,
            lua_bind::*,
        },
        lib::lua_const::*,
    },
};

mod brawl;
mod common;
mod exo;
mod melee;
mod smash_4;
mod smash_64;
mod ultimate;

pub fn install() {
    brawl::install();
    common::install();
    exo::install();
    melee::install();
    smash_4::install();
    smash_64::install();
    ultimate::install();
}