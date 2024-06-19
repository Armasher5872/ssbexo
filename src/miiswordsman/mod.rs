use {
  crate::{
    functions::{
      ext::fighter::common::*,
      var::{
        globals::*,
        miiswordsman::*,
      }
    },
    miiswordsman::status::{
      special_n::{
        special_n1::{
          special_n1::*,
          special_n1_loop::*,
          special_n1_attack::*,
        },
        special_n2::{
          special_n2::*,
          special_n2_hold::*,
          special_n2_fire::*,
        },
        special_n3::{
          special_n3::*,
          special_n3_slash::*,
        }
      },
      special_s::{
        special_s1::{
          special_s1_start::*,
          special_s1::*,
          special_s1_end::*,
        },
        special_s2::{
          special_s2::*,
          special_s2_attack_1::*,
          special_s2_attack_2::*,
          special_s2_attack_3::*,
        },
        special_s3::special_s3::*,
      },
      special_hi::{
        special_hi1::{
          special_hi1::*,
          special_hi1_jump::*,
          special_hi1_loop::*,
        },
        special_hi2::special_hi2::*,
        special_hi3::special_hi3::*,
      },
      special_lw::{
        special_lw1::special_lw1::*,
        special_lw2::special_lw2::*,
        special_lw3::special_lw3::*,
      }
    },
  },
  smash::{
    app::lua_bind::*,
    lib::{
      L2CValue,
      lua_const::*,
    },
    lua2cpp::L2CFighterCommon
  },
  smashline::*,
};

mod acmd;
mod opff;
pub mod status;

pub fn install() {
  acmd::install();
  opff::install();
}