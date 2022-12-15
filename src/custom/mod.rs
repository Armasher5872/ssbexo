mod airturn;
mod asdi;
mod breverse;
mod customgamemodes;
mod djc;
mod edgecancels;
mod grabbreak;
mod main;
pub mod momentumtransfer;
mod perfectpivot;
mod roawavedash;
mod specialafterinfliction;

pub fn install() {
  airturn::install();
  asdi::install();
  breverse::install();
  customgamemodes::install();
  djc::install();
  edgecancels::install();
  grabbreak::install();
  main::install();
  momentumtransfer::install();
  perfectpivot::install();
  roawavedash::install();
  specialafterinfliction::install();
}
