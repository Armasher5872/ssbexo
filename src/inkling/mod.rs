use {
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::*,
};

mod acmd;
pub mod opff;

pub fn install() {
  acmd::install();
}