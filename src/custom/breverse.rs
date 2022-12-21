#![allow(unused_macros)]
use {
    crate::functions::BomaExt,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
    }
};

//=================================================================
//== FighterStatusModuleImpl::set_fighter_status_data
//=================================================================
#[skyline::hook(replace=FighterStatusModuleImpl::set_fighter_status_data)]
unsafe fn set_fighter_status_data_hook(boma: &mut BattleObjectModuleAccessor, arg2: bool, treaded_kind: i32, arg4: bool, arg5: bool, arg6: bool, log_mask_flag: u64, status_attr: u32, power_up_attack_bit: u32, arg10: u32) {
    let mut new_status_attr = status_attr;
    if boma.is_fighter() {
        // this handles turnaround special/b-reversible moves
        if (boma.kind() == *FIGHTER_KIND_MARIO && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT]))
        || (boma.kind() == *FIGHTER_KIND_DONKEY && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N]))
        || (boma.kind() == *FIGHTER_KIND_LINK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST]))
        || (boma.kind() == *FIGHTER_KIND_SAMUS && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW]))
        || (boma.kind() == *FIGHTER_KIND_SAMUSD && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW]))
        || (boma.kind() == *FIGHTER_KIND_KIRBY && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S]))
        || (boma.kind() == *FIGHTER_KIND_CAPTAIN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_PURIN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_PEACH && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_DAISY && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_KOOPA && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A]))
        || (boma.kind() == *FIGHTER_KIND_ICE_CLIMBER && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_POPO && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_NANA && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_ZELDA && boma.is_status_one_of(&[*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_YOUNGLINK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_GANON && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_MEWTWO && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_METAKNIGHT && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N]))
        || (boma.kind() == *FIGHTER_KIND_IKE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S]))
        || (boma.kind() == *FIGHTER_KIND_SNAKE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_HANG]))
        || (boma.kind() == *FIGHTER_KIND_PZENIGAME && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N]))
        || (boma.kind() == *FIGHTER_KIND_PFUSHIGISOU && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N]))
        || (boma.kind() == *FIGHTER_KIND_DEDEDE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_PIKMIN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_TOONLINK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_MURABITO && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT_FAIL, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_F, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_B, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_F, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_B, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_AIR]))
        || (boma.kind() == *FIGHTER_KIND_WIIFIT && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_LITTLEMAC && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_START]))
        || (boma.kind() == *FIGHTER_KIND_GEKKOUGA && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI]))
        || (boma.kind() == *FIGHTER_KIND_PALUTENA && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_PACMAN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_SHULK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N]))
        || (boma.kind() == *FIGHTER_KIND_CLOUD && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_KAMUI && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK]))
        || (boma.kind() == *FIGHTER_KIND_BAYONETTA && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S]))
        || (boma.kind() == *FIGHTER_KIND_INKLING && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S]))
        || (boma.kind() == *FIGHTER_KIND_SIMON && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_RICHTER && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_KROOL && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_SHIZUE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_PACKUN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_BRAVE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, *FIGHTER_STATUS_KIND_SPECIAL_HI]))
        || (boma.kind() == *FIGHTER_KIND_MASTER && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_PICKEL && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S]))
        || (boma.kind() == *FIGHTER_KIND_EFLAME && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_ELIGHT && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_TRAIL && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_MIIFIGHTER && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N]))
        || (boma.kind() == *FIGHTER_KIND_MIISWORDSMAN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_MIIGUNNER && boma.is_status_one_of(&[*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_GROUND, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR])) {
            // if b-reverse flag does not already exist in status_attr bitmask
            if status_attr & *FIGHTER_STATUS_ATTR_START_TURN as u32 == 0 {
                // add b-reverse flag to status_attr bitmask
                new_status_attr = status_attr + *FIGHTER_STATUS_ATTR_START_TURN as u32;
            }
        }

    }
    original!()(boma, arg2, treaded_kind, arg4, arg5, arg6, log_mask_flag, new_status_attr, power_up_attack_bit, arg10)
}

pub fn install() {
    skyline::install_hooks!(
        set_fighter_status_data_hook,
    );
}