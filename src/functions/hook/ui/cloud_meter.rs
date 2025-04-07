use super::*;

#[derive(Default, Copy, Clone)]
pub struct CloudMeter {
    pub number: u64,
    pub value: i32,
    pub enabled: bool,
}

impl CloudMeter {
    pub fn new(layout_data: u64) -> Self {
        let number = get_pane_from_layout(layout_data, "cloud_number\0").expect("Couldn't find cloud_number");
        return Self {
            number: number,
            value: 0,
            enabled: false
        };
    }
    pub fn reset(&mut self) {
        set_pane_visible(self.number, true);
        self.value = 0;
    }
    pub fn set_meter_info(&mut self, value: i32) {
        self.value = value;
    }
    pub fn update_icon(&mut self) {
        let offset = match self.value {
            0 => 0.0,
            1 => 1.0,
            2 => 2.0,
            3 => 3.0,
            _ => -1.0
        };
        if offset < 0.0 {
            set_pane_visible(self.number, false);
            return;
        }
        let offset = offset/4.0;
        let len = 1.0/4.0;
        set_pane_visible(self.number, true);
        set_tex_coords(self.number, [offset, 0.0, offset+len, 0.0, offset, 1.0, offset+len, 1.0]);
    }
}