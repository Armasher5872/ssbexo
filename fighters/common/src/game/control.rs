#![allow(unused_variables)]
//The following section is credited to WuBoyTH, and is utilized to handle held buffer

const PRECEDE_EXTENSION: u8 = 15;

#[skyline::hook(offset = 0x6bd5b4, inline)]
unsafe fn set_hold_buffer_value(ctx: &mut skyline::hooks::InlineCtx) {
    let current_buffer = *ctx.registers[8].w.as_ref();
    let threshold = u8::MAX - PRECEDE_EXTENSION;
    let buffer = if current_buffer == 1 {u8::MAX as u32} else if current_buffer == threshold as u32 {1} else {current_buffer};
    *ctx.registers[8].w.as_mut() = buffer;
}

#[skyline::hook(offset = 0x6bd51c, inline)]
unsafe fn set_release_value_in_hitstop(ctx: &mut skyline::hooks::InlineCtx) {
    set_release_value_internal(ctx);
}

#[skyline::hook(offset = 0x6bd5d8, inline)]
unsafe fn set_release_value(ctx: &mut skyline::hooks::InlineCtx) {
    set_release_value_internal(ctx);
}

unsafe fn set_release_value_internal(ctx: &mut skyline::hooks::InlineCtx) {
    let threshold = u8::MAX - PRECEDE_EXTENSION;
    let current_buffer = ctx.registers[9].w.as_ref();
    let buffer = if *current_buffer < threshold as u32 {*current_buffer} else {1};
    *ctx.registers[8].w.as_mut() = buffer as u32;
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x6bd5b0).nop(); //Stubs the check if the buffer value is 1 and the button is held
    let _ = skyline::patching::Patch::in_text(0x6bd53c).nop(); //Stubs setting the buffer lifetime to 2 if held
    let _ = skyline::patching::Patch::in_text(0x6bd5b4).nop(); //Changes the held buffer assignment
	skyline::install_hooks!(
        set_hold_buffer_value,
        set_release_value_in_hitstop,
        set_release_value
    );
}