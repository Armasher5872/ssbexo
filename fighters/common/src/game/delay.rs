//Credit to BluJay, hacky method of rerouting the vsync to effectively make the game double buffered as opposed to triple buffered, and as such is prone to issues. Also doesn't work on emulators
#![allow(static_mut_refs)] //Addresses warning: creating a shared reference to mutable static is discouraged
use super::*;

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

//Function used to calculate the base of nnSdk (Nintendo Software Development Kit) for SSBU. If nnSdk for SSBU gets updated this method needs to be updated
unsafe fn calc_nnsdk_offset() -> u64 {
    let mut symbol = 0usize;
    skyline::nn::ro::LookupSymbol(&mut symbol, b"_ZN7android7IBinderD1Ev\0".as_ptr());
    (symbol - 0x240) as u64
}

//Patches either nvnWindowBuilderSetPresentInterval or nvnWindowSetPresentInterval to set the passed value to 0, disabling vsync
#[skyline::hook(replace = OFFSET1)]
unsafe fn set_present_interval_nvn(window: u64, _: i32) {
    call_original!(window, 0);
}

//Patches an internal presentation/surface device implementation for setting the present interval
#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn set_present_interval_android(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers[8].set_x(0);
}

pub static RUN: AtomicBool = AtomicBool::new(false);

//Hooks into the VSyncUpdate method for SSBU (a physical frame counter). This is used to manually synchronize on a vsync to be 1 sync behind what the game "expects", effectively slicing off 1 frame from input delay
#[skyline::hook(offset = 0x3810a64, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

#[skyline::hook(offset = 0x3747b7c, inline)]
unsafe fn run_scene_update(_: &skyline::hooks::InlineCtx) {
    while !RUN.swap(false, Ordering::SeqCst) {
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;

pub fn install() {
    if !is_on_ryujinx() {
        unsafe {
            OFFSET1 = calc_nnsdk_offset()+0x429d60;
            OFFSET2 = calc_nnsdk_offset()+0x26e94;
        }
        skyline::install_hooks!(
            set_present_interval_nvn,
            set_present_interval_android,
            vsync_count_thread,
            run_scene_update
        );
    }
}