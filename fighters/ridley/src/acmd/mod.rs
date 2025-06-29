use {
    exo_utils::vector::*,
    exo_var::consts::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::{
        *,
        macros::*
    },
    smashline::{
        *,
        Priority::Low
    },
};

mod grounded;
mod tilts;
mod smashes;
mod aerials;
mod throws;
mod specials;

pub fn install() {
  grounded::install();
  tilts::install();
  smashes::install();
  aerials::install();
  throws::install();
  specials::install();
}