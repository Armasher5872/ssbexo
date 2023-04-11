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
mod breverse;
mod commonstatuses;
mod customgamemodes;
mod edgecancels;
pub mod gliding;
mod main;
pub mod momentumtransfer;
mod perfectpivot;
mod roawavedash;
mod specialafterinfliction;

pub fn install() {
  airturn::install();
  breverse::install();
  commonstatuses::install();
  customgamemodes::install();
  edgecancels::install();
  gliding::install();
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
