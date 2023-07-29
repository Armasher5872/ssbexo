use {
    bitflags::bitflags,
    crate::functions::{
        ext::*,
        var::{
            consts::*,
            donkey::*,
            globals::*,
            samusd::*,
        },
        variables::*,
        util::*,
    },
    smash::{
        app::{
            BattleObject,
            BattleObjectWorld,
            BattleObjectSlow,
            BattleObjectManager,
            BattleObjectModuleAccessor,
            BossManager,
            FighterBayonettaFinalModule,
            FighterCutInManager,
            FighterParamAccessor2,
            FighterPitBFinalModule,
            GimmickEventPresenter,
            ItemManager,
            ItemParamAccessor,
            lua_bind::{
                PostureModule,
                *
            },
            FighterManager,
            StageManager,
            utility::*,
            *
        },
        hash40,
        lib::{
            L2CAgent,
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector2f,
            Vector3f,
            Vector4f
        }
    },
    smashline::*,
    smash_script::*,
    skyline::{
        c_str,
        from_c_str,
        hooks::{
            getRegionAddress,
            InlineCtx,
            Region
        },
        nn::ro::LookupSymbol
    },
    std::{
        os::raw::c_char,
        sync::Once
    }
};

pub mod ext;
pub mod hook;
pub mod offsets;
pub mod params;
pub mod reset;
pub mod singletons;
pub mod util;
pub mod var;
pub mod variables;

pub fn install() {
	hook::install();
	params::install();
	reset::install();
}