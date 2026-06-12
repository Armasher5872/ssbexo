use super::*;

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

//Installation
pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x60eb08).data(0x52800001u32); //Removes Jostle
	skyline::install_hook!(change_version_string_hook);
}