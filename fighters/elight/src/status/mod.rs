use {
    exo_var::element::*,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod attack_air;
mod attack_dash;
mod attack_s3;
mod attack_s4;
mod attack;
mod special_lw;
mod special_s_forward;
mod special_s;

pub fn install() {
    attack_air::install();
    attack_dash::install();
    attack_s3::install();
    attack_s4::install();
    attack::install();
    special_lw::install();
    special_s_forward::install();
    special_s::install();
}