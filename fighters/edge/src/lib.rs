use {
    exo_utils::{
        common::{
            extern_func::*,
            hook::*,
            var_reset::*,
            vtable_funcs::*,
            weapon::*,
        },
        fighter::edge::*,
        status::damage::*,
        structs::{
            collision_struct::*,
            getter_funcs::*,
            module_init::*,
            shielddata_struct::*,
            ui_manager::*,
        }
    },
    exo_var::{
        consts::*,
        edge::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smashline::*,
    smash_script::macros::*,
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
    unsafe {
        FIGHTER_EDGE_GENERATE_ARTICLE_ZANSHIN_SHOT += clone_weapon("link", *WEAPON_KIND_LINK_SWORD_BEAM, "edge", "swordbeamcloned", false);
    }
}