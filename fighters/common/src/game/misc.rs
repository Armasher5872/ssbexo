#![allow(dead_code, unexpected_cfgs)]
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

//Credit to Claude
#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe extern "C" fn const_allot_hook(unk: *const u8, constant: *const c_char, mut value: u32) {
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_NUM") {
        value = 0x7;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("WEAPON_LINK_NAVY_STATUS_KIND_NUM") {
        value = 0x9;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_MIIFIGHTER_STATUS_KIND_NUM") {
        value = 0x20D;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_MIISWORDSMAN_STATUS_KIND_NUM") {
        value = 0x203;
    }
    original!()(unk,constant,value)
}

//Gravity, used in Custom Gamemodes
#[skyline::hook(replace = smash::app::lua_bind::FighterInformation::gravity)]
unsafe fn gravity_replace(fighter_information: &mut smash::app::FighterInformation) -> f32 {
	let ret = original!()(fighter_information);
	if ret == 1.33 {
		SPECIAL_SMASH_GRAVITY = 1;
	}
	else if ret == 0.66 {
		SPECIAL_SMASH_GRAVITY = 2;
	}
	else {
		SPECIAL_SMASH_GRAVITY = 0;
	}
	return 1.0;
}

//Installation
pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x60eb08).data(0x52800001u32); //Removes Jostle
	skyline::install_hooks!(
        change_version_string_hook,
        const_allot_hook//,
        //gravity_replace
    );
}