use {
    crate::{
        common::status::grounded::dash::fgc_dashback_main,
        functions::{
            ext::utility::{
                boma_ext::*,
                commandcat::*,
            },
            var::{
                globals::*,
                ken::*,
            }
        }
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        }
    },
    smash_script::*,
    smashline::*,
};

mod acmd;
mod status;

pub fn install() {
    acmd::install();
    status::install();
}