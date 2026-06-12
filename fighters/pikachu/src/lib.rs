use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
        },
        fighter::pikachu::*,
        status::damage::*,
        structs::collision_struct::*,
    },
    exo_var::{
        consts::*,
        globals::*,
        variables::*,
        pikachu::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
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
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
    clone_weapon("ganon", *WEAPON_KIND_GANON_SWORD, "pikachu", "swordcloned", false);
}