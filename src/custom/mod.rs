#![allow(
    unused_unsafe,
    unused_imports,
    non_snake_case,
    unused_macros,
    unused_must_use,
    clippy::borrow_interior_mutable_const,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::absurd_extreme_comparisons,
    clippy::cmp_null,
    clippy::if_same_then_else
)]

mod airturn;
mod asdi;
mod breverse;
mod customgamemodes;
mod djc;
mod edgecancels;
pub mod gliding;
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
  gliding::install();
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
