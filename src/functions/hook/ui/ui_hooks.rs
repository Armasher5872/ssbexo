use super::*;

#[skyline::hook(offset = 0x1b6cc08, inline)]
unsafe fn get_set_info_alpha(ctx: &skyline::hooks::InlineCtx) {
    let layout_udata = *ctx.registers[0].x.as_ref();
    let layout_view = *(layout_udata as *const u64).add(1);
    let layout_pane = *(layout_view as *const u64).add(3);
    let ui2d_pane = *(layout_pane as *const u64);
    let name_ptr = (ui2d_pane as *const u8).add(0xb0);
    let len = skyline::libc::strlen(name_ptr);
    let name = std::str::from_utf8_unchecked(std::slice::from_raw_parts(name_ptr, len));
    let index = match name {
        "p1" => 0,
        "p2" => 1,
        "p3" => 2,
        "p4" => 3,
        "p5" => 4,
        "p6" => 5,
        "p7" => 6,
        "p8" => 7,
        _ => return,
    };
    let mut manager = UI_MANAGER.write();
    manager.palutena_meter[index] = PalutenaMeter::new(layout_udata);
    manager.robot_meter[index] = RobotMeter::new(layout_udata);
}

#[skyline::hook(offset = 0x138a710, inline)]
fn hud_update(_: &skyline::hooks::InlineCtx) {
    unsafe {
        let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53050f0) as *const u64;
        if [0x6020000, 0x4050000].contains(&*mode) {
            return;
        }
    }
    let mut mgr = UI_MANAGER.write();
    for palutena_meter in mgr.palutena_meter.iter_mut() {
        if palutena_meter.is_valid() && palutena_meter.is_enabled() {
            palutena_meter.update();
        }
    }
    for robot_meter in mgr.robot_meter.iter_mut() {
        if robot_meter.is_valid() && robot_meter.is_enabled() {
            robot_meter.update();
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        get_set_info_alpha, 
        hud_update
    );
}