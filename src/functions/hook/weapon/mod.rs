mod brawl;
pub mod common;
mod melee;
mod smash_4;
mod smash_64;

pub fn install() {
    brawl::install();
    melee::install();
    smash_4::install();
    smash_64::install();
}