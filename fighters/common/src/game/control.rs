//The following section is credited to WuBoyTH, and is utilized to handle held buffer
use super::*;

const PRECEDE_EXTENSION: u8 = 15;

#[skyline::hook(offset = 0x6bd5b4, inline)]
unsafe fn set_hold_buffer_value(ctx: &mut skyline::hooks::InlineCtx) {
    let current_buffer = ctx.registers[8].w();
    let threshold = u8::MAX - PRECEDE_EXTENSION;
    let buffer = if current_buffer == 1 {u8::MAX as u32} else if current_buffer == threshold as u32 {1} else {current_buffer};
    ctx.registers[8].set_w(buffer);
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
    let current_buffer = ctx.registers[9].w();
    let buffer = if current_buffer < threshold as u32 {current_buffer} else {1};
    ctx.registers[8].set_w(buffer as u32);
}

//The following section is credited to HewDraw Remix, and is utilized to fix control stick issues

//These 2 hooks prevent buffered nair after inputting C-stick on first few frames of jumpsquat. Both found in ControlModule::exec_command
#[skyline::hook(offset = 0x6be630)]
unsafe fn set_attack_air_stick_hook(control_module: u64, arg: u32) {
    //This check passes on the frame FighterControlModuleImpl::reserve_on_attack_button is called. Only happens during jumpsquat currently
    if *((control_module + 0x645) as *const bool) {
        return;
    }
    call_original!(control_module, arg);
}

/*
For some reason, the game resets your attack_air_kind value every frame even though it resets as soon as you perform an aerial attack. 
We don't want this to reset while in jumpsquat to allow the game to use your initial C-stick input during jumpsquat for your attack_air_kind
*/
#[skyline::hook(offset = 0x6bd6c4, inline)]
unsafe fn exec_command_reset_attack_air_kind_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = ctx.registers[21].x();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let status_kind = StatusModule::status_kind(boma);
    if status_kind != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        ControlModule::reset_attack_air_kind(boma);
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x6bd5b0).nop(); //Stubs the check if the buffer value is 1 and the button is held
    let _ = skyline::patching::Patch::in_text(0x6bd53c).nop(); //Stubs setting the buffer lifetime to 2 if held
    let _ = skyline::patching::Patch::in_text(0x6bd5b4).nop(); //Changes the held buffer assignment
    let _ = skyline::patching::Patch::in_text(0x6be664).data(0x52800040); //Prevents buffered C-stick aerials from triggering nair
    let _ = skyline::patching::Patch::in_text(0x6bd6c4).nop(); //Prevents attack_air_kind from resetting every frame. Found in ControlModule::exec_command
	skyline::install_hooks!(
        set_hold_buffer_value,
        set_release_value_in_hitstop,
        set_release_value,
        set_attack_air_stick_hook,
        exec_command_reset_attack_air_kind_hook
    );
}