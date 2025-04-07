use {
    crate::functions::{
        ext::{
            fighter::common::*,
            utility::controls::*,
        },
        hook::misc::*,
    },
    smash::{
        app::{
            BattleObject,
            BattleObjectModuleAccessor,
            lua_bind::*,
        },
        lib::lua_const::*,
        lua2cpp::{
            L2CFighterCommon,
            L2CWeaponCommon
        }
    }
};

pub mod ext;
pub mod hook;
pub mod util;
pub mod var;

pub fn install() {
    hook::install();
}