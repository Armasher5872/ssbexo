#![allow(non_upper_case_globals)]
use super::*;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct Cat1: i32 {
        const AttackN       = 0x1;
        const AttackS3      = 0x2;
        const AttackHi3     = 0x4;
        const AttackLw3     = 0x8;
        const AttackS4      = 0x10;
        const AttackHi4     = 0x20;
        const AttackLw4     = 0x40;
        const AttackAirN    = 0x80;
        const AttackAirF    = 0x100;
        const AttackAirB    = 0x200;
        const AttackAirHi   = 0x400;
        const AttackAirLw   = 0x800;
        const SpecialN      = 0x1000;
        const SpecialS      = 0x2000;
        const SpecialHi     = 0x4000;
        const SpecialLw     = 0x8000;
        const SpecialAny    = 0xF000;
        const Walk          = 0x10000;
        const Dash          = 0x20000;
        const Turn          = 0x40000;
        const TurnDash      = 0x80000;
        const Jump          = 0x100000;
        const JumpButton    = 0x200000;
        const AirEscape     = 0x400000;
        const Squat         = 0x800000;
        const Escape        = 0x1000000;
        const EscapeF       = 0x2000000;
        const EscapeB       = 0x4000000;
        const WallJumpLeft  = 0x8000000;
        const WallJumpRight = 0x10000000;
        const Catch         = 0x20000000;
        const NoCmd         = 0x40000000;
    }
    #[derive(Copy, Clone)]
    pub struct Cat2: i32 {
        const AppealSL            = 0x1;
        const AppealSR            = 0x2;
        const AppealHi            = 0x4;
        const AppealLw            = 0x8;
        const AppealSmash         = 0x10;
        const AppealAll           = 0x1F;
        const AttackDashAttackHi4 = 0x20;
        const FallJump            = 0x40;
        const DashAttackS4        = 0x80;
        const DamageFallToFall    = 0x100;
        const DownToDownStandFB   = 0x200;
        const DownToDownStand     = 0x400;
        const GuardToPass         = 0x800;
        const SquatToSquatF       = 0x1000;
        const SquatToSquatB       = 0x2000;
        const TurnToEscapeF       = 0x4000;
        const TurnToEscapeB       = 0x8000;
        const StickEscapeF        = 0x10000;
        const StickEscapeB        = 0x20000;
        const StickEscape         = 0x40000;
        const SpecialNReverseLR   = 0x80000;
        const ThrowF              = 0x100000;
        const ThrowB              = 0x200000;
        const ThrowHi             = 0x400000;
        const ThrowLw             = 0x800000;
        const CommonGuard         = 0x1000000;
        const AirLasso            = 0x2000000;
        const AttackN2            = 0x4000000;
        const FinalReverseLR      = 0x8000000;
    }
    #[derive(Copy, Clone)]
    pub struct Cat3: i32 {
        const ItemLightThrowFB4    = 0x1;
        const ItemLightThrowHi4    = 0x2;
        const ItemLightThrowLw4    = 0x4;
        const ItemLightThrowHi     = 0x8;
        const ItemLightThrowLw     = 0x10;
        const ItemLightDrop        = 0x20;
        const ItemLightThrowFB     = 0x40;
        const ItemLightThrowAirFB  = 0x80;
        const ItemLightThrowAirFB4 = 0x100;
        const ItemLightThrowAirHi  = 0x200;
        const ItemLightThrowAirHi4 = 0x400;
        const ItemLightThrowAirLw  = 0x800;
        const ItemLightThrowAirLw4 = 0x1000;
        const ItemLightDropAir     = 0x2000;
        const ItemHeavyThrowFB     = 0x4000;
        const ItemGetAir           = 0x8000;
        const SpecialSSmash        = 0x10000;
        const SpecialSSmashDash    = 0x20000;

        const ItemLightThrow       = 0x58;
        const ItemLightThrowAir    = 0xA80;
        const ItemLightThrow4      = 0x7;
        const ItemLightThrow4Air   = 0x1500;
        const ItemLightThrowAll    = 0x5F;
        const ItemLightThrowAirAll = 0x1F80;
    }
    #[derive(Copy, Clone)]
    pub struct Cat4: i32 {
        const SpecialNCommand       = 0x1; //236
        const SpecialN2Command      = 0x2; //41246
        const SpecialSCommand       = 0x4; //214
        const SpecialHiCommand      = 0x8;
        const Command6N6            = 0x10;
        const Command4N4            = 0x20;
        const AttackCommand1        = 0x40;
        const SpecialHi2Command     = 0x80;
        const SuperSpecialCommand   = 0x100;
        const SuperSpecialRCommand  = 0x200;
        const SuperSpecial2Command  = 0x400;
        const SuperSpecial2RCommand = 0x800;
        const Command623NB          = 0x1000;
        const Command623Strict      = 0x2000;
        const Command623ALong       = 0x4000;
        const Command623BLong       = 0x8000;
        const Command623A           = 0x10000;
        const Command2              = 0x20000;
        const Command3              = 0x40000;
        const Command1              = 0x80000;
        const Command6              = 0x100000;
        const Command4              = 0x200000;
        const Command8              = 0x400000;
        const Command9              = 0x800000;
        const Command7              = 0x1000000;
        const Command6N6AB          = 0x2000000;
        const Command323Catch       = 0x4000000;
    }
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

#[derive(Copy, Clone)]
pub enum CommandCat {
    Cat1(Cat1),
    Cat2(Cat2),
    Cat3(Cat3),
    Cat4(Cat4)
}

impl Into<CommandCat> for Cat1 {
    fn into(self) -> CommandCat {
        CommandCat::Cat1(self)
    }
}

impl Into<CommandCat> for Cat2 {
    fn into(self) -> CommandCat {
        CommandCat::Cat2(self)
    }
}

impl Into<CommandCat> for Cat3 {
    fn into(self) -> CommandCat {
        CommandCat::Cat3(self)
    }
}

impl Into<CommandCat> for Cat4 {
    fn into(self) -> CommandCat {
        CommandCat::Cat4(self)
    }
}

impl Cat1 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe {
            Cat1::from_bits_retain(ControlModule::get_command_flag_cat(boma, 0))
        }
    }
}

impl Cat2 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe { 
            Cat2::from_bits_retain(ControlModule::get_command_flag_cat(boma, 1)) 
        }
    }
}

impl Cat3 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe { 
            Cat3::from_bits_retain(ControlModule::get_command_flag_cat(boma, 2)) 
        }
    }
}

impl Cat4 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe { 
            Cat4::from_bits_retain(ControlModule::get_command_flag_cat(boma, 3)) 
        }
    }
}