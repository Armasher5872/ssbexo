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

Weapon Attack Callback may be the 29th entry

Vtable Shared Offsets:

0x68d5a0: Bowser, Dark Samus, Diddy Kong, Ken, Megaman, Meta Knight, Ridley, Ryu, Samus
0x68d5e0: Charizard, Chrom, Daisy, Diddy Kong, Dr. Mario, Greninja, Ike, Ivysaur, Jigglypuff, Luigi, Mario, Meta Knight, Mewtwo, Ness, Palutena, Peach, Pichu, Pikachu, Piranha Plant, Robin, Rosalina & Luma, Roy, Senator Armstrong, Sonic, Squirtle, Wii Fit Trainer
0xc732a0: Lucas, Lucina, Marth
0xf95c90: Ivysaur, Squirtle
0xf6d5c0: Dark Pit, Pit
0xf6d1c0: Dark Pit, Pit
0xc280f0: Link, Toon Link, Young Link
0xc28280: Link, Toon Link, Young Link
0xa61650: Fox, Wolf
0xa617c0: Fox, Wolf
0xa62480: Fox, Wolf
0x10bb480: Chrom, Roy
0x10bbaa0: Chrom, Roy
0xe88ca0: Daisy, Peach
0xcd98a0: Lucina, Marth
0x68d670: Donkey Kong, Lucina, Marth, Mii Swordfighter
0xfb6750: Nana, Popo
0xfb6a50: Nana, Popo
0x68d880: Chrom, Daisy, Jigglypuff, Peach, Roy
0xf2a520: Pichu, Pikachu
0xf2a630: Pichu, Pikachu
0x819430: Bayonetta, Sephiroth
0x819440: Bayonetta, Sephiroth
0x10d4570: Ken, Ryu
0xdba810: Isabelle, Villager
0xdbab30: Isabelle, Villager
0x10f3630: Dark Samus, Samus
0x10f37a0: Dark Samus, Samus
0x1194120: Richter, Simon
0x1194130: Richter, Simon
*/