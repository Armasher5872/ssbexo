pub fn set_pane_visible(pane: u64, visible: bool) {
    unsafe {
        let internal = *(pane as *const u64);
        *(internal as *mut u8).add(0x58) &= 0xFE;
        *(internal as *mut u8).add(0x58) |= visible as u8;
    }
}

pub fn set_pane_colors(pane: u64, white: [f32; 4], black: [f32; 4]) {
    set_vertex_colors(pane, black, black, white, white);
}

pub fn set_vertex_colors(pane: u64, tl: [f32; 4], tr: [f32; 4], bl: [f32; 4], br: [f32; 4]) {
    unsafe {
        let internal = *(pane as *const u64);
        let colors = [tl, tr, bl, br];
        for (index, color) in colors.iter().enumerate() {
            *(internal as *mut u8).add(0xe0+index*4) = (color[0]*255.0) as u8;
            *(internal as *mut u8).add(0xe1+index*4) = (color[1]*255.0) as u8;
            *(internal as *mut u8).add(0xe2+index*4) = (color[2]*255.0) as u8;
            *(internal as *mut u8).add(0xe3+index*4) = (color[3]*255.0) as u8;
        }
    }
}

pub unsafe fn get_pane_by_name(layout_view: u64, name: &str) -> [u64; 4] {
    let func: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3775F80));
    func(layout_view, name.as_ptr())
}

pub fn set_tex_coords(pane: u64, coords: [f32; 8]) {
    unsafe {
        let internal = *(pane as *const u64);
        let coordinates = std::slice::from_raw_parts_mut(*((internal + 0xf8) as *mut *mut f32), 8);
        coordinates[0] = coords[0];
        coordinates[1] = coords[1];
        coordinates[2] = coords[2];
        coordinates[3] = coords[3];
        coordinates[4] = coords[4];
        coordinates[5] = coords[5];
        coordinates[6] = coords[6];
        coordinates[7] = coords[7];
    }
}

pub fn is_pane_valid(pane: u64) -> bool {
    unsafe { 
        pane != 0 && *(pane as *const u64) != 0 
    }
}

pub fn set_width_height(pane: u64, width: f32, height: f32) {
    unsafe {
        let internal = *(pane as *const u64);
        *(internal as *mut f32).add(0x50/4) = width;
        *(internal as *mut f32).add(0x54/4) = height;
    }
}

pub fn get_width_height(pane: u64) -> (f32, f32) {
    unsafe {
        let internal = *(pane as *const u64);
        (
            *(internal as *mut f32).add(0x50/4),
            *(internal as *mut f32).add(0x54/4),
        )
    }
}

pub fn get_pane_from_layout(layout_data: u64, name: &str) -> Option<u64> {
    unsafe {
        let pane_udata = get_pane_by_name(layout_data, name);
        if pane_udata[1] != 0 {
            Some(pane_udata[1])
        } 
        else {
            None
        }
    }
}
