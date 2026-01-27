use super::*;

#[derive(Default, Copy, Clone)]
pub struct SonicMeter {
    pub base_bar: u64,
    pub bar1: u64,
    pub bar2: u64,
    pub original_bar_width: f32,
    pub original_bar_height: f32,
    pub original_bar2_width: f32,
    pub original_bar2_height: f32,
    pub percent: f32,
    pub enabled: bool,
}

impl SonicMeter {
    pub fn new(layout_data: u64) -> Self {
        let base_bar = get_pane_from_layout(layout_data, "sonic_meter_base\0").expect("Could not find base meter!");
        let bar1 = get_pane_from_layout(layout_data, "sonic_bar1\0").expect("Could not find first bar!");
        let bar2 = get_pane_from_layout(layout_data, "sonic_bar2\0").expect("Could not find second bar!");
        return Self {
            base_bar: base_bar,
            bar1: bar1,
            bar2: bar2,
            original_bar_width: -1.0,
            original_bar_height: -1.0,
            original_bar2_width: -1.0,
            original_bar2_height: -1.0,
            percent: -1.0,
            enabled: false
        };
    }
    pub fn reset(&mut self) {
        set_pane_visible(self.base_bar, true);
        set_pane_visible(self.bar1, true);
        set_pane_visible(self.bar2, true);
        if self.original_bar_height < 0.0 {
            self.original_bar_width = get_width_height(self.bar1).0;
            self.original_bar_height = get_width_height(self.bar1).1;
            self.original_bar2_width = get_width_height(self.bar2).0;
            self.original_bar2_height = get_width_height(self.bar2).1;
        }
        self.percent = 0.0;
    }
    pub fn set_percent(&mut self, gauge: f32) {
        let percent = gauge/100.0;
        self.percent = percent;
    }
    pub fn update_number(&mut self) {
        let percentage = 0.38+(self.percent*0.52);
        set_tex_coords(self.bar2, [0.0, 0.0, percentage, 0.0, 0.0, 1.0, percentage, 1.0]);
        set_width_height(self.bar2, self.original_bar2_width*percentage, self.original_bar2_height);
    }
}