use super::*;

const CONSTANT_OFFSET: usize = 0x3728410;

//Changes the title screen version
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const skyline::libc::c_char) {
	let original_string = unsafe { from_c_str(string) };
	if original_string.contains("13.0.4") {
        let version = match std::fs::read_to_string("sd:/ultimate/mods/Super Smash Bros EXO/ui/exo_version.txt") {
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
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_MIIFIGHTER_STATUS_KIND_NUM") {
        value = 0x20D;
    }
    original!()(unk,constant,value)
}

//Credit to HewDraw Remix. Fixes common statuses being weird with edge cancels
#[skyline::hook(replace=GroundModule::correct)]
unsafe extern "C" fn groundmodule_correct(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    if situation_kind == *SITUATION_KIND_GROUND {
        if [*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
            return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    original!()(boma, kind)
}

//Installation
pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x60eb08).data(0x52800001u32); //Removes Jostle
	skyline::install_hooks!(
        change_version_string_hook,
        const_allot_hook//,
        //groundmodule_correct
    );
}