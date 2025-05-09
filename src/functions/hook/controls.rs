#![allow(unused_variables)]
use super::*;

macro_rules! apply_button_mappings {
    ($controller:ident, $mappings:ident, $(($button:ident, $mapped:ident, $kind:ident, $output:expr))*) => {{
        let mut buttons = Buttons::empty();
        $(
                if $controller.current_buttons.$button() && (*$mappings).$mapped == InputKind::$kind {
                    buttons |= $output;
                }
        )*
        buttons
    }}
}

#[repr(C)]
struct SomeControllerStruct {
    padding: [u8; 0x10],
    controller: &'static mut Controller
}

#[repr(C)]
struct ControlModuleInternal {
    vtable: *mut u8,
    controller_index: i32,
    buttons: Buttons,
    stick_x: f32,
    stick_y: f32,
    padding: [f32; 2],
    unk: [u32; 8],
    clamped_lstick_x: f32,
    clamped_lstick_y: f32,
    padding2: [f32; 2],
    clamped_rstick_x: f32,
    clamped_rstick_y: f32,
}

#[skyline::hook(offset = 0x1d3a000)]
unsafe fn get_button_label_by_operation_kind(hashed_string: &mut HashedString, operation: u8, arg: bool) {
    if operation == InputKind::JumpMini as u8 {
        for (index, byte) in "mnu_opt_btn_key_short_hop\0".as_bytes().iter().enumerate() {
            hashed_string.contents[index] = *byte;
        }
        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_short_hop");
    } else if operation == InputKind::FullHop as u8 {
        for (index, byte) in "mnu_opt_btn_key_tilt_attack\0".as_bytes().iter().enumerate() {
            hashed_string.contents[index] = *byte;
        }
        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_tilt_attack");
    } else {
        return call_original!(hashed_string, operation, arg)
    }
}

#[skyline::hook(offset = 0x1d334e8, inline)]
unsafe fn add_footstool_to_gc(ctx: &skyline::hooks::InlineCtx) {
    let button = *ctx.registers[25].w.as_ref();
    if ![0x3, 0x4, 0x5, 0x8].contains(&button) {
        let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);

        if input_list_vector.len() < 8 {
            input_list_vector.push(InputKind::AppealHi as u8);
            input_list_vector.push(InputKind::JumpMini as u8);
            input_list_vector.push(InputKind::SmashAttack as u8);
            input_list_vector.push(InputKind::FullHop as u8);
        }
    }
}

#[skyline::hook(offset = 0x1D331f8, inline)]
unsafe fn add_footstool_to_fk(ctx: &skyline::hooks::InlineCtx) {
    let button = *ctx.registers[25].w.as_ref();
    if [0x4, 0x5, 0x6, 0x9].contains(&button) {
        return;
    }
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);

    if input_list_vector.len() < 8 {
        input_list_vector.push(InputKind::AppealHi as u8);
        input_list_vector.push(InputKind::JumpMini as u8);
        input_list_vector.push(InputKind::SmashAttack as u8);
        input_list_vector.push(InputKind::FullHop as u8);
    }
}

#[skyline::hook(offset = 0x1D33Cd8, inline)]
unsafe fn add_footstool_to_jc(ctx: &skyline::hooks::InlineCtx) {
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);

    if input_list_vector.len() < 8 {
        input_list_vector.push(InputKind::AppealHi as u8);
        input_list_vector.push(InputKind::JumpMini as u8);
        input_list_vector.push(InputKind::SmashAttack as u8);
        input_list_vector.push(InputKind::FullHop as u8);
    }
}

#[skyline::hook(offset = 0x1D3594C, inline)]
unsafe fn add_more_buttons(ctx: &mut skyline::hooks::InlineCtx) {
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);
    // panic!("{}", input_list_vector.len());
    *ctx.registers[25].x.as_mut() = input_list_vector.len() as u64;
}

#[skyline::hook(offset = 0x16d85dc, inline)]
unsafe fn packed_packet_creation(ctx: &mut skyline::hooks::InlineCtx) {
    *ctx.registers[22].x.as_mut() = 0x2;
}

#[skyline::hook(offset = 0x16d8610, inline)]
unsafe fn write_packet(ctx: &mut skyline::hooks::InlineCtx) {
    let raw = *ctx.registers[19].x.as_ref();

    let mapped_inputs = *((raw + 0x49508) as *const MappedInputs);
    let mut packet = 0u64;

    *(&mut packet as *mut u64 as *mut i8) = mapped_inputs.lstick_x;
    *(&mut packet as *mut u64 as *mut i8).add(1) = mapped_inputs.lstick_y;

    let buttons = (mapped_inputs.buttons.bits() as u64) << 16;
    packet |= buttons;

    *(&mut packet as *mut u64 as *mut i8).add(6) = mapped_inputs.rstick_x;
    *(&mut packet as *mut u64 as *mut i8).add(7) = mapped_inputs.rstick_y;

    *ctx.registers[8].x.as_mut() = packet;
}

#[skyline::hook(offset = 0x1750f70)]
unsafe fn map_controls_hook(
    mappings: *mut ControllerMapping,
    player_idx: i32,
    out: *mut MappedInputs,
    controller_struct: &mut SomeControllerStruct,
    arg: bool
) {
    let entry_count = (*mappings.add(player_idx as usize))._34[0];
    let ret = original!()(mappings, player_idx, out, controller_struct, arg);
    let controller = &mut controller_struct.controller;

    //println!("entry_count vs. current: {} vs. {}", entry_count, (*mappings.add(player_idx as usize))._34[0]);

    if (*out).buttons.contains(Buttons::CStickOn) && (*mappings.add(player_idx as usize))._34[0] != entry_count {
        (*out).rstick_x = (controller.left_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.left_stick_y * (i8::MAX as f32)) as i8;
        (*out).buttons |= Buttons::CStickOverride;
    } else {
        (*out).rstick_x = (controller.right_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.right_stick_y * (i8::MAX as f32)) as i8;
    }

    let mappings = mappings.add(player_idx as usize);

    if controller.style == ControllerStyle::GCController {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, gc_r, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, gc_z, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, gc_z, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (a, gc_a, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (b, gc_b, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (x, gc_x, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (y, gc_y, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l, SmashAttack, Buttons::AttackAll)
                (r, gc_r, SmashAttack, Buttons::AttackAll)
                (zl, gc_z, SmashAttack, Buttons::AttackAll)
                (zr, gc_z, SmashAttack, Buttons::AttackAll)
                (a, gc_a, SmashAttack, Buttons::AttackAll)
                (b, gc_b, SmashAttack, Buttons::AttackAll)
                (x, gc_x, SmashAttack, Buttons::AttackAll)
                (y, gc_y, SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l,   AppealHi, Buttons::AppealHi)
                (r, gc_r,   AppealHi, Buttons::AppealHi)
                (zl, gc_z,  AppealHi, Buttons::AppealHi)
                (zr, gc_z,  AppealHi, Buttons::AppealHi)
                (a, gc_a,   AppealHi, Buttons::AppealHi)
                (b, gc_b,   AppealHi, Buttons::AppealHi)
                (x, gc_x,   AppealHi, Buttons::AppealHi)
                (y, gc_y,   AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l,   FullHop, Buttons::FullHop | Buttons::Jump)
                (r, gc_r,   FullHop, Buttons::FullHop | Buttons::Jump)
                (zl, gc_z,  FullHop, Buttons::FullHop | Buttons::Jump)
                (zr, gc_z,  FullHop, Buttons::FullHop | Buttons::Jump)
                (a, gc_a,   FullHop, Buttons::FullHop | Buttons::Jump)
                (b, gc_b,   FullHop, Buttons::FullHop | Buttons::Jump)
                (x, gc_x,   FullHop, Buttons::FullHop | Buttons::Jump)
                (y, gc_y,   FullHop, Buttons::FullHop | Buttons::Jump)
        );
        /*
        if (*mappings.add(player_idx as usize)).gc_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
            }
        }
        */
    } else if controller.style == ControllerStyle::LeftJoycon || controller.style == ControllerStyle::RightJoycon {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, joy_shoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, joy_zshoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, joy_zshoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (left_sl, joy_sl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (left_sr, joy_sr, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (right_sl, joy_sl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (right_sr, joy_sr, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   SmashAttack, Buttons::AttackAll)
                (r, joy_shoulder,   SmashAttack, Buttons::AttackAll)
                (zl, joy_zshoulder, SmashAttack, Buttons::AttackAll)
                (zr, joy_zshoulder, SmashAttack, Buttons::AttackAll)
                (left_sl, joy_sl,   SmashAttack, Buttons::AttackAll)
                (left_sr, joy_sr,   SmashAttack, Buttons::AttackAll)
                (right_sl, joy_sl,  SmashAttack, Buttons::AttackAll)
                (right_sr, joy_sr,  SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   AppealHi, Buttons::AppealHi)
                (r, joy_shoulder,   AppealHi, Buttons::AppealHi)
                (zl, joy_zshoulder, AppealHi, Buttons::AppealHi)
                (zr, joy_zshoulder, AppealHi, Buttons::AppealHi)
                (left_sl, joy_sl,   AppealHi, Buttons::AppealHi)
                (left_sr, joy_sr,   AppealHi, Buttons::AppealHi)
                (right_sl, joy_sl,  AppealHi, Buttons::AppealHi)
                (right_sr, joy_sr,  AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   FullHop, Buttons::FullHop | Buttons::Jump)
                (r, joy_shoulder,   FullHop, Buttons::FullHop | Buttons::Jump)
                (zl, joy_zshoulder, FullHop, Buttons::FullHop | Buttons::Jump)
                (zr, joy_zshoulder, FullHop, Buttons::FullHop | Buttons::Jump)
                (left_sl, joy_sl,   FullHop, Buttons::FullHop | Buttons::Jump)
                (left_sr, joy_sr,   FullHop, Buttons::FullHop | Buttons::Jump)
                (right_sl, joy_sl,  FullHop, Buttons::FullHop | Buttons::Jump)
                (right_sr, joy_sr,  FullHop, Buttons::FullHop | Buttons::Jump)
        );

        if controller.style == ControllerStyle::LeftJoycon {
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_right, joy_up, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_up, joy_left, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_down, joy_right, JumpMini, Buttons::JumpMini | Buttons::Jump)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down, SmashAttack, Buttons::AttackAll)
                    (dpad_right, joy_up, SmashAttack, Buttons::AttackAll)
                    (dpad_up, joy_left, SmashAttack, Buttons::AttackAll)
                    (dpad_down, joy_right, SmashAttack, Buttons::AttackAll)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down,   AppealHi, Buttons::AppealHi)
                    (dpad_right, joy_up,    AppealHi, Buttons::AppealHi)
                    (dpad_up, joy_left,     AppealHi, Buttons::AppealHi)
                    (dpad_down, joy_right,  AppealHi, Buttons::AppealHi)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down,   FullHop, Buttons::FullHop | Buttons::Jump)
                    (dpad_right, joy_up,    FullHop, Buttons::FullHop | Buttons::Jump)
                    (dpad_up, joy_left,     FullHop, Buttons::FullHop | Buttons::Jump)
                    (dpad_down, joy_right,  FullHop, Buttons::FullHop | Buttons::Jump)
            );
        } else {
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (y, joy_up, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (b, joy_left, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (x, joy_right, JumpMini, Buttons::JumpMini | Buttons::Jump)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down, SmashAttack, Buttons::AttackAll)
                    (y, joy_up, SmashAttack, Buttons::AttackAll)
                    (b, joy_left, SmashAttack, Buttons::AttackAll)
                    (x, joy_right, SmashAttack, Buttons::AttackAll)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down,   AppealHi, Buttons::AppealHi)
                    (y, joy_up,     AppealHi, Buttons::AppealHi)
                    (b, joy_left,   AppealHi, Buttons::AppealHi)
                    (x, joy_right,  AppealHi, Buttons::AppealHi)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down,   FullHop, Buttons::FullHop | Buttons::Jump)
                    (y, joy_up,     FullHop, Buttons::FullHop | Buttons::Jump)
                    (b, joy_left,   FullHop, Buttons::FullHop | Buttons::Jump)
                    (x, joy_right,  FullHop, Buttons::FullHop | Buttons::Jump)
            );
        }
        /*
        if (*mappings.add(player_idx as usize)).joy_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
            }
        }*/
    } else {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, pro_r, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, pro_zl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, pro_zr, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (a, pro_a, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (b, pro_b, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (x, pro_x, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (y, pro_y, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l, SmashAttack, Buttons::AttackAll)
                (r, pro_r, SmashAttack, Buttons::AttackAll)
                (zl, pro_zl, SmashAttack, Buttons::AttackAll)
                (zr, pro_zr, SmashAttack, Buttons::AttackAll)
                (a, pro_a, SmashAttack, Buttons::AttackAll)
                (b, pro_b, SmashAttack, Buttons::AttackAll)
                (x, pro_x, SmashAttack, Buttons::AttackAll)
                (y, pro_y, SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l,      AppealHi, Buttons::AppealHi)
                (r, pro_r,      AppealHi, Buttons::AppealHi)
                (zl, pro_zl,    AppealHi, Buttons::AppealHi)
                (zr, pro_zr,    AppealHi, Buttons::AppealHi)
                (a, pro_a,      AppealHi, Buttons::AppealHi)
                (b, pro_b,      AppealHi, Buttons::AppealHi)
                (x, pro_x,      AppealHi, Buttons::AppealHi)
                (y, pro_y,      AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l,      FullHop, Buttons::FullHop | Buttons::Jump)
                (r, pro_r,      FullHop, Buttons::FullHop | Buttons::Jump)
                (zl, pro_zl,    FullHop, Buttons::FullHop | Buttons::Jump)
                (zr, pro_zr,    FullHop, Buttons::FullHop | Buttons::Jump)
                (a, pro_a,      FullHop, Buttons::FullHop | Buttons::Jump)
                (b, pro_b,      FullHop, Buttons::FullHop | Buttons::Jump)
                (x, pro_x,      FullHop, Buttons::FullHop | Buttons::Jump)
                (y, pro_y,      FullHop, Buttons::FullHop | Buttons::Jump)
        );
        /*
        if (*mappings.add(player_idx as usize)).pro_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
            }
        }*/
    }

    // Check if the button combos are being pressed and then force Stock Share + AttackRaw/SpecialRaw depending on input

    if controller.current_buttons.l()
    && controller.current_buttons.r()
    && controller.current_buttons.a()
    && (controller.current_buttons.minus() || controller.current_buttons.plus())
    {
        controller.current_buttons.set_plus(false);
        controller.current_buttons.set_minus(false);
        controller.just_down.set_plus(false);
        controller.just_down.set_minus(false);

        if controller.current_buttons.y() {
            (*out).buttons = Buttons::StockShare | Buttons::AttackRaw;
        } else if controller.current_buttons.x() {
            (*out).buttons = Buttons::StockShare | Buttons::SpecialRaw;
        } else {
            controller.current_buttons.set_plus(true);
            controller.current_buttons.set_minus(true);
            controller.just_down.set_plus(true);
            controller.just_down.set_minus(true);
        }
    }
}

#[skyline::hook(offset = 0x3666b00, inline)]
unsafe fn analog_trigger_l(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[9].x.as_ref() & 0x40 != 0 {
        *ctx.registers[11].x.as_mut() = 0;
    } else {
        *ctx.registers[11].w.as_mut() = 0x27FF;
    }
}

#[skyline::hook(offset = 0x3666b14, inline)]
unsafe fn analog_trigger_r(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[8].x.as_ref() & 0x80 != 0 {
        *ctx.registers[11].x.as_mut() = 0;
    } else {
        *ctx.registers[11].w.as_mut() = 0x27FF;
    }
}

#[skyline::hook(offset = 0x3f7240)]
unsafe fn parse_inputs(this: &mut ControlModuleInternal) {
    const NEUTRAL: f32 = 0.2;
    const CLAMP_MAX: f32 = 120.0;
    if this.controller_index == -1 {
        return call_original!(this);
    }
    let inputs = get_mapped_controller_inputs_from_id(this.controller_index as usize);
    let clamp_mul = 1.0 / CLAMP_MAX;
    let raw_rstick_x = ((inputs.rstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
    let raw_rstick_y = ((inputs.rstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);
    LAST_ALT_STICK[0] = if raw_rstick_x.abs() >= NEUTRAL { raw_rstick_x } else { 0.0 };
    LAST_ALT_STICK[1] = if raw_rstick_y.abs() >= NEUTRAL { raw_rstick_y } else { 0.0 };
    LAST_ANALOG = ((inputs.buttons.bits() >> 22) & 1023) as f32 / 1023.0;
    call_original!(this)
}

#[skyline::hook(offset = 0x6b9c7c, inline)]
unsafe fn after_exec(ctx: &skyline::hooks::InlineCtx) {
    let module = *ctx.registers[19].x.as_ref();
    let internal_class = *(module as *const u64).add(0x110 / 0x8);
    *(internal_class as *mut f32).add(0x40 / 0x4) = LAST_ALT_STICK[0];
    *(internal_class as *mut f32).add(0x44 / 0x4) = LAST_ALT_STICK[1];
}

#[skyline::hook(offset = 0x16d7034, inline)]
unsafe fn handle_incoming_packet(ctx: &mut skyline::hooks::InlineCtx) {
    let packet = *ctx.registers[15].x.as_ref();

    let mut inputs = MappedInputs {
        buttons: Buttons::empty(),
        lstick_x: 0,
        lstick_y: 0,
        rstick_x: 0,
        rstick_y: 0
    };

    let raw_buttons = ((packet >> 16) & 0xFFFF_FFFF) as u32;
    let lstick_x = (packet & 0xFF) as i8;
    let lstick_y = ((packet & 0xFF00) >> 8) as i8;
    let rstick_x = ((packet >> 0x30) & 0xFF) as i8;
    let rstick_y = ((packet >> 0x38) & 0xFF) as i8;

    inputs.buttons = Buttons::from_bits_retain(raw_buttons as _);
    inputs.lstick_x = lstick_x;
    inputs.lstick_y = lstick_y;
    inputs.rstick_x = rstick_x;
    inputs.rstick_y = rstick_y;

    *ctx.registers[13].x.as_mut() = std::mem::transmute(inputs);
}

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

//Fix throws not respecting the cstick, especially dk cargo throw
#[skyline::hook(replace = L2CFighterCommon_IsThrowStick)]
unsafe extern "C" fn is_throw_stick(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut out = fighter.local_func__fighter_status_catch_1();
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x;
    let stick_y;
    if Buttons::from_bits_retain(ControlModule::get_button(fighter.module_accessor)).intersects(Buttons::CStickOverride) {
        stick_x = ControlModule::get_sub_stick_x(fighter.module_accessor);
        stick_y = ControlModule::get_sub_stick_y(fighter.module_accessor);
    }
    else {
        stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    }
    let stick_x = stick_x*lr;
    let throw_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_x"));
    let throw_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi4_stick_y"));
    if stick_x > throw_stick_x {
        out["f"].assign(&true.into());
    } 
    else if stick_x < -throw_stick_x {
        out["b"].assign(&true.into());
    }
    if stick_y > throw_stick_y {
        out["hi"].assign(&true.into());
    } 
    else if stick_y < -throw_stick_y {
        out["lw"].assign(&true.into());
    }
    out
}

/*
fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(is_throw_stick);
    }
}
*/

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x1D3594C).nop(); //Replaces the vanilla function in favor of the updated one
    let _ = skyline::patching::Patch::in_text(0x6bd5b0).nop(); //Stubs the check if the buffer value is 1 and the button is held
    let _ = skyline::patching::Patch::in_text(0x6bd53c).nop(); //Stubs setting the buffer lifetime to 2 if held
    let _ = skyline::patching::Patch::in_text(0x6bd5b4).nop(); //Changes the held buffer assignment
    //let _ = skyline::nro::add_hook(nro_hook);
	skyline::install_hooks!(
		get_button_label_by_operation_kind,
        add_footstool_to_gc,
        add_footstool_to_fk,
        add_footstool_to_jc,
        add_more_buttons,
		map_controls_hook,
        analog_trigger_l,
        analog_trigger_r,
        packed_packet_creation,
        write_packet,
        parse_inputs,
        handle_incoming_packet,
        after_exec,
        set_hold_buffer_value,
        set_release_value_in_hitstop,
        set_release_value
    );
}