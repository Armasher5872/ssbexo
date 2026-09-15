//The following section is credited to HewDraw Remix, and is utilized to fix control stick issues
use super::*;

#[skyline::hook(offset = 0x3f8470)]
unsafe fn reset_flick_y(control_module: u64) {
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    WorkModule::set_int(boma, u8::MAX as i32-1, *FIGHTER_INSTANCE_WORK_ID_INT_LEFT_STICK_FLICK_Y);
    call_original!(control_module);
}

//Credited to WuBoyTH, handles c stick stuff
#[skyline::hook(offset = 0x6bac10)]
unsafe fn exec_command_hook(control_module: u64, flag: bool) {
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    exec_internal(boma);
    let cat1_prev = unsafe {ControlModule::get_command_flag_cat(boma, 0)};
    call_original!(control_module, flag);
    exec_post(boma, cat1_prev);
}

//Prevent game from thinking you are inputting a flick on the frame the cstick stops overriding left stick
unsafe fn exec_internal(module_accessor: *mut BattleObjectModuleAccessor) {
    if Buttons::from_bits_retain(ControlModule::get_release(module_accessor)).intersects(Buttons::CStickOverride) {
        ControlModule::reset_flick_x(module_accessor);
        ControlModule::reset_flick_y(module_accessor);
    }
}

//Cull Attack inputs if grab is used
unsafe fn exec_post(module_accessor: *mut BattleObjectModuleAccessor, cat1_prev: i32) {
    let cmd_cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && cat1_prev & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH == 0 {
        if !Buttons::from_bits_retain(ControlModule::get_trigger(module_accessor)).intersects(Buttons::CStickOn) {
            for attack in 0..7 {
                ControlModule::clear_command_one(module_accessor, 0, attack);
            }
        }
        else {
            ControlModule::clear_command_one(module_accessor, 0, 0x1D);
        }
    }
}

//This hook prevents buffered nair after inputting C-stick on first few frames of jumpsquat. It is found in ControlModule::exec_command
#[skyline::hook(offset = 0x6be630)]
unsafe fn set_attack_air_stick_hook(control_module: u64, arg: u32) {
    //This check passes on the frame FighterControlModuleImpl::reserve_on_attack_button is called. Only happens during jumpsquat currently
    if *((control_module + 0x645) as *const bool) {
        return;
    }
    call_original!(control_module, arg);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x6be664).data(0x52800040); //Prevents buffered C-stick aerials from triggering nair
    let _ = skyline::patching::Patch::in_text(0x6bd6c4).nop(); //Prevents attack_air_kind from resetting every frame. Found in ControlModule::exec_command
	skyline::install_hooks!(
        reset_flick_y,
        exec_command_hook,
        set_attack_air_stick_hook
    );
}