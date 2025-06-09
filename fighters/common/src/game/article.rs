use super::*;

//A hook regarding the generation/visiblity of articles. Used to allow entry articles to generate
#[skyline::hook(offset = 0x3a6670)]
unsafe extern "C" fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u32 {
    if [
        *WEAPON_KIND_MARIO_DOKAN, *WEAPON_KIND_DONKEY_DKBARREL, *WEAPON_KIND_LINK_PARASAIL, *WEAPON_KIND_SAMUS_TRANSPORTATION, *WEAPON_KIND_KIRBY_WARPSTAR, *WEAPON_KIND_FOX_ARWING, *WEAPON_KIND_PIKACHU_MONSTERBALL, *WEAPON_KIND_LUIGI_DOKAN, 
        *WEAPON_KIND_CAPTAIN_BLUEFALCON, *WEAPON_KIND_PURIN_MONSTERBALL, *WEAPON_KIND_PEACH_KASSAR, *WEAPON_KIND_DAISY_KASSAR, *WEAPON_KIND_MARIOD_CAPSULEBLOCK, *WEAPON_KIND_PICHU_MONSTERBALL, *WEAPON_KIND_FALCO_ARWING, *WEAPON_KIND_LUCINA_MASK, 
        *WEAPON_KIND_ROY_SWORD, *WEAPON_KIND_GAMEWATCH_ENTRY, *WEAPON_KIND_METAKNIGHT_MANTLE, *WEAPON_KIND_SZEROSUIT_GUNSHIP, *WEAPON_KIND_WARIO_WARIOBIKE, *WEAPON_KIND_DIDDY_DKBARREL, *WEAPON_KIND_LUCAS_DOSEITABLE, *WEAPON_KIND_DEDEDE_SHRINE,
        *WEAPON_KIND_PIKMIN_DOLFIN, *WEAPON_KIND_MURABITO_HOUSE, *WEAPON_KIND_WIIFIT_BALANCEBOARD, *WEAPON_KIND_WIIFIT_WIIBO, *WEAPON_KIND_LITTLEMAC_SWEATLITTLEMAC, *WEAPON_KIND_LITTLEMAC_THROWSWEAT, *WEAPON_KIND_GEKKOUGA_MONSTERBALL, 
        *WEAPON_KIND_PALUTENA_GATE, *WEAPON_KIND_PACMAN_BIGPACMAN, *WEAPON_KIND_SHIZUE_OFFICE, *WEAPON_KIND_GAOGAEN_MONSTERBALL, *WEAPON_KIND_MASTER_BATON, *WEAPON_KIND_PICKEL_ENTRYOBJECT
    ].contains(&weapon_kind) {
        return *ARTICLE_USETYPE_FINAL as u32;
    }
    call_original!(weapon_kind, entry_id)
}

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
    let _ = skyline::patching::Patch::in_text(0x60eb08).data(0x52800001u32); //Removes Jostle
	skyline::install_hooks!(
        get_article_use_type_mask,
        article_module_generate_article
    );
}