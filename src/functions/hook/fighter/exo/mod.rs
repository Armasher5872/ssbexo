use {
    crate::functions::{
        ext::{
            fighter::common::*,
            utility::{
                boma_ext::*,
                misc::*,
            }
        },
        hook::{
            attack::*,
            misc::*,
        },
        var::{
            armstrong::*,
            consts::*,
            globals::*,
        },
        util::*,
    },
    smash::{
        app::{
            BattleObject,
            BattleObjectModuleAccessor,
            DamageNoReactionMode,
            Fighter,
            lua_bind::*,
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash2::app::{
        LinkEvent,
        LinkEventCapture
    },
    smash_script::*,
};

mod armstrong;

pub fn install() {
    armstrong::install();
}