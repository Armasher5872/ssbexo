use super::*;

#[skyline::hook(offset = 0x3d4180)]
unsafe extern "C" fn article_module_generate_article(module: u64, article_kind: i32, arg3: bool, arg4: i32) {
    let boma = *(module as *mut *mut BattleObjectModuleAccessor).add(1);
    let status_module = *(boma as *const u64).add(0x8);
    let fighter = get_fighter_common_from_accessor(&mut (*boma));
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let motion_kind = MotionModule::motion_kind(boma);
    let is_respawn_entry = StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH && ([hash40("entry_l"), hash40("entry_r")].contains(&motion_kind) || current_frame <= 1.0);
    if is_respawn_entry {
        *((status_module + 0x98) as *mut i32) = *FIGHTER_STATUS_KIND_ENTRY;
    }
    call_original!(module, article_kind, arg3, arg4);
    if is_respawn_entry {    
        *((status_module + 0x98) as *mut i32) = *FIGHTER_STATUS_KIND_REBIRTH;
    }
}

//Installation
pub fn install() {
    update_int_2(-*WEAPON_KIND_MARIO_DOKAN, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_DONKEY_DKBARREL, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_LINK_PARASAIL, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_SAMUS_TRANSPORTATION, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_KIRBY_WARPSTAR, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_FOX_ARWING, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_PIKACHU_MONSTERBALL, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_LUIGI_DOKAN, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_CAPTAIN_BLUEFALCON, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_PURIN_MONSTERBALL, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_PEACH_KASSAR, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_DAISY_KASSAR, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_MARIOD_CAPSULEBLOCK, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_PICHU_MONSTERBALL, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_FALCO_ARWING, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_LUCINA_MASK, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_ROY_SWORD, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_GAMEWATCH_ENTRY, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_METAKNIGHT_MANTLE, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_SZEROSUIT_GUNSHIP, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_WARIO_WARIOBIKE, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_WARIO_GARLIC, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_DIDDY_DKBARREL, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_LUCAS_DOSEITABLE, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_DEDEDE_SHRINE, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_PIKMIN_DOLFIN, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_MURABITO_HOUSE, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_WIIFIT_BALANCEBOARD, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_WIIFIT_WIIBO, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_LITTLEMAC_SWEATLITTLEMAC, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_LITTLEMAC_THROWSWEAT, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_GEKKOUGA_MONSTERBALL, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_PALUTENA_GATE, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_PACMAN_BIGPACMAN, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_SHIZUE_OFFICE, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_GAOGAEN_MONSTERBALL, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_MASTER_BATON, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
    update_int_2(-*WEAPON_KIND_PICKEL_ENTRYOBJECT, vec![-1].clone(), (hash40("article_use_type"), 0, *ARTICLE_USETYPE_FINAL));
	skyline::install_hook!(article_module_generate_article);
}