use {
    exo_utils::extern_func::*,
    exo_var::{
        consts::*,
        globals::*,
        sonic::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod final_smash;
mod special_hi;
mod special_n_hit;
mod special_s;
mod special_s_adventure;
mod special_s_boost;
mod special_s_bounce;
mod special_s_drop;
mod special_s_eagle;
mod super_sonic_status;

pub fn install() {
    final_smash::install();
    special_hi::install();
    special_n_hit::install();
    special_s::install();
    special_s_adventure::install();
    special_s_boost::install();
    special_s_bounce::install();
    special_s_drop::install();
    special_s_eagle::install();
    super_sonic_status::install();
}