use {
    bitflags::bitflags,
    crate::functions::{
        ext::*,
        var::{
            armstrong::*,
            consts::*,
            donkey::*,
            globals::*,
            kirby::*,
            luigi::*,
            metaknight::*,
            miiswordsman::*,
            pfushigisou::*,
            pikachu::*,
            samusd::*,
            sonic::*,
            variables::*,
        },
        util::*,
    },
    modular_bitfield::{
        bitfield,
        specifiers::B4
    },
    smash::{
        app::{
            ArticleOperationTarget,
            BattleObject,
            BattleObjectWorld,
            BattleObjectSlow,
            BattleObjectManager,
            BattleObjectModuleAccessor,
            BossManager,
            Fighter,
            FighterBayonettaFinalModule,
            FighterCutInManager,
            FighterManager,
            FighterParamAccessor2,
            FighterPitBFinalModule,
            FighterUtil,
            GimmickEventPresenter,
            GroundCorrectKind,
            ItemManager,
            ItemParamAccessor,
            lua_bind::{
                PostureModule,
                *
            },
            StageManager,
            sv_animcmd,
            sv_battle_object,
            utility::*,
        },
        hash40,
        lib::{
            L2CAgent,
            L2CValue,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        phx::{
            Hash40,
            Vector3f,
            Vector4f
        }
    },
    smashline::*,
    smash_script::*,
    skyline::{
        c_str,
        from_c_str,
        hooks::InlineCtx,
        nn::ro::LookupSymbol
    },
    std::{
        ffi::CStr,
        os::raw::{
            c_char,
            c_int
        },
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

pub fn install() {
	hook::install();
	params::install();
	reset::install();
}