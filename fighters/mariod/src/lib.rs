use {
    exo_utils::{
        common::{
            hook::*,
            status_end_control::*,
            var_reset::*,
        },
        structs::{
            collision_struct::*,
            getter_funcs::*,
            ui_manager::*,
        }
    },
    exo_var::{
        globals::*,
        mariod::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
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
    update_weapon_count(*WEAPON_KIND_MARIOD_DRCAPSULE, 1);
}