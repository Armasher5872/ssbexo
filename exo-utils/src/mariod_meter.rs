use super::*;

#[derive(Default, Copy, Clone)]
pub struct MarioDMeter {
    pub icon: u64,
    pub element: i32,
    pub is_enabled: bool,
}

impl MarioDMeter {
    pub fn new(layout_data: u64) -> Self {
        let pill_pane = get_pane_from_layout(layout_data, "pill_pane\0").expect("Couldn't find pill pane");
        return Self {
            icon: pill_pane,
            element: 0,
            is_enabled: false
        };
    }
    pub fn reset(&mut self) {
        set_pane_visible(self.icon, true);
        self.element = 0;
    }
    pub fn set_meter_info(&mut self, element: i32) {
        self.element = element;
    }
    pub fn update_icon(&mut self) {
        let offset = match self.element {
            0 => 0.0,
            1 => 1.0,
            2 => 2.0,
            3 => 3.0,
            _ => -1.0
        };
        if offset < 0.0 {
            set_pane_visible(self.icon, false);
            return;
        }
        let offset = offset/4.0;
        let len = 1.0/4.0;
        set_pane_visible(self.icon, true);
        set_tex_coords(self.icon, [offset, 0.0, offset+len, 0.0, offset, 1.0, offset+len, 1.0]);
    }
}