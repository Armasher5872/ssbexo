use {
    crate::functions::ext::{
        fighter::common::*,
        utility::controls::*,
    },
    smash::{
        app::{
            BattleObject,
            BattleObjectModuleAccessor
        },
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