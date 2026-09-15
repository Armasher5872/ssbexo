use {
    exo_utils::{
        common::{
            extern_func::*,
            hook::*,
            var_reset::*,
        },
        fighter::inkling::*,
        structs::getter_funcs::*,
    },
    exo_var::inkling::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
    },
    smashline::*,
};

mod acmd;
mod status;
mod vtable;

pub fn install() {
    acmd::install();
    status::install();
    vtable::install();
    update_weapon_count(*WEAPON_KIND_INKLING_ROLLERINK, 20);
}