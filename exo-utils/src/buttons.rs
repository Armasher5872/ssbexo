//The following two are credited to WuBoytH
use super::*;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct Buttons: i32 {
        const Attack      = 0x1;
        const Special     = 0x2;
        const Jump        = 0x4;
        const Guard       = 0x8;
        const Catch       = 0x10;
        const Smash       = 0x20;
        const JumpMini    = 0x40;
        const CStickOn    = 0x80;
        const StockShare  = 0x100;
        const AttackRaw   = 0x200;
        const AppealHi    = 0x400;
        const SpecialRaw  = 0x800;
        const AppealLw    = 0x1000;
        const AppealSL    = 0x2000;
        const AppealSR    = 0x4000;
        const FlickJump   = 0x8000;
        const GuardHold   = 0x10000;
        const SpecialRaw2 = 0x20000;
        // We leave a blank at 0x4000 because the internal control mapping will map 1 << InputKind to the button bitfield, and so our shorthop button
        // would get mapped to FullHop (issue #776)
        const FullHop  = 0x80000;
        const CStickOverride = 0x100000;

        const SpecialAll  = 0x20802;
        const AttackAll   = 0x201;
        const AppealAll   = 0x7400;
    }
}

pub mod cat4 {
    pub const SPECIAL_N_COMMAND : usize = 0x0;
    pub const SPECIAL_N2_COMMAND : usize = 0x1;
    pub const SPECIAL_S_COMMAND : usize = 0x2;
    pub const SPECIAL_HI_COMMAND : usize = 0x3;
    pub const COMMAND_6N6 : usize = 0x4;
    pub const COMMAND_4N4 : usize = 0x5;
    pub const ATTACK_COMMAND1 : usize = 0x6;
    pub const SPECIAL_HI2_COMMAND : usize = 0x7;
    pub const SUPER_SPECIAL_COMMAND : usize = 0x8;
    pub const SUPER_SPECIAL_R_COMMAND : usize = 0x9;
    pub const SUPER_SPECIAL2_COMMAND : usize = 0xA;
    pub const SUPER_SPECIAL2_R_COMMAND : usize = 0xB;
    pub const COMMAND_623NB : usize = 0xC;
    pub const COMMAND_623STRICT : usize = 0xD;
    pub const COMMAND_623ALONG : usize = 0xE;
    pub const COMMAND_623BLONG : usize = 0xF;
    pub const COMMAND_623A : usize = 0x10;
    pub const COMMAND_2 : usize = 0x11;
    pub const COMMAND_3 : usize = 0x12;
    pub const COMMAND_1 : usize = 0x13;
    pub const COMMAND_6 : usize = 0x14;
    pub const COMMAND_4 : usize = 0x15;
    pub const COMMAND_8 : usize = 0x16;
    pub const COMMAND_9 : usize = 0x17;
    pub const COMMAND_7 : usize = 0x18;
    pub const COMMAND_6N6AB : usize = 0x19;
    pub const COMMAND_323CATCH : usize = 0x1A;
}