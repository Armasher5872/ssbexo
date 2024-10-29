use super::*;

//Changes the title screen version
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const skyline::libc::c_char) {
	let original_string = unsafe { from_c_str(string) };
	if original_string.contains("Ver.") {
        let version = match std::fs::read_to_string("sd:/ultimate/mods/Super Smash Bros EXO (Code Edits Only)/ui/exo_version.txt") {
            Ok(version_value) => version_value.trim().to_string(),
            Err(_) => {
                #[cfg(feature = "main_nro")]
                if !is_on_ryujinx() {
                    skyline_web::dialog_ok::DialogOk::ok("Super Smash Bros EXO Version unknown!");
                }
                String::from("UNKNOWN")
            }
        };
		let version_str = format!("{} / SSB:EXO (Beta) | Ver. {}\0", original_string, version);
		call_original!(arg, c_str(&version_str))
	} 
    else {
		call_original!(arg, string)
	}
}

//A hook regarding the generation/visiblity of articles. Used to allow entry articles to generate
#[skyline::hook(offset = 0x3a6670)]
unsafe extern "C" fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u32 {
    if [*WEAPON_KIND_DONKEY_DKBARREL, *WEAPON_KIND_LINK_PARASAIL, *WEAPON_KIND_DIDDY_DKBARREL].contains(&weapon_kind) {
        return *ARTICLE_USETYPE_FINAL as u32;
    }
    call_original!(weapon_kind, entry_id)
}

//Credit to Claude
#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe extern "C" fn const_allot_hook(unk: *const u8, constant: *const c_char, mut value: u32) {
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_LINK_STATUS_KIND_NUM") {
        value = 0x1F1;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_LUIGI_STATUS_KIND_NUM") {
        value = 0x1F3;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_MIIFIGHTER_STATUS_KIND_NUM") {
        value = 0x20D;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_MIISWORDSMAN_STATUS_KIND_NUM") {
        value = 0x203;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_SHEIK_STATUS_KIND_NUM") {
        value = 0x1F7;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_SONIC_STATUS_KIND_NUM") {
        value = 0x1F9;
    }
    original!()(unk,constant,value)
}

#[skyline::hook(offset = 0x15db0b0)]
pub unsafe extern "C" fn create_item(item_manager: *mut smash::app::ItemManager, create_item_param: *mut CreateItemParam, unk: bool, unk2: bool, unk3: bool) -> *mut BattleObject {
    if (*create_item_param).variation_kind > 7 {
        (*create_item_param).variation_kind = 0;
    }
    original!()(item_manager, create_item_param, unk, unk2, unk3)
}

#[skyline::from_offset(0x3ac560)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

#[skyline::from_offset(0x159fb20)]
pub unsafe extern "C" fn stage_hide(boma: *mut smash::app::BattleObjectModuleAccessor, set_vis: i32);

//Installation
pub fn install() {
	skyline::install_hooks!(
        const_allot_hook,
		change_version_string_hook,
        get_article_use_type_mask,
        create_item
    );
}