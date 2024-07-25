use {
    crate::functions::{
        ext::fighter::common::*,
        var::{
            consts::*,
            globals::*,
            luigi::*,
        }
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
    },
    smashline::*,
};

mod catch;
mod catch_attack;
mod catch_cut;
mod catch_dash;
mod catch_dash_pull;
mod catch_pull;
mod catch_turn;
mod catch_wait;
mod special_lw_end;
mod special_lw_loop;
mod special_lw;
mod special_s_charge;
mod special_s_ram;
mod throw;

pub fn install() {
    catch::install();
    catch_attack::install();
    catch_cut::install();
    catch_dash::install();
    catch_dash_pull::install();
    catch_pull::install();
    catch_turn::install();
    catch_wait::install();
    special_lw_end::install();
    special_lw_loop::install();
    special_lw::install();
    special_s_charge::install();
    special_s_ram::install();
    throw::install();
}