use super::*;

const BACKGROUND_BLUE: [f32; 4] = [0.251, 0.878, 0.816, 1.0];
const BACKGROUND_YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const BACKGROUND_RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BACKGROUND_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const FOREGROUND_CHARGE_WHITE: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; //Controls bottom color
const FOREGROUND_CHARGE_BLACK: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; //Controls top color

const FULL_TEXCOORDS: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];

const EMPTY_TEXCOORDS: [f32; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0];

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct RobotMeter {
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
    pub enabled: bool,
}

impl RobotMeter {
    pub fn new(layout_data: u64) -> Self {
        let base_bar = get_pane_from_layout(layout_data, "robot_meter_base\0").expect("Could not find base meter!");
        let bar1 = get_pane_from_layout(layout_data, "robot_meter_bar1\0").expect("Could not find first bar!");
        let bar2 = get_pane_from_layout(layout_data, "robot_meter_bar2\0").expect("Could not find second bar!");
        let bar1_bg = get_pane_from_layout(layout_data, "robot_meter_bar1_bg\0").expect("Could not find first bg bar!");
        let bar2_bg = get_pane_from_layout(layout_data, "robot_meter_bar2_bg\0").expect("Could not find second bg bar!");
        let number = get_pane_from_layout(layout_data, "robot_meter_num\0").expect("Could not find number!");
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
    pub fn change_color_blue(&mut self) {
        set_pane_colors(self.bars_background[0], BACKGROUND_BLUE, BACKGROUND_BLACK);
        set_pane_colors(self.bars_background[1], BACKGROUND_BLUE, BACKGROUND_BLACK);
    }
    pub fn change_color_yellow(&mut self) {
        set_pane_colors(self.bars_background[0], BACKGROUND_YELLOW, BACKGROUND_BLACK);
        set_pane_colors(self.bars_background[1], BACKGROUND_YELLOW, BACKGROUND_BLACK);
    }
    pub fn change_color_red(&mut self) {
        set_pane_colors(self.bars_background[0], BACKGROUND_RED, BACKGROUND_BLACK);
        set_pane_colors(self.bars_background[1], BACKGROUND_RED, BACKGROUND_BLACK);
    }
    pub fn update_percentages(&mut self) {
        self.visual_percentage = f32::min(self.visual_percentage-0.01, self.actual_percentage).clamp(0.0, 100.0);
    }
}