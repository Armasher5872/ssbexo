use {
  crate::status::common_func::*,
  exo_utils::{
    attack::*,
    attack_xx4::*,
    littlemac::*,
  },
  exo_var::{
    globals::*,
    littlemac::*,
  },
  smash::{
    app::{
      lua_bind::*,
      *
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::Vector2f
  },
  smash_script::*,
  smashline::*,
};

mod attack_100;
mod attack_air;
mod attack_dash;
mod attack_hi3;
mod attack_hi4_hold;
mod attack_hi4_start;
mod attack_hi4;
mod attack_lw3;
mod attack_lw4_hold;
mod attack_lw4_start;
mod attack_lw4;
mod attack_s3;
mod attack_s4_hold;
mod attack_s4_start;
mod attack_s4;
mod attack;
mod common_func;
mod special_hi_jump;
mod special_hi_start;
mod special_lw_hit;
mod special_lw;
mod special_n;
mod special_s_blow;
mod special_s_jump;

pub fn install() {
  attack_100::install();
  attack_air::install();
  attack_dash::install();
  attack_hi3::install();
  attack_hi4_hold::install();
  attack_hi4_start::install();
  attack_hi4::install();
  attack_lw3::install();
  attack_lw4_hold::install();
  attack_lw4_start::install();
  attack_lw4::install();
  attack_s3::install();
  attack_s4_hold::install();
  attack_s4_start::install();
  attack_s4::install();
  attack::install();
  special_hi_jump::install();
  special_hi_start::install();
  special_lw::install();
  special_lw_hit::install();
  special_n::install();
  special_s_blow::install();
  special_s_jump::install();
}