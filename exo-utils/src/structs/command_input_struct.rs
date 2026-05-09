//The functions here are credited to WuBoyTH
use super::*;

//Command Input State, used for command input handling
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CommandInputState {
    pub vtable: u64,
    pub command_timer: u8,
    pub state: u8,
    pub unk2: u8,
    pub input_allow: InputAllow,
    pub max_timer: u8,
    pub enable_timer: u8,
    pub lr: i8
}

//Command Input Flags, relates to command input addition
bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CommandInputFlags: u32 {
        const UP = 0b10;
        const DOWN = 0b100;
        const LEFT = 0b1000;
        const RIGHT = 0b10000;

        const UP_LEFT = 0b100000;
        const DOWN_LEFT = 0b1000000;
        const UP_RIGHT = 0b10000000;
        const DOWN_RIGHT = 0b100000000;

        const ATTACK_EDGE = 0b1000000000;
        const SPECIAL_EDGE = 0b10000000000;
        const GRAB_EDGE = 0b100000000000;

        const ATTACK_PRESSING = 0b1000000000000;
        const SPECIAL_PRESSING = 0b10000000000000;
        const ATTACK_RAW_PRESSING = 0b100000000000000; // IDK CHIEF

        const ANY_DIRECTION = 0x1FE;
    }
    #[derive(Debug, Copy, Clone)]
    pub struct InputAllow: u8 {
        const ATTACK = 0x1;
        const SPECIAL = 0x2;
    }
}

impl CommandInputFlags {
    pub fn back(&self, lr: f32) -> bool {
        if lr < 0.0 {
            self.intersects(Self::RIGHT)
        } 
        else {
            self.intersects(Self::LEFT)
        }
    }
    pub fn back_down(&self, lr: f32) -> bool {
        if lr < 0.0 {
            self.intersects(Self::DOWN_RIGHT)
        } 
        else {
            self.intersects(Self::DOWN_LEFT)
        }
    }
    pub fn back_up(&self, lr: f32) -> bool {
        if lr < 0.0 {
            self.intersects(Self::UP_RIGHT)
        } 
        else {
            self.intersects(Self::UP_LEFT)
        }
    }
    pub fn front(&self, lr: f32) -> bool {
        if lr > 0.0 {
            self.intersects(Self::RIGHT)
        } 
        else {
            self.intersects(Self::LEFT)
        }
    }
    pub fn front_down(&self, lr: f32) -> bool {
        if lr > 0.0 {
            self.intersects(Self::DOWN_RIGHT)
        } 
        else {
            self.intersects(Self::DOWN_LEFT)
        }
    }
    pub fn front_up(&self, lr: f32) -> bool {
        if lr > 0.0 {
            self.intersects(Self::UP_RIGHT)
        } 
        else {
            self.intersects(Self::UP_LEFT)
        }
    }
    pub fn up(&self) -> bool {
        self.intersects(Self::UP)
    }
    pub fn down(&self) -> bool {
        self.intersects(Self::DOWN)
    }
    pub fn left(&self) -> bool {
        self.intersects(Self::LEFT)
    }
    pub fn right(&self) -> bool {
        self.intersects(Self::RIGHT)
    }
    pub fn up_right(&self) -> bool {
        self.intersects(Self::UP_RIGHT)
    }
    pub fn up_left(&self) -> bool {
        self.intersects(Self::UP_LEFT)
    }
    pub fn down_right(&self) -> bool {
        self.intersects(Self::DOWN_RIGHT)
    }
    pub fn down_left(&self) -> bool {
        self.intersects(Self::DOWN_LEFT)
    }
}

impl InputAllow {
    pub fn check(&self, inputs: &CommandInputFlags) -> bool {
        let mut input = false;
        if self.intersects(Self::ATTACK) && inputs.intersects(CommandInputFlags::ATTACK_EDGE) {
            input |= true;
        }
        if self.intersects(Self::SPECIAL) && inputs.intersects(CommandInputFlags::SPECIAL_EDGE) {
            input |= true;
        }
        input
    }
}