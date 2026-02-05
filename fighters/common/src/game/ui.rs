use super::*;

#[skyline::hook(offset = 0x1b6cc08, inline)]
unsafe fn get_set_info_alpha(ctx: &skyline::hooks::InlineCtx) {
    let layout_udata = ctx.registers[0].x();
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
    manager.robot_meter[index] = RobotMeter::new(layout_udata);
    manager.ice_climber_meter[index] = IceClimberMeter::new(layout_udata);
    manager.mariod_meter[index] = MarioDMeter::new(layout_udata);
    manager.cloud_meter[index] = CloudMeter::new(layout_udata);
    manager.link_stamina[index] = LinkStamina::new(layout_udata);
    manager.sonic_meter[index] = SonicMeter::new(layout_udata);
}

#[skyline::hook(offset = 0x138a710, inline)]
fn hud_update(_: &skyline::hooks::InlineCtx) {
    unsafe {
        let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53040f0) as *const u64;
        if [0x6020000, 0x4050000].contains(&*mode) {
            return;
        }
    }
    let mut mgr = UI_MANAGER.write();
    for robot_meter in mgr.robot_meter.iter_mut() {
        if robot_meter.is_valid() && robot_meter.is_enabled() {
            robot_meter.update();
        }
    }
    for ice_climber_meter in mgr.ice_climber_meter.iter_mut() {
        if ice_climber_meter.is_valid_4() {
            if ice_climber_meter.is_valid_1() && ice_climber_meter.is_enabled_1() {
                ice_climber_meter.update_1();
            }
            if ice_climber_meter.is_valid_2() && ice_climber_meter.is_enabled_2() {
                ice_climber_meter.update_2();
            }
            if ice_climber_meter.is_valid_3() && ice_climber_meter.is_enabled_3() {
                ice_climber_meter.update_3();
            }
        }
    }
    for mariod_meter in mgr.mariod_meter.iter_mut() {
        if mariod_meter.is_valid() && mariod_meter.is_enabled() {
            mariod_meter.update();
        }
    }
    for cloud_meter in mgr.cloud_meter.iter_mut() {
        if cloud_meter.is_valid() && cloud_meter.is_enabled() {
            cloud_meter.update();
        }
    }
    for link_stamina in mgr.link_stamina.iter_mut() {
        if link_stamina.is_valid() && link_stamina.is_enabled() {
            link_stamina.update();
        }
    }
    for sonic_meter in mgr.sonic_meter.iter_mut() {
        if sonic_meter.is_valid() && sonic_meter.is_enabled() {
            sonic_meter.update();
        }
    }
}

#[skyline::hook(offset = 0x4b638, inline)]
pub unsafe fn handle_draw_ui(ctx: &mut skyline::hooks::InlineCtx) {
    let layout = ctx.registers[0].x() as *mut Layout;
    ctx.registers[0].set_x(handle_layout_ui(layout) as u64);    
}

unsafe fn handle_layout_ui(layout: *mut Layout) -> *mut Layout {
    let layout_name = skyline::from_c_str((*layout).layout_name);
    let root_pane = &mut *(*layout).root_pane;
    if layout_name == "info_final" {
        if let Some(parent) = root_pane.find_pane_by_name_recursive("cloud_window") {
            if let Some(child) = parent.find_pane_by_name_recursive("txt_cloud") {
                let pane_text = match UiManager::get_limit_type() {
                    1 => "Cross-Slash",
                    2 => "Finishing Touch",
                    3 => "Climhazzard",
                    _ => "Omnislash"
                };
                child.as_textbox().set_text_string(&pane_text);
            }
        }
        if let Some(parent) = root_pane.find_pane_by_name_recursive("cloud_window_01") {
            if let Some(child) = parent.find_pane_by_name_recursive("txt_cloud_00") {
                let pane_text = match UiManager::get_limit_type() {
                    1 => "Cross-Slash",
                    2 => "Finishing Touch",
                    3 => "Climhazzard",
                    _ => "Omnislash Ver. 5"
                };
                child.as_textbox().set_text_string(&pane_text);
            }
        }
    }
    return layout;
}

pub fn install() {
    skyline::install_hooks!(
        get_set_info_alpha, 
        hud_update,
        handle_draw_ui
    );
}