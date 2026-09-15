#![allow(static_mut_refs)] //Addresses creating a mutable reference to mutable static
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

//Credit to HDR, reduces rim lighting on fighters to 0.5 strength
#[skyline::hook(offset = 0x3802ad0)]
unsafe fn set_uniform_buffer(stage: u64, index: u64, buffer: u64) {
    let cbuf = *((buffer + 8) as *const u64);
    let buffer_ptr = *((cbuf + 0x98) as *const u64);
    if buffer_ptr == 0 {
        return call_original!(stage, index, buffer);
    }
    let size = *((cbuf + 0x10) as *const usize);
    let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8;
    let func = *(text.add(0x593fd28) as *const u64);
    let func: extern "C" fn(*mut u8) -> *mut u8 = std::mem::transmute(func);
    let map = func(buffer_ptr as _);
    let data = std::slice::from_raw_parts_mut(map, size);
    skyline::nn::os::LockMutex(DATA_ACCESS_LOCK.as_mut_ptr().cast());
    if index == 8 {
        data[0xAC as usize..0xB0 as usize].copy_from_slice(&vec![0x00, 0x00, 0x00, 0x3F]);  // 0x00, 0x00, 0x00, 0x3F = 0.5 floating point
    }
    let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8;
    let func = *(text.add(0x593fd38) as *const u64);
    let func: extern "C" fn(*mut u8, isize, usize) = std::mem::transmute(func);
    func(buffer_ptr as _, 0, size);
    skyline::nn::os::UnlockMutex(DATA_ACCESS_LOCK.as_mut_ptr().cast());
    call_original!(stage, index, buffer);
}

//Installation
pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x60eb08).data(0x52800001u32); //Removes Jostle
	skyline::install_hooks!(
        change_version_string_hook,
        set_uniform_buffer
    );
}