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
  unsafe {
    // removes phantoms
    skyline::patching::Patch::in_text(0x3e6ce8).data(0x14000012u32);
  }
}
