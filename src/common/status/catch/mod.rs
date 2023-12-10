use {
    crate::functions::var::{
        globals::*,
        variables::*,
    },
    smash::{
        app::{
            lua_bind::{
                PostureModule,
                *
            },
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::Hash40
    },
    smash_script::*,
};

mod catch;
mod catchdash;
mod catchpull;
mod catchturn;

pub fn install() {
    catch::install();
    catchdash::install();
    catchpull::install();
    catchturn::install();
}