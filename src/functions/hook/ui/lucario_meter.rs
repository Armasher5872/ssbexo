use super::*;

#[derive(Default, Copy, Clone)]
pub struct LucarioMeter {
    pub number: u64,
    pub value: i32,
    pub icon: u64,
    pub enabled: bool,
}

impl LucarioMeter {
    pub fn new(layout_data: u64) -> Self {
        let number = get_pane_from_layout(layout_data, "lucario_number\0").expect("Couldn't find lucario_number");
        let icon = get_pane_from_layout(layout_data, "lucario_icon\0").expect("Couldn't find lucario_icon");
        return Self {
            number: number,
            value: 0,
            icon: icon,
            enabled: false
        };
    }
    pub fn reset(&mut self) {
        set_pane_visible(self.number, true);
        set_pane_visible(self.icon, true);
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
            4 => 4.0,
            5 => 5.0,
            6 => 6.0,
            7 => 7.0,
            8 => 8.0,
            9 => 9.0,
            10 => 10.0,
            _ => -1.0
        };
        if offset < 0.0 {
            set_pane_visible(self.number, false);
            return;
        }
        let offset = offset/11.0;
        let len = 1.0/11.0;
        set_pane_visible(self.number, true);
        set_tex_coords(self.number, [offset, 0.0, offset+len, 0.0, offset, 1.0, offset+len, 1.0]);
    }
}