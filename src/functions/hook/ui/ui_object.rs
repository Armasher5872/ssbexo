#![allow(unconditional_recursion)]
use super::*;

pub trait UiObject {
    fn update(&mut self);
    fn is_valid(&self) -> bool;
    fn set_enable(&mut self, enable: bool);
    fn is_enabled(&self) -> bool;
    fn reset(&mut self);
    fn change_color_green(&mut self);
    fn change_color_purple(&mut self);
    fn change_color_blue(&mut self);
    fn change_color_yellow(&mut self);
    fn change_color_red(&mut self);
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
    fn change_color_blue(&mut self) {
        self.change_color_blue();
    }
    fn change_color_yellow(&mut self) {
        self.change_color_yellow();
    }
    fn change_color_red(&mut self) {
        self.change_color_red();
    }
}

impl UiObject for RobotMeter {
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
    fn change_color_blue(&mut self) {
        self.change_color_blue();
    }
    fn change_color_yellow(&mut self) {
        self.change_color_yellow();
    }
    fn change_color_red(&mut self) {
        self.change_color_red();
    }
}