use super::*;

const BACKGROUND_GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BACKGROUND_PURPLE: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
const BACKGROUND_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const FOREGROUND_CHARGE_WHITE: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; //Controls bottom color
const FOREGROUND_CHARGE_BLACK: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; //Controls top color

const FULL_TEXCOORDS: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];

const EMPTY_TEXCOORDS: [f32; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0];

fn set_pane_visible(pane: u64, visible: bool) {
    unsafe {
        let internal = *(pane as *const u64);
        *(internal as *mut u8).add(0x58) &= 0xFE;
        *(internal as *mut u8).add(0x58) |= visible as u8;
    }
}

fn set_pane_colors(pane: u64, white: [f32; 4], black: [f32; 4]) {
    set_vertex_colors(pane, black, black, white, white);
}

fn set_vertex_colors(pane: u64, tl: [f32; 4], tr: [f32; 4], bl: [f32; 4], br: [f32; 4]) {
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

unsafe fn get_pane_by_name(layout_view: u64, name: &str) -> [u64; 4] {
    let func: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3775F60));
    func(layout_view, name.as_ptr())
}

fn set_tex_coords(pane: u64, coords: [f32; 8]) {
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

fn is_pane_valid(pane: u64) -> bool {
    unsafe { 
        pane != 0 && *(pane as *const u64) != 0 
    }
}

fn set_width_height(pane: u64, width: f32, height: f32) {
    unsafe {
        let internal = *(pane as *const u64);
        *(internal as *mut f32).add(0x50/4) = width;
        *(internal as *mut f32).add(0x54/4) = height;
    }
}

fn get_width_height(pane: u64) -> (f32, f32) {
    unsafe {
        let internal = *(pane as *const u64);
        (
            *(internal as *mut f32).add(0x50/4),
            *(internal as *mut f32).add(0x54/4),
        )
    }
}

fn get_pane_from_layout(layout_data: u64, name: &str) -> Option<u64> {
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

trait UiObject {
    fn update(&mut self);
    fn is_valid(&self) -> bool;
    fn set_enable(&mut self, enable: bool);
    fn is_enabled(&self) -> bool;
    fn reset(&mut self);
    fn change_color_green(&mut self);
    fn change_color_purple(&mut self);
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct PalutenaMeter {
    pub base_bar: u64,
    pub bars: [u64; 2],
    pub bars_background: [u64; 2],
    pub number: u64,
    pub original_bar_width: f32,
    pub original_bar_height: f32,
    pub original_bar2_width: f32,
    pub original_bar2_height: f32,
    pub actual_percentage: f32,
    pub visual_percentage: f32,
    pub current_number: usize,
    pub current_max: usize,
    enabled: bool,
}

impl PalutenaMeter {
    pub fn new(layout_data: u64) -> Self {
        let base_bar = get_pane_from_layout(layout_data, "ff_meter_base\0").expect("Could not find base meter!");
        let bar1 = get_pane_from_layout(layout_data, "ff_meter_bar1\0").expect("Could not find first bar!");
        let bar2 = get_pane_from_layout(layout_data, "ff_meter_bar2\0").expect("Could not find second bar!");
        let bar1_bg = get_pane_from_layout(layout_data, "ff_meter_bar1_bg\0").expect("Could not find first bg bar!");
        let bar2_bg = get_pane_from_layout(layout_data, "ff_meter_bar2_bg\0").expect("Could not find second bg bar!");
        let number = get_pane_from_layout(layout_data, "ff_meter_num\0").expect("Could not find number!");
        Self {
            base_bar,
            bars: [bar1, bar2],
            bars_background: [bar1_bg, bar2_bg],
            number: number,
            original_bar_width: -1.0,
            original_bar_height: -1.0,
            original_bar2_width: -1.0,
            original_bar2_height: -1.0,
            actual_percentage: -1.0,
            visual_percentage: -1.0,
            current_number: 0,
            current_max: 2,
            enabled: false,
        }
    }
    pub fn reset(&mut self) {
        set_pane_visible(self.base_bar, true);
        set_pane_visible(self.bars[0], false);
        set_pane_visible(self.bars[1], false);
        set_pane_visible(self.bars_background[0], false);
        set_pane_visible(self.bars_background[1], false);
        set_pane_visible(self.number, true);
        if self.original_bar_height < 0.0 {
            self.original_bar_width = get_width_height(self.bars[0]).0;
            self.original_bar_height = get_width_height(self.bars[0]).1;
            self.original_bar2_width = get_width_height(self.bars[1]).0;
            self.original_bar2_height = get_width_height(self.bars[1]).1;
        }
        self.current_number = 0;
        self.current_max = 2;
        self.actual_percentage = 0.0;
        self.visual_percentage = 0.0;
    }
    pub fn set_meter_info(&mut self, current: f32, max: f32, per_level: f32) {
        let bar_total = per_level*2.0;
        self.current_max = (max/bar_total) as usize;
        let number = current/bar_total;
        let number = number.clamp(0.0, 5.0) as usize;
        let percent = if number == self.current_max {1.0} else {(current%bar_total)/bar_total};
        if number != self.current_number && number != 5 {
            self.actual_percentage = percent;
            self.visual_percentage = 0.0;
        } 
        else {
            self.actual_percentage = percent;
        }
        self.current_number = number;
    }
    pub fn set_tex_coords(&mut self) {
        set_pane_colors(self.bars[0], FOREGROUND_CHARGE_WHITE, FOREGROUND_CHARGE_BLACK);
        set_pane_colors(self.bars[1], FOREGROUND_CHARGE_WHITE, FOREGROUND_CHARGE_BLACK);
        if self.actual_percentage >= 0.5 {
            set_tex_coords(self.bars_background[0], FULL_TEXCOORDS);
            set_width_height(self.bars_background[0], self.original_bar_width, self.original_bar_height);
            set_pane_visible(self.bars_background[0], true);
            if self.actual_percentage >= 1.0 {
                set_tex_coords(self.bars_background[1], FULL_TEXCOORDS);
                set_width_height(self.bars_background[1], self.original_bar2_width, self.original_bar2_height);
                set_pane_visible(self.bars_background[1], true);
            } 
            else {
                let percentage = (self.actual_percentage-0.5)/0.5;
                set_pane_visible(self.bars_background[1], true);
                set_tex_coords(self.bars_background[1], [0.0, 0.0, percentage, 0.0, 0.0, 1.0, percentage, 1.0]);
                set_width_height(self.bars_background[1], self.original_bar2_width*percentage, self.original_bar2_height);
            }
        } 
        else {
            set_tex_coords(self.bars_background[1], EMPTY_TEXCOORDS);
            set_width_height(self.bars_background[1], 0.0, 0.0);
            set_pane_visible(self.bars_background[1], false);
            let percentage = self.actual_percentage/0.5;
            set_tex_coords(self.bars_background[0], [0.0, 0.0, percentage, 0.0, 0.0, 1.0, percentage, 1.0]);
            set_width_height(self.bars_background[0], self.original_bar_width*percentage, self.original_bar_height);
            set_pane_visible(self.bars_background[0], true);
        }
        if self.visual_percentage >= 0.5 {
            set_tex_coords(self.bars[0], FULL_TEXCOORDS);
            set_width_height(self.bars[0], self.original_bar_width, self.original_bar_height);
            set_pane_visible(self.bars[0], true);
            if self.visual_percentage >= 1.0 {
                set_tex_coords(self.bars[1], FULL_TEXCOORDS);
                set_width_height(self.bars[1], self.original_bar2_width, self.original_bar2_height);
                set_pane_visible(self.bars[1], true);
            } 
            else {
                let percentage = (self.visual_percentage-0.5)/0.5;
                set_pane_visible(self.bars[1], true);
                set_tex_coords(self.bars[1], [0.0, 0.0, percentage, 0.0, 0.0, 1.0, percentage, 1.0]);
                set_width_height(self.bars[1], self.original_bar2_width*percentage, self.original_bar2_height);
            }
        } 
        else {
            set_tex_coords(self.bars[1], EMPTY_TEXCOORDS);
            set_width_height(self.bars[1], 0.0, 0.0);
            set_pane_visible(self.bars[1], false);
            let percentage = self.visual_percentage/0.5;
            set_tex_coords(self.bars[0], [0.0, 0.0, percentage, 0.0, 0.0, 1.0, percentage, 1.0]);
            set_width_height(self.bars[0], self.original_bar_width*percentage, self.original_bar_height);
            set_pane_visible(self.bars[0], true);
        }
    }
    pub fn change_color_green(&mut self) {
        set_pane_colors(self.bars_background[0], BACKGROUND_GREEN, BACKGROUND_BLACK);
        set_pane_colors(self.bars_background[1], BACKGROUND_GREEN, BACKGROUND_BLACK);
    }
    pub fn change_color_purple(&mut self) {
        set_pane_colors(self.bars_background[0], BACKGROUND_PURPLE, BACKGROUND_BLACK);
        set_pane_colors(self.bars_background[1], BACKGROUND_PURPLE, BACKGROUND_BLACK);
    }
    pub fn update_percentages(&mut self) {
        self.visual_percentage = f32::min(self.visual_percentage-0.01, self.actual_percentage).clamp(0.0, 100.0);
    }
}

impl UiObject for PalutenaMeter {
    fn update(&mut self) {
        self.set_tex_coords();
        self.update_percentages();
    }
    fn is_valid(&self) -> bool {
        is_pane_valid(self.base_bar) && is_pane_valid(self.number)
    }
    fn set_enable(&mut self, enable: bool) {
        if enable && !self.enabled {
            self.reset();
        } 
        else if !enable {
            set_pane_visible(self.base_bar, false);
            set_pane_visible(self.number, false);
            set_pane_visible(self.bars[0], false);
            set_pane_visible(self.bars[1], false);
            set_pane_visible(self.bars_background[0], false);
            set_pane_visible(self.bars_background[1], false);
        }
        self.enabled = enable;
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn reset(&mut self) {
        self.reset();
    }
    fn change_color_green(&mut self) {
        self.change_color_green();
    }
    fn change_color_purple(&mut self) {
        self.change_color_purple();
    }
}

#[repr(C)]
pub struct UiManager {
    palutena_meter: [PalutenaMeter; 8]
}

impl UiManager {
    fn get_ui_index_from_entry_id(entry_id: u32) -> u32 {
        let mut index = 0;
        for n in 0..entry_id {
            if get_battle_object_from_entry_id(n).is_some() {
                index += 1;
            }
        }
        return index;
    }
    #[export_name = "UiManager__set_palutena_meter_enable"]
    pub extern "C" fn set_palutena_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        manager.palutena_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable(enable);
    }
    #[export_name = "UiManager__set_palutena_meter_info"]
    pub extern "C" fn set_palutena_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        let mut manager = UI_MANAGER.write();
        manager.palutena_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_meter_info(current, max, per_level);
    }
    #[export_name = "UiManager__reset_palutena_meter"]
    pub extern "C" fn reset_palutena_meter(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.palutena_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].reset();
    }
    #[export_name = "UiManager__change_palutena_meter_color_green"]
    pub extern "C" fn change_palutena_meter_color_green(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.palutena_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].change_color_green();
    }
    #[export_name = "UiManager__change_palutena_meter_color_purple"]
    pub extern "C" fn change_palutena_meter_color_purple(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.palutena_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].change_color_purple();
    }
}

static UI_MANAGER: Lazy<RwLock<UiManager>> = Lazy::new(||{RwLock::new(UiManager {palutena_meter: [PalutenaMeter::default(); 8]})});

#[skyline::hook(offset = 0x1b6cbe8, inline)]
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
}

pub fn install() {
    skyline::install_hooks!(
        get_set_info_alpha, 
        hud_update
    );
}