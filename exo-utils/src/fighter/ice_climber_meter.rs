use super::*;

static mut PERCENTAGE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

#[derive(Default, Copy, Clone)]
pub struct IceClimberMeter {
    pub percent_1: u64,
    pub percent_2: u64,
    pub percent_3: u64,
    pub element_1: i32,
    pub element_2: i32,
    pub element_3: i32,
    pub percentage: u64,
    pub enabled_1: bool,
    pub enabled_2: bool,
    pub enabled_3: bool,
}

impl IceClimberMeter {
    pub fn new(layout_data: u64) -> Self {
        let number_1 = get_pane_from_layout(layout_data, "iceclimber_number_1\0").expect("Couldn't find iceclimber_number_1");
        let number_2 = get_pane_from_layout(layout_data, "iceclimber_number_2\0").expect("Couldn't find iceclimber_number_2");
        let number_3 = get_pane_from_layout(layout_data, "iceclimber_number_3\0").expect("Couldn't find iceclimber_number_3");
        let percentage = get_pane_from_layout(layout_data, "iceclimber_percent\0").expect("Couldn't find iceclimber_percent");
        return Self {
            percent_1: number_1,
            percent_2: number_2,
            percent_3: number_3,
            element_1: 0,
            element_2: 0,
            element_3: 0,
            percentage: percentage,
            enabled_1: false,
            enabled_2: false,
            enabled_3: false
        };
    }
    pub fn reset(&mut self) {
        set_pane_visible(self.percent_1, true);
        set_pane_visible(self.percent_2, true);
        set_pane_visible(self.percent_3, true);
        set_pane_visible(self.percentage, true);
        self.element_1 = 0;
        self.element_2 = 0;
        self.element_3 = 0;
    }
    pub fn set_meter_info(&mut self, element_1: i32, element_2: i32, element_3: i32) {
        self.element_1 = element_1;
        self.element_2 = element_2;
        self.element_3 = element_3;
    }
    pub fn update_icon_1(&mut self) {
        let offset_1 = match self.element_1 {
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
            _ => -1.0
        };
        if offset_1 < 0.0 {
            set_pane_visible(self.percent_1, false);
            return;
        }
        let offset_1 = offset_1/10.0;
        let len = 1.0/10.0;
        set_pane_visible(self.percent_1, true);
        set_tex_coords(self.percent_1, [offset_1, 0.0, offset_1+len, 0.0, offset_1, 1.0, offset_1+len, 1.0]);
    }
    pub fn update_icon_2(&mut self) {
        let offset_2 = match self.element_2 {
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
            _ => -1.0
        };
        if offset_2 < 0.0 {
            set_pane_visible(self.percent_2, false);
            return;
        }
        let offset_2 = offset_2/10.0;
        let len = 1.0/10.0;
        set_pane_visible(self.percent_2, true);
        set_tex_coords(self.percent_2, [offset_2, 0.0, offset_2+len, 0.0, offset_2, 1.0, offset_2+len, 1.0]);
    }
    pub fn update_icon_3(&mut self) {
        let offset_3 = match self.element_3 {
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
            _ => -1.0
        };
        if offset_3 < 0.0 {
            set_pane_visible(self.percent_3, false);
            return;
        }
        let offset_3 = offset_3/10.0;
        let len = 1.0/10.0;
        set_pane_visible(self.percent_3, true);
        set_tex_coords(self.percent_3, [offset_3, 0.0, offset_3+len, 0.0, offset_3, 1.0, offset_3+len, 1.0]);
    }
    pub unsafe fn update_color(&mut self, percentage: f32) {
        match percentage {
            0.0..=40.0 => {
                PERCENTAGE_COLOR[0] = 1.0-(percentage/(10200.0/7.0)); //RGB: 249
                PERCENTAGE_COLOR[1] = 1.0-(percentage/(5100.0/31.0)); //RGB: 194
                PERCENTAGE_COLOR[2] = 1.0-(percentage/(2040.0/37.0)); //RGB: 71
            },
            41.0..=70.0 => {
                PERCENTAGE_COLOR[0] = 0.973-(percentage/(5950.0/3.0)); //RGB: 239
                PERCENTAGE_COLOR[1] = 0.757-(percentage/(3570.0/19.0)); //RGB: 98
                PERCENTAGE_COLOR[2] = 0.275-(percentage/1190.0); //RGB: 55
            },
            71.0..=150.0 => {
                PERCENTAGE_COLOR[0] = 0.937-(percentage/1159.1); //RGB: 206
                PERCENTAGE_COLOR[1] = 0.384-(percentage/750.0); //RGB: 47
                PERCENTAGE_COLOR[2] = 0.216-(percentage/3187.5); //RGB: 43
            },
            151.0..=300.0 => {
                PERCENTAGE_COLOR[0] = 0.808-(percentage/813.830); //RGB: 112
                PERCENTAGE_COLOR[1] = 0.184-(percentage/3187.5); //RGB: 23
                PERCENTAGE_COLOR[2] = 0.169-(percentage/9562.5); //RGB: 35
            }
            _ => {
                PERCENTAGE_COLOR[0] = 0.439;
                PERCENTAGE_COLOR[1] = 0.09;
                PERCENTAGE_COLOR[2] = 0.137;
            }
        }
        set_pane_colors(self.percent_1, PERCENTAGE_COLOR, PERCENTAGE_COLOR);
        set_pane_colors(self.percent_2, PERCENTAGE_COLOR, PERCENTAGE_COLOR);
        set_pane_colors(self.percent_3, PERCENTAGE_COLOR, PERCENTAGE_COLOR);
        set_pane_colors(self.percentage, PERCENTAGE_COLOR, PERCENTAGE_COLOR);
    }
}