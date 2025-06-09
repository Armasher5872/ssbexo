use super::*;

//Credited to HDR, makes the main menu load quicker
#[skyline::hook(offset = 0x235cad0, inline)]
unsafe extern "C" fn main_menu_quick(ctx: &skyline::hooks::InlineCtx) {
    let sp = (ctx as *const skyline::hooks::InlineCtx as *mut u8).add(0x100);
    *(sp.add(0x60) as *mut u64) = 0x1100000000;
    let slice = std::slice::from_raw_parts_mut(sp.add(0x68), 18);
    slice.copy_from_slice(b"MenuSequenceScene\0");
}

#[skyline::hook(offset = 0x1792c60, inline)]
unsafe extern "C" fn on_rule_select_hook(_: &skyline::hooks::InlineCtx) {
    BALL_ID = 0;
    BALL_OWNER = 9;
    SPECIAL_SMASH_BODY = 0;
    SPECIAL_SMASH_GRAVITY = 0;
    SPECIAL_SMASH_HEAD = 0;
    SPECIAL_SMASH_RATE = 0;
    SPECIAL_SMASH_SIZE = 0;
    SPECIAL_SMASH_STATUS = 0;
    TOTAL_FIGHTER = 1;
}

//Credited to HDR, retains your previously selected character when returning from a match exit to the CSS
#[skyline::hook(offset = 0x1798ac8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = *ctx.registers[0].x.as_ref() as *mut u64;
    let ptr2 = *ctx.registers[1].x.as_ref() as *mut u64;
    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

pub fn install() {
	skyline::install_hooks!(
        main_menu_quick,
        //on_rule_select_hook,
        fix_chara_replace
    );
}