use {
    exo_var::{
        consts::*,
        globals::*,
        samusd::*,
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::{
            Vector2f,
            Vector3f
        }
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

mod special_hi;
mod special_s1a;
mod special_s2a;

pub fn install() {
    special_hi::install();
    special_s1a::install();
    special_s2a::install();
}