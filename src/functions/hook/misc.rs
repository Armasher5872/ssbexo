use super::*;

//Changes the title screen version
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const skyline::libc::c_char) {
	let original_string = unsafe {from_c_str(string)};
	if original_string.contains("Ver.") {
		let version_string = format!("{} / SSB:EXO (Beta) | Ver. 0.4.5 \0", original_string);
		call_original!(arg, c_str(&version_string));
	}
	else {
		call_original!(arg, string);
	}
}

//A hook regarding the generation/visiblity of articles. Used to allow entry articles to generate
#[skyline::hook(offset = 0x3a6670)]
unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
    if [*WEAPON_KIND_DONKEY_DKBARREL, *WEAPON_KIND_LINK_PARASAIL].contains(&weapon_kind) {
        return 1;
    }
    if [*WEAPON_KIND_BUDDY_PIECE].contains(&weapon_kind) {
        return 0;
    }
    call_original!(weapon_kind, entry_id)
}

//Credit to Claude
#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe fn const_allot_hook(unk: *const u8, constant: *const c_char, mut value: u32) {
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_TERM")
    || CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_TERM") {
        value = 0x200000E4;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_TERM") {
        value = 0x100000C4;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_TERM") {
        value = 0x52;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_GANON_INSTANCE_WORK_ID_INT_TERM")
    || CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_TERM") {
        value = 0x100000C1;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_LINK_STATUS_KIND_NUM") {
        value = 0x1F1;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_LUIGI_STATUS_KIND_NUM") {
        value = 0x1F3;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_MIISWORDSMAN_STATUS_KIND_NUM") {
        value = 0x203;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_SHEIK_STATUS_KIND_NUM") {
        value = 0x1F7;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_TERM") {
        value = 0x200000EB;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_TERM") {
        value = 0x100000CA;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_SONIC_STATUS_KIND_NUM") {
        value = 0x1F9;
    }
    original!()(unk,constant,value)
}

#[skyline::hook(offset = 0x15db0b0)]
pub unsafe fn create_item(item_manager: *mut smash::app::ItemManager, create_item_param: *mut CreateItemParam, unk: bool, unk2: bool, unk3: bool) -> *mut BattleObject {
    if (*create_item_param).variation_kind > 7 {
        (*create_item_param).variation_kind = 0;
    }
    original!()(item_manager, create_item_param, unk, unk2, unk3)
}

//Installation
pub fn install() {
    skyline::install_hook!(const_allot_hook);
	skyline::install_hooks!(
		change_version_string_hook,
        get_article_use_type_mask,
        create_item
    );
}