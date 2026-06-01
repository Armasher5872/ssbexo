use super::*;

const CONSTANT_OFFSET: usize = 0x3728410;

//Changes the title screen version
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const skyline::libc::c_char) {
	let original_string = unsafe { from_c_str(string) };
	if original_string.contains("13.0.4") && !original_string.contains("Omnislash") {
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

//Credit to HDR: This is a hook on the main scene transition function. key_str is the scene name. Add anything requiring a scene transition check here
#[skyline::hook(offset = 0x3726120)]
unsafe fn scene_transition(list_ptr: *mut c_void, key_struct: *const HashedString, context_struct: *const HashedString, factory: *mut c_void) {
    MATCH_EXITING.store(false, Ordering::Relaxed);
    call_original!(list_ptr, key_struct, context_struct, factory);
}

//Installation
pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x60eb08).data(0x52800001u32); //Removes Jostle
	skyline::install_hooks!(
        change_version_string_hook,
        const_allot_hook/*,
        scene_transition
        */
    );
}