use {
    exo_utils::fighter::gaogaen::*,
    exo_var::{
        gaogaen::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::*
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

mod attack_lw4;
mod attack;
mod catch_attack;
mod catch_dash_pull;
mod catch_dash;
mod catch_pull;
mod catch_turn;
mod catch_wait;
mod catch;
mod special_hi_turn;
mod special_hi;
mod special_lw_hit;
mod special_lw;
mod special_n;
mod throw;

pub fn install() {
    attack_lw4::install();
    attack::install();
    catch_attack::install();
    catch_dash_pull::install();
    catch_dash::install();
    catch_pull::install();
    catch_turn::install();
    catch_wait::install();
    catch::install();
    special_hi_turn::install();
    special_hi::install();
    special_lw_hit::install();
    special_lw::install();
    special_n::install();
    throw::install();
}