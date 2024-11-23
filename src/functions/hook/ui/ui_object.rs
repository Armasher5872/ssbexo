#![allow(dead_code, unconditional_recursion)]
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

pub trait IceClimberUiObject {
    fn update_1(&mut self);
    fn update_2(&mut self);
    fn update_3(&mut self);
    fn is_valid_1(&self) -> bool;
    fn is_valid_2(&self) -> bool;
    fn is_valid_3(&self) -> bool;
    fn is_valid_4(&self) -> bool;
    fn set_enable_1(&mut self, enable: bool);
    fn set_enable_2(&mut self, enable: bool);
    fn set_enable_3(&mut self, enable: bool);
    fn is_enabled_1(&self) -> bool;
    fn is_enabled_2(&self) -> bool;
    fn is_enabled_3(&self) -> bool;
    unsafe fn update_color(&mut self, percentage: f32);
}

pub trait MarioDUiObject {
    fn update(&mut self);
    fn is_valid(&self) -> bool;
    fn set_enable(&mut self, enable: bool);
    fn is_enabled(&self) -> bool;
    fn pill_id(&self) -> i32;
}

pub trait LucarioUiObject {
    fn update(&mut self);
    fn is_valid(&self) -> bool;
    fn set_enable(&mut self, enable: bool);
    fn is_enabled(&self) -> bool;
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

impl IceClimberUiObject for IceClimberMeter {
    fn update_1(&mut self) {
        self.update_icon_1();
    }
    fn update_2(&mut self) {
        self.update_icon_2();
    }
    fn update_3(&mut self) {
        self.update_icon_3();
    }
    fn is_valid_1(&self) -> bool {
        is_pane_valid(self.percent_1)
    }
    fn is_valid_2(&self) -> bool {
        is_pane_valid(self.percent_2)
    }
    fn is_valid_3(&self) -> bool {
        is_pane_valid(self.percent_3)
    }
    fn is_valid_4(&self) -> bool {
        is_pane_valid(self.percentage)
    }
    fn set_enable_1(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.percent_1, false);
            set_pane_visible(self.percentage, false);
        }
        else if !self.enabled_1 {
            self.reset();
        }
        self.enabled_1 = enable;
    }
    fn set_enable_2(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.percent_2, false);
        }
        else if !self.enabled_2 {
            self.reset();
        }
        self.enabled_2 = enable;
    }
    fn set_enable_3(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.percent_3, false);
        }
        else if !self.enabled_3 {
            self.reset();
        }
        self.enabled_3 = enable;
    }
    fn is_enabled_1(&self) -> bool {
        self.enabled_1
    }
    fn is_enabled_2(&self) -> bool {
        self.enabled_2
    }
    fn is_enabled_3(&self) -> bool {
        self.enabled_3
    }
    unsafe fn update_color(&mut self, percent: f32) {
        self.update_color(percent);
    }
}

impl MarioDUiObject for MarioDMeter {
    fn update(&mut self) {
        self.update_icon();
    }
    fn is_valid(&self) -> bool {
        return is_pane_valid(self.icon);
    }
    fn set_enable(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.icon, false);
        } 
        else if !self.is_enabled {
            self.reset();
        }
        self.is_enabled = enable;
    }
    fn is_enabled(&self) -> bool {
        return self.is_enabled;
    }
    fn pill_id(&self) -> i32 {
        return self.element;
    }
}

impl LucarioUiObject for LucarioMeter {
    fn update(&mut self) {
        self.update_icon();
    }
    fn is_valid(&self) -> bool {
        is_pane_valid(self.number) && is_pane_valid(self.icon)
    }
    fn set_enable(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.number, false);
            set_pane_visible(self.icon, false);
        }
        else if !self.enabled {
            self.reset();
        }
        self.enabled = enable;
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}