use {
    exo_utils::{
        common::{
            hook::*,
            var_reset::*,
            vtable_funcs::*,
            weapon::*,
        },
        fighter::{
            ganon::*,
            koopajr::*,
            springtrap::*,
        },
        status::{
            attack::*,
            damage::*,
        },
        structs::{
            collision_struct::*,
            getter_funcs::*,
            module_init::*,
            shielddata_struct::*,
            vector::*,
        }
    },
    exo_var::{
        consts::*,
        donkey::*,
        ganon::*,
        gekkouga::*,
        globals::*,
        koopajr::*,
        springtrap::*,
        variables::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::*
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
}