use {
    exo_var::{
        captain::*,
        globals::*,
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
    },
    smashline::*,
};

mod attack_hi3;
mod attack_lw3;
mod attack_s3;
mod attack_s4_start;
mod attack;
mod catch_attack;
mod catch_dash_pull;
mod catch_dash;
mod catch_pull;
mod catch;
mod special_lw_wall_end;
mod special_lw;
mod special_n;

pub fn install() {
    attack_hi3::install();
    attack_lw3::install();
    attack_s3::install();
    attack_s4_start::install();
    attack::install();
    catch_attack::install();
    catch_dash_pull::install();
    catch_dash::install();
    catch_pull::install();
    catch::install();
    special_lw_wall_end::install();
    special_lw::install();
    special_n::install();
}