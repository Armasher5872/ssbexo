use {
    exo_utils::{
        common::{
            status_end_control::*,
            var_reset::*,
            weapon::*,
        },
        fighter::pfushigisou::*,
    },
    exo_var::{
        globals::*,
        pfushigisou::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

mod acmd;
mod opff;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
    clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "pfushigisou", "sludge", false);
}