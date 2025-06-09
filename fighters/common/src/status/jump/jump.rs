/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Pre Jump Sub Param, handles momentum transfer
#[skyline::hook(replace = L2CFighterCommon_status_pre_Jump_sub_param)]
unsafe extern "C" fn status_pre_jump_sub_param(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue, param_5: L2CValue) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    call_original!(fighter, param_1, param_2, param_3, param_4, param_5)
}

//The following set of hooks handles adjusting fullhop height speed, making jumping account for the C Stick, and properly applying Momentum Transfer.

//The jump_y*gravity*2.0 formula controls your characters total jump height, with lower values decreasing your total height
//The 0.5*gravity formula controls how quickly you accelerate to the top of your jump, with higher values increasing how quickly you ascend
#[skyline::hook(offset = 0x6d2194, inline)]
unsafe extern "C" fn fullhop_initial_y_speed_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let jump_y = callable(work_module, hash40("jump_y"), 0);
    let gravity = callable(work_module, hash40("air_accel_y"), 0);
    let initital_jump_vel = (jump_y*gravity*2.0).sqrt()+(0.5*gravity);
    asm!("fmov s0, w8", in("w8") initital_jump_vel)
}

#[skyline::hook(offset = 0x6ce6d8, inline)]
unsafe extern "C" fn jump1_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d19c4, inline)]
unsafe extern "C" fn jump2_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d1b10, inline)]
unsafe extern "C" fn jump3_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d0454, inline)]
unsafe extern "C" fn jump4_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6ce7d0, inline)]
unsafe extern "C" fn jump_aerial_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d05cc, inline)]
unsafe extern "C" fn jump_aerial_2_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d117c, inline)]
unsafe extern "C" fn jump_aerial_3_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6ce28c, inline)]
unsafe extern "C" fn jump_aerial_4_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6ce70c, inline)]
unsafe extern "C" fn jump1_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, hash40("run_speed_max"), 0);
    let ratio = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    let jump_speed_x_max = run_speed_max*ratio;
    asm!("fmov s0, w8", in("w8") jump_speed_x_max)
}

#[skyline::hook(offset = 0x6d19f8, inline)]
unsafe extern "C" fn jump2_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, hash40("run_speed_max"), 0);
    let ratio = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    let jump_speed_x_max = run_speed_max*ratio;
    asm!("fmov s0, w8", in("w8") jump_speed_x_max)
}

#[skyline::hook(offset = 0x6d1b44, inline)]
unsafe extern "C" fn jump3_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, hash40("run_speed_max"), 0);
    let ratio = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    let jump_speed_x_max = run_speed_max*ratio;
    asm!("fmov s0, w8", in("w8") jump_speed_x_max)
}

#[skyline::hook(offset = 0x6d04e4, inline)]
unsafe extern "C" fn jump4_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, hash40("run_speed_max"), 0);
    let ratio = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    let jump_speed_x_max = run_speed_max*ratio;
    asm!("fmov s0, w8", in("w8") jump_speed_x_max)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(status_pre_jump_sub_param);
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x6d2194).nop(); //Stubs vanilla fullhop initial y velocity calculations in favor of the custom function
    let _ = skyline::patching::Patch::in_text(0x6d21a8).data(0x52800015u32); //Changes the calculation of fullhop vertical velocity to that of shorthops
    // Stubs ControlModule::get_stick_x calls when calculating horizontal jump velocity
    let _ = skyline::patching::Patch::in_text(0x6ce6d8).nop();
    let _ = skyline::patching::Patch::in_text(0x6d19c4).nop();
    let _ = skyline::patching::Patch::in_text(0x6d1b10).nop();
    let _ = skyline::patching::Patch::in_text(0x6d0454).nop();
    // Stubs ControlModule::get_stick_x calls when calculating double jump velocity
    let _ = skyline::patching::Patch::in_text(0x6ce7d0).nop();
    let _ = skyline::patching::Patch::in_text(0x6d05cc).nop();
    let _ = skyline::patching::Patch::in_text(0x6d117c).nop();
    let _ = skyline::patching::Patch::in_text(0x6ce28c).nop();
    // Stubs vanilla initial horizontal jump speed calculations
    let _ = skyline::patching::Patch::in_text(0x6ce70c).nop();
    let _ = skyline::patching::Patch::in_text(0x6d19f8).nop();
    let _ = skyline::patching::Patch::in_text(0x6d1b44).nop();
    let _ = skyline::patching::Patch::in_text(0x6d04e4).nop();
    let _ = skyline::nro::add_hook(nro_hook);
    skyline::install_hooks!(
        fullhop_initial_y_speed_hook,
        jump1_stick_x_hook,
        jump2_stick_x_hook,
        jump3_stick_x_hook,
        jump4_stick_x_hook,
        jump_aerial_stick_x_hook,
        jump_aerial_2_stick_x_hook,
        jump_aerial_3_stick_x_hook,
        jump_aerial_4_stick_x_hook,
        jump1_jump_speed_x_max_hook,
        jump2_jump_speed_x_max_hook,
        jump3_jump_speed_x_max_hook,
        jump4_jump_speed_x_max_hook
    );
}