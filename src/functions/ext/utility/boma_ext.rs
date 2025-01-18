#![allow(unused_parens)]
use super::*;

//BomaExt, helps with various things
pub trait BomaExt {
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
	unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32;
    unsafe fn is_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn down_input(&mut self) -> bool;
    unsafe fn check_jump_cancel(&mut self, update_lr: bool, skip_other_checks: bool) -> bool;
    unsafe fn gimmick_flash(&mut self);
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn is_item(&mut self) -> bool;
    unsafe fn status(&mut self) -> i32;
    unsafe fn magic_series(&mut self) -> i32;
}

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn is_fighter(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_FIGHTER;
    }
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::status_kind(self);
        return kinds.contains(&kind);
    }
	unsafe fn is_weapon(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_WEAPON;
    }
    unsafe fn kind(&mut self) -> i32 {
        return smash::app::utility::get_kind(self);
    }
    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32 {
        return StatusModule::change_status_request_from_script(self, kind, repeat) as i32;
    }
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).intersects(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).intersects(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).intersects(cat)
        }
    }
    unsafe fn is_situation(&mut self, kind: i32) -> bool {
        return StatusModule::situation_kind(self) == kind;
    }
    unsafe fn down_input(&mut self) -> bool {
        let stick_y = ControlModule::get_stick_y(self);
        //Checks if you're holding down the control stick less than the shield drop threshold
        if stick_y <= -0.6875 {
            return true;
        }
        //Checks if you flick the stick down more than 3 times but less than 20 times, or your stick is less than or equal to -1.0
        if ((ControlModule::get_flick_y(self) >= 3 && ControlModule::get_flick_y(self) < 20) || stick_y <= -1.0) {
            return true;
        };
        return false;
    }
    unsafe fn check_jump_cancel(&mut self, update_lr: bool, skip_other_checks: bool) -> bool {
        let fighter = crate::functions::util::get_fighter_common_from_accessor(self);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            if !skip_other_checks {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            }
            if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool()
            || (!skip_other_checks && fighter.sub_transition_group_check_ground_attack().get_bool())
            || fighter.sub_transition_group_check_ground_jump().get_bool() {
                if update_lr {
                    PostureModule::set_stick_lr(self, 0.0);
                    PostureModule::update_rot_y_lr(self);
                }
                return true;
            }
        } 
        else {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT);
            if fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
                return true;
            }
        }
        false
    }
    unsafe fn gimmick_flash(&mut self) {
        let fighter = crate::functions::util::get_fighter_common_from_accessor(self);
        let lr = PostureModule::lr(fighter.module_accessor);
        let offset = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
        if !smash::app::sv_information::is_ready_go() {
            return
        }
        smash::app::FighterUtil::flash_eye_info(fighter.module_accessor);
        if WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        }
        else {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, offset, 2, 0, 0, 0, 1.0, true);
        }
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.831, 0.686, 0.216);
    }
    unsafe fn is_status(&mut self, kind: i32) -> bool {
        return StatusModule::status_kind(self) == kind;
    }
    unsafe fn is_item(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_ITEM;
    }
    //Gets the current status kind for the fighter
    unsafe fn status(&mut self) -> i32 {
        return StatusModule::status_kind(self);
    }
    unsafe fn magic_series(&mut self) -> i32 {
        let fighter = crate::functions::util::get_fighter_common_from_accessor(self);
        let status_kind = fighter.global_table[STATUS_KIND].get_i32();
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            //Jab Cancels
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK
            && [hash40("attack_11"), hash40("attack_12")].contains(&motion_kind) {
                if fighter.is_cat_flag(Cat1::AttackS3) {
                    return 1;
                }
                else if fighter.is_cat_flag(Cat1::AttackHi3) {
                    return 2;
                }
                else if fighter.is_cat_flag(Cat1::AttackLw3) {
                    return 3;
                }
                else if fighter.is_cat_flag(Cat1::Dash) && fighter.is_cat_flag(Cat1::AttackN) {
                    return 4;
                }
                else if fighter.is_cat_flag(Cat1::AttackS4) {
                    return 5;
                }
                else if fighter.is_cat_flag(Cat1::AttackHi4) {
                    return 6;
                }
                else if fighter.is_cat_flag(Cat1::AttackLw4) {
                    return 7;
                }
                else if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 8;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 9;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 10;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 11;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 12;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 13;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 14;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 15;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 16;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 17;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 18;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 19;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 20;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 21;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 22;
                }
                else if fighter.is_cat_flag(Cat1::SpecialN) {
                    return 23;
                }
                else if fighter.is_cat_flag(Cat1::SpecialS) {
                    return 24;
                }
                else if fighter.is_cat_flag(Cat1::SpecialHi) {
                    return 25;
                }
                else if fighter.is_cat_flag(Cat1::SpecialLw) {
                    return 26;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
                if fighter.is_cat_flag(Cat1::AttackS4) {
                    return 27;
                }
                else if fighter.is_cat_flag(Cat1::AttackHi4) {
                    return 28;
                }
                else if fighter.is_cat_flag(Cat1::AttackLw4) {
                    return 29;
                }
                else if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 30;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 31;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 32;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 33;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 34;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 35;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 36;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 37;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 38;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 39;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 40;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 41;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 42;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 43;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 44;
                }
                else if fighter.is_cat_flag(Cat1::SpecialN) {
                    return 45;
                }
                else if fighter.is_cat_flag(Cat1::SpecialS) {
                    return 46;
                }
                else if fighter.is_cat_flag(Cat1::SpecialHi) {
                    return 47;
                }
                else if fighter.is_cat_flag(Cat1::SpecialLw) {
                    return 48;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                if fighter.is_cat_flag(Cat1::AttackS4) {
                    return 49;
                }
                else if fighter.is_cat_flag(Cat1::AttackHi4) {
                    return 50;
                }
                else if fighter.is_cat_flag(Cat1::AttackLw4) {
                    return 51;
                }
                else if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 52;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 53;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 54;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 55;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 56;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 57;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 58;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 59;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 60;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 61;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 62;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 63;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 64;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 65;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 66;
                }
                else if fighter.is_cat_flag(Cat1::SpecialN) {
                    return 67;
                }
                else if fighter.is_cat_flag(Cat1::SpecialS) {
                    return 68;
                }
                else if fighter.is_cat_flag(Cat1::SpecialHi) {
                    return 69;
                }
                else if fighter.is_cat_flag(Cat1::SpecialLw) {
                    return 70;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
                if fighter.is_cat_flag(Cat1::AttackS4) {
                    return 71;
                }
                else if fighter.is_cat_flag(Cat1::AttackHi4) {
                    return 72;
                }
                else if fighter.is_cat_flag(Cat1::AttackLw4) {
                    return 73;
                }
                else if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 74;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 75;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 76;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 77;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 78;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 79;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 80;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 81;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 82;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 83;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 84;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 85;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 86;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 87;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 88;
                }
                else if fighter.is_cat_flag(Cat1::SpecialN) {
                    return 89;
                }
                else if fighter.is_cat_flag(Cat1::SpecialS) {
                    return 90;
                }
                else if fighter.is_cat_flag(Cat1::SpecialHi) {
                    return 91;
                }
                else if fighter.is_cat_flag(Cat1::SpecialLw) {
                    return 92;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
                if fighter.is_cat_flag(Cat1::AttackS3) {
                    return 93;
                }
                if fighter.is_cat_flag(Cat1::AttackHi3) {
                    return 94;
                }
                if fighter.is_cat_flag(Cat1::AttackLw3) {
                    return 95;
                }
                else if fighter.is_cat_flag(Cat1::AttackS4) {
                    return 96;
                }
                else if fighter.is_cat_flag(Cat1::AttackHi4) {
                    return 97;
                }
                else if fighter.is_cat_flag(Cat1::AttackLw4) {
                    return 98;
                }
                else if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 99;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 100;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 101;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 102;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 103;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 104;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 105;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 106;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 107;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 108;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 109;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 110;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 111;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 112;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 113;
                }
                else if fighter.is_cat_flag(Cat1::SpecialN) {
                    return 114;
                }
                else if fighter.is_cat_flag(Cat1::SpecialS) {
                    return 115;
                }
                else if fighter.is_cat_flag(Cat1::SpecialHi) {
                    return 116;
                }
                else if fighter.is_cat_flag(Cat1::SpecialLw) {
                    return 117;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
                if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 118;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 119;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 120;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 121;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 122;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 123;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 124;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 125;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 126;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 127;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 128;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 129;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 130;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 131;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 132;
                }
                else if fighter.is_cat_flag(Cat1::SpecialN) {
                    return 133;
                }
                else if fighter.is_cat_flag(Cat1::SpecialS) {
                    return 134;
                }
                else if fighter.is_cat_flag(Cat1::SpecialHi) {
                    return 135;
                }
                else if fighter.is_cat_flag(Cat1::SpecialLw) {
                    return 136;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 137;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 138;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 139;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 140;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 141;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 142;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 143;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 144;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 145;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 146;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 147;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 148;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 149;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 150;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 151;
                }
                else if fighter.is_cat_flag(Cat1::SpecialN) {
                    return 152;
                }
                else if fighter.is_cat_flag(Cat1::SpecialS) {
                    return 153;
                }
                else if fighter.is_cat_flag(Cat1::SpecialHi) {
                    return 154;
                }
                else if fighter.is_cat_flag(Cat1::SpecialLw) {
                    return 155;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
                if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 156;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 157;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 158;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 159;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 160;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 161;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 162;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 163;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 164;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 165;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 166;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 167;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 168;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 169;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 170;
                }
                else if fighter.is_cat_flag(Cat1::SpecialN) {
                    return 171;
                }
                else if fighter.is_cat_flag(Cat1::SpecialS) {
                    return 172;
                }
                else if fighter.is_cat_flag(Cat1::SpecialHi) {
                    return 173;
                }
                else if fighter.is_cat_flag(Cat1::SpecialLw) {
                    return 174;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
                let attack_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
                if (attack_air_kind == hash40("attack_air_n") || motion_kind == hash40("attack_air_n")) {
                    if fighter.is_cat_flag(Cat1::AttackAirF) {
                        return 175;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirB) {
                        return 176;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirHi) {
                        return 177;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirLw) {
                        return 178;
                    }
                    if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                        return 179;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                        return 180;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                        return 181;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                        return 182;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                        return 183;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                        return 184;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                        return 185;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                        return 186;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                        return 187;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623NB) {
                        return 188;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623BLong) {
                        return 189;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623ALong) {
                        return 190;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623A) {
                        return 191;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623Strict) {
                        return 192;
                    }
                    else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                        return 193;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialN) {
                        return 194;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialS) {
                        return 195;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialHi) {
                        return 196;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialLw) {
                        return 197;
                    }
                    else {
                        return 0;
                    }
                }
                else if (attack_air_kind == hash40("attack_air_f") || motion_kind == hash40("attack_air_f")) {
                    if fighter.is_cat_flag(Cat1::AttackAirN) {
                        return 198;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirB) {
                        return 199;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirHi) {
                        return 200;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirLw) {
                        return 201;
                    }
                    if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                        return 202;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                        return 203;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                        return 204;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                        return 205;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                        return 206;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                        return 207;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                        return 208;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                        return 209;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                        return 210;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623NB) {
                        return 211;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623BLong) {
                        return 212;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623ALong) {
                        return 213;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623A) {
                        return 214;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623Strict) {
                        return 215;
                    }
                    else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                        return 216;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialN) {
                        return 217;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialS) {
                        return 218;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialHi) {
                        return 219;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialLw) {
                        return 220;
                    }
                    else {
                        return 0;
                    }
                }
                else if (attack_air_kind == hash40("attack_air_b") || motion_kind == hash40("attack_air_b")) {
                    if fighter.is_cat_flag(Cat1::AttackAirN) {
                        return 221;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirF) {
                        return 222;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirHi) {
                        return 223;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirLw) {
                        return 224;
                    }
                    if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                        return 225;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                        return 226;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                        return 227;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                        return 228;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                        return 229;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                        return 230;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                        return 231;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                        return 232;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                        return 233;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623NB) {
                        return 234;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623BLong) {
                        return 235;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623ALong) {
                        return 236;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623A) {
                        return 237;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623Strict) {
                        return 238;
                    }
                    else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                        return 239;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialN) {
                        return 240;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialS) {
                        return 241;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialHi) {
                        return 242;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialLw) {
                        return 243;
                    }
                    else {
                        return 0;
                    }
                }
                else if (attack_air_kind == hash40("attack_air_hi") || motion_kind == hash40("attack_air_hi")) {
                    if fighter.is_cat_flag(Cat1::AttackAirN) {
                        return 244;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirF) {
                        return 245;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirB) {
                        return 246;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirLw) {
                        return 247;
                    }
                    if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                        return 248;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                        return 249;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                        return 250;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                        return 251;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                        return 252;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                        return 253;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                        return 254;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                        return 255;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                        return 256;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623NB) {
                        return 257;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623BLong) {
                        return 258;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623ALong) {
                        return 259;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623A) {
                        return 260;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623Strict) {
                        return 261;
                    }
                    else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                        return 262;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialN) {
                        return 263;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialS) {
                        return 264;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialHi) {
                        return 265;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialLw) {
                        return 266;
                    }
                    else {
                        return 0;
                    }
                }
                else if (attack_air_kind == hash40("attack_air_lw") || motion_kind == hash40("attack_air_lw")) {
                    if fighter.is_cat_flag(Cat1::AttackAirN) {
                        return 267;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirF) {
                        return 268;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirB) {
                        return 269;
                    }
                    else if fighter.is_cat_flag(Cat1::AttackAirHi) {
                        return 270;
                    }
                    if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                        return 271;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                        return 272;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                        return 273;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                        return 274;
                    }
                    else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                        return 275;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                        return 276;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                        return 277;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                        return 278;
                    }
                    else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                        return 279;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623NB) {
                        return 280;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623BLong) {
                        return 281;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623ALong) {
                        return 282;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623A) {
                        return 283;
                    }
                    else if fighter.is_cat_flag(Cat4::Command623Strict) {
                        return 284;
                    }
                    else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                        return 285;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialN) {
                        return 286;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialS) {
                        return 287;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialHi) {
                        return 288;
                    }
                    else if fighter.is_cat_flag(Cat1::SpecialLw) {
                        return 289;
                    }
                    else {
                        return 0;
                    }
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
                if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 290;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 291;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 292;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 293;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 294;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 295;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 296;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 297;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 298;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 299;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 300;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 301;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 302;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 303;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 304;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 305;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 306;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 307;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 308;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 309;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 310;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 311;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 312;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 313;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 314;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 315;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 316;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 317;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 318;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 319;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 320;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 321;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 322;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 323;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 324;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 325;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 326;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 327;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 328;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 329;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 330;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 331;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 332;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 333;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 334;
                }
                else {
                    return 0;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                    return 335;
                }
                else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                    return 336;
                }
                else if fighter.is_cat_flag(Cat4::SpecialSCommand) {
                    return 337;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHiCommand) {
                    return 338;
                }
                else if fighter.is_cat_flag(Cat4::SpecialHi2Command) {
                    return 339;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialCommand) {
                    return 340;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecialRCommand) {
                    return 341;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2Command) {
                    return 342;
                }
                else if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand) {
                    return 343;
                }
                else if fighter.is_cat_flag(Cat4::Command623NB) {
                    return 344;
                }
                else if fighter.is_cat_flag(Cat4::Command623BLong) {
                    return 345;
                }
                else if fighter.is_cat_flag(Cat4::Command623ALong) {
                    return 346;
                }
                else if fighter.is_cat_flag(Cat4::Command623A) {
                    return 347;
                }
                else if fighter.is_cat_flag(Cat4::Command623Strict) {
                    return 348;
                }
                else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                    return 349;
                }
                else {
                    return 0;
                }
            }
        }
        return 0;
    }
}