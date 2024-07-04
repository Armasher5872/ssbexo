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
    }
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

/*
VTable Notation:

Fighter Vtable Location: 710529cfd0

Fighters have 146 entries

Weapon Vtable Location: 710529d400

Weapons presumably have 216 entries, need to research if weapons have a repeating pattern akin to fighters
*/