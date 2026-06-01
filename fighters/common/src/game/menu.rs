use super::*;

//Credited to HDR, makes the main menu load quicker
#[skyline::hook(offset = 0x235cad0, inline)]
unsafe extern "C" fn main_menu_quick(ctx: &skyline::hooks::InlineCtx) {
    let sp = (ctx as *const skyline::hooks::InlineCtx as *mut u8).add(0x100);
    *(sp.add(0x60) as *mut u64) = 0x1100000000;
    let slice = std::slice::from_raw_parts_mut(sp.add(0x68), 18);
    slice.copy_from_slice(b"MenuSequenceScene\0");
}

//Credited to HDR, retains your previously selected character when returning from a match exit to the CSS
#[skyline::hook(offset = 0x1798ac8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = ctx.registers[0].x() as *mut u64;
    let ptr2 = ctx.registers[1].x() as *mut u64;
    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

//Credited to jobrien97, skips the results screen with start button
#[skyline::hook(offset = 0x36650c0)]
unsafe fn process_inputs_handheld(controller: &mut exo_utils::structs::controller_struct::Controller) {
    let entry_count = smash::app::lua_bind::FighterManager::entry_count(singletons::FighterManager());
    if smash::app::lua_bind::FighterManager::is_result_mode(singletons::FighterManager()) && entry_count > 0 {
        if is_press(ninput::Buttons::PLUS) {
            SHOULD_END_RESULT_SCREEN = true;
        }
        if is_press(ninput::Buttons::B) {
            SHOULD_END_RESULT_SCREEN = false;
        }
        if SHOULD_END_RESULT_SCREEN {
            let mut rng = rand::thread_rng();
            //Need to space apart A-presses so it does not seem like we are holding the button.
            let n: u32 = rng.gen_range(0..3);
            if n == 1 {
                controller.current_buttons.set_a(true);
                controller.just_down.set_a(true);
            }
        }
    }
    if entry_count == 0 {
        SHOULD_END_RESULT_SCREEN = false;
    }
    call_original!(controller);
}

pub fn install() {
	skyline::install_hooks!(
        main_menu_quick,
        fix_chara_replace,
        process_inputs_handheld
    );
}