//Disables Training Mode Reset from resetting music (Credit to HDR)
#[skyline::from_offset(0x23ed810)]
unsafe extern "C" fn music_function1(arg: u64);

#[skyline::from_offset(0x23ee0c0)]
unsafe extern "C" fn music_function2(arg: u64, arg2: u64);

#[skyline::hook(offset = 0x1509fd4, inline)]
unsafe extern "C" fn training_reset_music1(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function1(*ctx.registers[0].x.as_ref());
    }
}

#[skyline::hook(offset = 0x14f99cc, inline)]
unsafe extern "C" fn training_reset_music2(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function2(*ctx.registers[0].x.as_ref(), *ctx.registers[1].x.as_ref());
    }
}

pub fn install() {
    //Overwrites the vanilla functions regarding resetting the training mode music to use the custom functions above
    let _ = skyline::patching::Patch::in_text(0x14f99cc).nop().unwrap();
    let _ = skyline::patching::Patch::in_text(0x1509fd4).nop().unwrap();
	skyline::install_hooks!(
        training_reset_music1,
        training_reset_music2
    );
}