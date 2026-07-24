use {
    exo_var::gaogaen::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::*
    },
    smash_script::{
        macros::{
            ATTACK_ABS,
            *
        },
        *
    },
    smashline::{
        Priority::Low,
        *
    },
};

mod catch_attack;
mod catch_dash_hi;
mod catch_dash_lw;
mod catch_dash;
mod catch_hi;
mod catch_lw;
mod catch_turn_hi;
mod catch_turn_lw;
mod catch_turn;
mod catch;
mod throw_b_revenge;
mod throw_f_revenge;
mod throw_hi_revenge;
mod throw_lw_revenge;

pub fn install() {
    catch_attack::install();
    catch_dash_hi::install();
    catch_dash_lw::install();
    catch_dash::install();
    catch_hi::install();
    catch_lw::install();
    catch_turn_hi::install();
    catch_turn_lw::install();
    catch_turn::install();
    catch::install();
    throw_b_revenge::install();
    throw_f_revenge::install();
    throw_hi_revenge::install();
    throw_lw_revenge::install();
}