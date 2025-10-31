use {
    crate::status::common_func::*,
    exo_utils::fighter_common::*,
    exo_var::{
        consts::*,
        donkey::*,
        globals::*,
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
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod air_lasso_landing;
mod air_lasso;
mod catch_pull;
pub mod common_func;
mod shoulder_start;
mod special_hi;
mod special_n;
mod special_s;
mod throw_f_b;
mod throw_f_f;
mod throw_f_hi;
mod throw_f_lw;

pub fn install() {
    air_lasso_landing::install();
    air_lasso::install();
    catch_pull::install();
    shoulder_start::install();
    special_hi::install();
    special_n::install();
    special_s::install();
    throw_f_b::install();
    throw_f_f::install();
    throw_f_hi::install();
    throw_f_lw::install();
}