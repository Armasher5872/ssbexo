use {
    exo_utils::{
        catch::*,
        extern_func::*,
        fighter_common::*,
    },
    exo_var::{
        globals::*,
        luigi::*,
    },
    smash::{
        app::{
            ArticleOperationTarget,
            lua_bind::*,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::*,
};

mod batabata;
mod catch_attack;
mod catch_cut;
mod catch_dash_pull;
mod catch_dash;
mod catch_pull;
mod catch_turn;
mod catch_wait;
mod catch;
mod obakyumu_catch;
mod obakyumu_special_lw_catch_pull;
mod obakyumu_special_lw_end;
mod obakyumu_special_lw_loop;
mod obakyumu_special_lw_plunger;
mod obakyumu_special_lw_throw;
mod special_lw_catch_pull;
mod special_lw_end;
mod special_lw_loop;
mod special_lw_plunger;
mod special_lw_throw;
mod special_lw;
mod special_n_attack;
mod special_n;
mod special_s_ram;
mod throw;

pub fn install() {
    batabata::install();
    catch_attack::install();
    catch_cut::install();
    catch_dash_pull::install();
    catch_dash::install();
    catch_pull::install();
    catch_turn::install();
    catch_wait::install();
    catch::install();
    obakyumu_catch::install();
    obakyumu_special_lw_catch_pull::install();
    obakyumu_special_lw_end::install();
    obakyumu_special_lw_loop::install();
    obakyumu_special_lw_plunger::install();
    obakyumu_special_lw_throw::install();
    special_lw_catch_pull::install();
    special_lw_end::install();
    special_lw_loop::install();
    special_lw_plunger::install();
    special_lw_throw::install();
    special_lw::install();
    special_n_attack::install();
    special_n::install();
    special_s_ram::install();
    throw::install();
}