use {
  exo_utils::{
    fighter_common::*,
    status_end_control::*,
  },
  exo_var::globals::*,
  smash::{
    app::Fighter,
    lib::L2CValue
  }
};

mod acmd;
mod vtable;

pub fn install() {
  acmd::install();
  vtable::install();
}