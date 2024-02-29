//Most of this is credited to either HDR's Code Repository, or Championship Edition
use super::*;

//Checks what alt you are
pub unsafe fn get_player_number(module_accessor:  &mut smash::app::BattleObjectModuleAccessor) -> usize {
    let player_number;
    if smash::app::utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
        player_number = WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
    }
    else if get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    else {
        let mut owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        while get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER { //Keep checking the owner of the boma we're working with until we've hit a boma that belongs to a fighter
            owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        }
        player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    return player_number;
}

//Gets attacker number
pub unsafe fn get_attacker_number(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> usize {
	let attacker_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR) as usize;
	return attacker_number;
}

//Gets the boma
pub unsafe fn get_boma(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
	let boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(entry_id));
	return boma;
}

//Gets Article Boma
pub unsafe fn get_article_boma(boma: *mut BattleObjectModuleAccessor, article_type: skyline::libc::c_int) -> *mut BattleObjectModuleAccessor {
    let article = ArticleModule::get_article(boma, article_type);
    let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    return sv_battle_object::module_accessor(object_id);
}

//A variety of extern C functions mainly regarding custom game modes and other offsets in Main
extern "C" {
	#[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
	pub fn dead_range(lua_state: u64) -> Vector4f;

    #[link_name = "\u{1}_ZN3app17sv_camera_manager12camera_rangeEv"]
	pub fn camera_range() -> Vector4f;

    #[link_name = "\u{1}_ZN3app9curryshot15is_preview_modeEv"]
	pub fn is_preview_mode() -> bool;

    #[link_name = "\u{1}_ZN3app8lua_bind28PostureModule__rot_y_lr_implEPNS_26BattleObjectModuleAccessorE"]
	pub fn imported_rot_y_lr(boma: &mut BattleObjectModuleAccessor) -> f32;

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Luigi14delete_plungerERNS_7FighterEb"]
	pub fn delete_plunger(fighter: *mut smash::app::Fighter, param: bool) -> u64;

    pub fn change_version_string(arg: u64, string: *const c_char);
}

//Full Smash Attack Check
pub unsafe fn attack_4_hold(fighter: &mut L2CFighterCommon) {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if frame >= 59.0 {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    }
    physics!(fighter, MA_MSC_CMD_PHYSICS_STOP_CHARGE);
    fighter.pop_lua_stack(1);
}

//Used for GGST COUNTER!
pub unsafe fn estimate_frame(module_accessor: &mut smash::app::BattleObjectModuleAccessor, frame: f32) -> bool {
    let motion_frame = MotionModule::frame(module_accessor);
	if motion_frame >= frame && motion_frame < frame + 1.0 {
		return true;
	}
	else {
		return false;
	}
}

bitflags! {
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
    pub struct Cat4: i32 {
        const SpecialNCommand       = 0x1;
        const SpecialN2Command      = 0x2;
        const SpecialSCommand       = 0x4;
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
    Cat4(Cat4)
}

impl Into<CommandCat> for Cat1 {
    fn into(self) -> CommandCat {
        CommandCat::Cat1(self)
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
            Cat1::from_bits_unchecked(ControlModule::get_command_flag_cat(boma, 0))
        }
    }
}

impl Cat4 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe { 
            Cat4::from_bits_unchecked(ControlModule::get_command_flag_cat(boma, 3)) 
        }
    }
}

//BomaExt, helps with various things
pub trait BomaExt {
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
	unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
	unsafe fn set_front_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn set_back_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32;
    unsafe fn is_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn down_input(&mut self) -> bool;
    unsafe fn up_input(&mut self) -> bool;
    unsafe fn jump_cancel(&mut self) -> bool;
    unsafe fn gimmick_flash(&mut self);
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn dacsa_check(&mut self) -> i32;
    unsafe fn stick_x(&mut self) -> f32;
    unsafe fn stick_y(&mut self) -> f32;
    unsafe fn get_param_float(&mut self, obj: &str, field: &str) -> f32;
    unsafe fn is_item(&mut self) -> bool;
    unsafe fn status(&mut self) -> i32;
    unsafe fn magic_series(&mut self) -> i32;
    unsafe fn get_grabber_boma(&mut self) -> &mut BattleObjectModuleAccessor;
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
	/* 
	Sets the position of the front/red ledge-grab box (see [`set_center_cliff_hangdata`](BomaExt::set_center_cliff_hangdata) for more information) 
	
	# Arguments:
	
	* `x` - The x coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
	* `y` - The y coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
	*/
	unsafe fn set_front_cliff_hangdata(&mut self, x: f32, y: f32) {
		let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x530 / 4) = x;
        *ground_data.add(0x534 / 4) = y;
    }
	/* 
	Sets the position of the front/red ledge-grab box (see [`set_center_cliff_hangdata`](BomaExt::set_center_cliff_hangdata) for more information) 
	
	# Arguments:
	
	* `x` - The x coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
	* `y` - The y coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
	*/
    unsafe fn set_back_cliff_hangdata(&mut self, x: f32, y: f32) {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x540 / 4) = x;
        *ground_data.add(0x544 / 4) = y;
    }
    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32 {
        return StatusModule::change_status_request_from_script(self, kind, repeat) as i32;
    }
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat),
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
    unsafe fn up_input(&mut self) -> bool {
        let stick_y = ControlModule::get_stick_y(self);
        //Checks if you're holding down the control stick more than the tap jump threshold
        if stick_y >= 0.7 {
            return true;
        }
        //Checks if you flick the stick down more than 3 times but less than 20 times, or your stick is greater than or equal to 1.0
        if ((ControlModule::get_flick_y(self) >= 3 && ControlModule::get_flick_y(self) < 20) || stick_y >= 1.0) {
            return true;
        };
        return false;
    }
    unsafe fn jump_cancel(&mut self) -> bool {
        let fighter = crate::functions::util::get_fighter_common_from_accessor(self);
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        || (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_stick_y(fighter.module_accessor) >= 0.7)
        || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI)
        || fighter.is_cat_flag(Cat1::Jump)
        || fighter.is_cat_flag(Cat1::JumpButton)
        || (fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.sub_transition_group_check_ground_jump().get_bool())
        || fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
            //Checks situation kind
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                //Enables transition terms
                let transition = [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT];
                for x in 0..transition.len() {
                    WorkModule::enable_transition_term(self, transition[x]);
                    //If transition terms are enabled, return true
                    if WorkModule::is_enable_transition_term(self, transition[x]) == true {
                        return true;
                    }
                }
            }
            else {
                //Enables transition terms
                let transition = [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT];
                for x in 0..transition.len() {
                    WorkModule::enable_transition_term(self, transition[x]);
                    //If transition terms are enabled (And your jump count is less than your max), return true
                    if WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                        if WorkModule::is_enable_transition_term(self, transition[x]) == true {
                            return true;
                        }
                    }
                }
            }    
        }
        //If all conditions fail, return false
        return false;
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
    unsafe fn dacsa_check(&mut self) -> i32 {
        let fighter = crate::functions::util::get_fighter_common_from_accessor(self);
        let frame = MotionModule::frame(fighter.module_accessor);
        let f5 = [*FIGHTER_KIND_FOX, *FIGHTER_KIND_SONIC, *FIGHTER_KIND_LUIGI];
        let f6 = [*FIGHTER_KIND_PURIN, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_WARIO];
        let f7 = [*FIGHTER_KIND_DAISY, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_JACK, *FIGHTER_KIND_MARIO, *FIGHTER_KIND_MIIFIGHTER, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_PALUTENA, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_ZELDA];
        let f8 = [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_PITB, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_KEN, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_LUCARIO, *FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_PIT, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_RYU, *FIGHTER_KIND_TRAIL, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_SZEROSUIT];
        let f9 = [*FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_NESS, *FIGHTER_KIND_PIKMIN, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_REFLET, *FIGHTER_KIND_PICKEL, *FIGHTER_KIND_YOUNGLINK];
        let f10 = [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA, *FIGHTER_KIND_KIRBY, *FIGHTER_KIND_MIISWORDSMAN, *FIGHTER_KIND_ELIGHT, *FIGHTER_KIND_MURABITO];
        let f11 = [*FIGHTER_KIND_DUCKHUNT, *FIGHTER_KIND_GANON, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_SIMON, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_YOSHI];
        let f12 = [*FIGHTER_KIND_KOOPA, *FIGHTER_KIND_WOLF];
        let f13 = [*FIGHTER_KIND_KAMUI, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_SHULK];
        let f14 = [*FIGHTER_KIND_CHROM, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_ROY];
        let f15 = [*FIGHTER_KIND_EDGE];
        let f16 = [*FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_IKE, *FIGHTER_KIND_DEMON];
        let f18 = [*FIGHTER_KIND_EFLAME];
        let f21 = [*FIGHTER_KIND_LINK];
        let f22 = [*FIGHTER_KIND_BRAVE];
        let f23 = [*FIGHTER_KIND_DEDEDE];
        //DACUS/DACDS
        if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
        && ((f5.contains(&fighter.kind()) && frame <= 5.0) 
        || (f6.contains(&fighter.kind()) && frame <= 6.0)
        || (f7.contains(&fighter.kind()) && frame <= 7.0) 
        || (f8.contains(&fighter.kind()) && frame <= 8.0) 
        || (f9.contains(&fighter.kind()) && frame <= 9.0) 
        || (f10.contains(&fighter.kind()) && frame <= 10.0)
        || (f11.contains(&fighter.kind()) && frame <= 11.0)
        || (f12.contains(&fighter.kind()) && frame <= 12.0)
        || (f13.contains(&fighter.kind()) && frame <= 13.0)
        || (f14.contains(&fighter.kind()) && frame <= 14.0)
        || (f15.contains(&fighter.kind()) && frame <= 15.0)
        || (f16.contains(&fighter.kind()) && frame <= 16.0)
        || (f18.contains(&fighter.kind()) && frame <= 18.0)
        || (f21.contains(&fighter.kind()) && frame <= 21.0)
        || (f22.contains(&fighter.kind()) && frame <= 22.0)
        || (f23.contains(&fighter.kind()) && frame <= 23.0)) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) || WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START) {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 || (fighter.global_table[STICK_Y].get_f32() > 0.7 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                    return 1;
                }
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0 || (fighter.down_input() && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                    return 2;
                }
            }
        }
        return 0;
    }
    unsafe fn stick_x(&mut self) -> f32 {
        return ControlModule::get_stick_x(self);
    }
    unsafe fn stick_y(&mut self) -> f32 {
        return ControlModule::get_stick_y(self);
    }
    unsafe fn get_param_float(&mut self, obj: &str, field: &str) -> f32 {
        let obj = obj.into();
        let field = field.into();
        WorkModule::get_param_float(self, Hash40::new(obj).hash, Hash40::new(field).hash)
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
    unsafe fn get_grabber_boma(&mut self) -> &mut BattleObjectModuleAccessor {
        let opponent_id = LinkModule::get_parent_object_id(self, *LINK_NO_CAPTURE) as u32;
        let opponent_object = super::util::get_battle_object_from_id(opponent_id);
        &mut *(*opponent_object).module_accessor
    }
}

pub trait GetObjects {
    unsafe fn boma(&mut self) -> &'static mut BattleObjectModuleAccessor {
        Self::get_boma(self)
    }

    unsafe fn object(&mut self) -> &'static mut BattleObject {
        Self::get_object(self)
    }

    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor;
    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject;
}

impl GetObjects for smash::lib::L2CAgent {
    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        std::mem::transmute(this.module_accessor)
    }

    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject {
        std::mem::transmute(this.battle_object)
    }
}

impl GetObjects for BattleObject {
    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        std::mem::transmute(this.module_accessor)
    }

    unsafe fn get_object(_: &mut Self) -> &'static mut BattleObject {
        panic!("Gannot call GetObjects::get_object on BattleObject!")
    }
}

impl GetObjects for BattleObjectModuleAccessor {
    unsafe fn get_boma(_: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        panic!("Gannot call GetObjects::get_boma on BattleObjectModuleAccessor!")
    }

    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject {
        std::mem::transmute(super::util::get_battle_object_from_id(this.battle_object_id))
    }
}

//Frame Info, helps with a variety of things
pub struct FrameInfo {
	pub lua_state: u64,
	pub agent: *mut L2CAgent,
	pub boma: *mut smash::app::BattleObjectModuleAccessor,
	pub fighter_kind: i32,
	pub status_kind: i32,
	pub situation_kind: i32,
	pub motion_kind: smash::phx::Hash40,
	pub cur_frame: f32,
	pub frame: f32,
	pub cat: [i32; 4],
	pub facing: f32,
	pub stick_x: f32,
	pub stick_y: f32,
	pub id: usize,
	pub costume_slot_number: i32
}

impl FrameInfo {
	pub unsafe fn update_and_get(fighter: &mut L2CFighterCommon) -> Option<Self> {
		let lua_state = fighter.lua_state_agent;
		let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
		let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if !(0..8).contains(&id) {
			return None;
		}
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		let cat2 = ControlModule::get_command_flag_cat(boma, 1);
		let cat3 = ControlModule::get_command_flag_cat(boma, 2);
		let cat4 = ControlModule::get_command_flag_cat(boma, 3);
		let cur_frame = MotionModule::frame(boma);
		Some(Self {
			lua_state: lua_state,
			agent: fighter as *mut L2CFighterCommon as *mut L2CAgent,
			boma: boma as *mut smash::app::BattleObjectModuleAccessor,
			fighter_kind: smash::app::utility::get_kind(boma),
			status_kind: StatusModule::status_kind(boma),
			situation_kind: StatusModule::situation_kind(boma),
			motion_kind: Hash40::new_raw(MotionModule::motion_kind(boma)),
			cur_frame: MotionModule::frame(boma),
			frame: cur_frame + 1.0,
			cat: [cat1, cat2, cat3, cat4],
			facing: PostureModule::lr(boma),
			stick_x: ControlModule::get_stick_x(boma),
			stick_y: ControlModule::get_stick_y(boma),
			id: id,
			costume_slot_number: WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)
		})
	}
}

/// Checks if your attack input is strictly a Neutral Attack input.
#[inline(always)]
pub unsafe fn only_jabs(fighter: &mut L2CFighterCommon) -> bool {
    !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
    && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
    && fighter.global_table[CMD_CAT1].get_i32() & (
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 |
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 |
        *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH
    ) == 0
}

pub unsafe extern "C" fn should_use_special_n_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn should_use_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&situation_kind)
    || [*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind)
    || WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    }
    true.into()
}

/// Enum for the kinds of controls that are mapped
/// Can map any of these over any button
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InputKind {
    Attack = 0x0,
    Special = 0x1,
    Jump = 0x2,
    Guard = 0x3,
    Grab = 0x4,
    SmashAttack = 0x5,
    AppealHi = 0xA,
    AppealS = 0xB,
    AppealLw = 0xC,
    Unset = 0xD,
    JumpMini = 0x12, // this is ours :), also start at 0x12 to avoid masking errors
    FullHop = 0x13, // also custom, this one is for tilts!
}

/// 0x50 Byte struct containing the information for controller mappings
#[derive(Debug)]
#[repr(C)]
pub struct ControllerMapping {
    pub gc_l: InputKind,
    pub gc_r: InputKind,
    pub gc_z: InputKind,
    pub gc_dup: InputKind,
    pub gc_dlr: InputKind,
    pub gc_ddown: InputKind,
    pub gc_a: InputKind,
    pub gc_b: InputKind,
    pub gc_cstick: InputKind,
    pub gc_y: InputKind,
    pub gc_x: InputKind,
    pub gc_rumble: bool,
    pub gc_absmash: bool,
    pub gc_tapjump: bool,
    pub gc_sensitivity: u8,
    // 0xF
    pub pro_l: InputKind,
    pub pro_r: InputKind,
    pub pro_zl: InputKind,
    pub pro_zr: InputKind,
    pub pro_dup: InputKind,
    pub pro_dlr: InputKind,
    pub pro_ddown: InputKind,
    pub pro_a: InputKind,
    pub pro_b: InputKind,
    pub pro_cstick: InputKind,
    pub pro_x: InputKind,
    pub pro_y: InputKind,
    pub pro_rumble: bool,
    pub pro_absmash: bool,
    pub pro_tapjump: bool,
    pub pro_sensitivity: u8,
    // 0x1F
    pub joy_shoulder: InputKind,
    pub joy_zshoulder: InputKind,
    pub joy_sl: InputKind,
    pub joy_sr: InputKind,
    pub joy_up: InputKind,
    pub joy_right: InputKind,
    pub joy_left: InputKind,
    pub joy_down: InputKind,
    pub joy_rumble: bool,
    pub joy_absmash: bool,
    pub joy_tapjump: bool,
    pub joy_sensitivity: u8,
    // 0x2B
    pub _2b: u8,
    pub _2c: u8,
    pub _2d: u8,
    pub _2e: u8,
    pub _2f: u8,
    pub _30: u8,
    pub _31: u8,
    pub _32: u8,
    pub is_absmash: bool,
    pub _34: [u8; 0x1C]
}

/// Controller class used internally by the game
#[repr(C)]
pub struct Controller {
    pub vtable: *const u64,
    pub current_buttons: ButtonBitfield,
    pub previous_buttons: ButtonBitfield,
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub left_trigger: f32,
    pub _left_padding: u32,
    pub right_stick_x: f32,
    pub right_stick_y: f32,
    pub right_trigger: f32,
    pub _right_padding: u32,
    pub gyro: [f32; 4],
    pub button_timespan: AutorepeatInfo,
    pub lstick_timespan: AutorepeatInfo,
    pub rstick_timespan: AutorepeatInfo,
    pub just_down: ButtonBitfield,
    pub just_release: ButtonBitfield,
    pub autorepeat_keys: u32,
    pub autorepeat_threshold: u32,
    pub autorepeat_initial_press_threshold: u32,
    pub style: ControllerStyle,
    pub controller_id: u32,
    pub primary_controller_color1: u32,
    pub primary_controller_color2: u32,
    pub secondary_controller_color1: u32,
    pub secondary_controller_color2: u32,
    pub led_pattern: u8,
    pub button_autorepeat_initial_press: bool,
    pub lstick_autorepeat_initial_press: bool,
    pub rstick_autorepeat_initial_press: bool,
    pub is_valid_controller: bool,
    pub _xB9: [u8; 2],
    pub is_connected: bool,
    pub is_left_connected: bool,
    pub is_right_connected: bool,
    pub is_wired: bool,
    pub is_left_wired: bool,
    pub is_right_wired: bool,
    pub _xC1: [u8; 3],
    pub npad_number: u32,
    pub _xC8: [u8; 8]
}

/// Re-ordered bitfield the game uses for buttons
#[bitfield]
#[derive(Debug, Default, Copy, Clone)]
pub struct ButtonBitfield {
    pub dpad_up: bool,
    pub dpad_right: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub x: bool,
    pub a: bool,
    pub b: bool,
    pub y: bool,
    pub l: bool,
    pub r: bool,
    pub zl: bool,
    pub zr: bool,
    pub left_sl: bool,
    pub left_sr: bool,
    pub right_sl: bool,
    pub right_sr: bool,
    pub stick_l: bool,
    pub stick_r: bool,
    pub plus: bool,
    pub minus: bool,
    pub l_up: bool,
    pub l_right: bool,
    pub l_down: bool,
    pub l_left: bool,
    pub r_up: bool,
    pub r_right: bool,
    pub r_down: bool,
    pub r_left: bool,
    pub unused: B4
}

#[repr(C)]
pub struct AutorepeatInfo {
    field: [u8; 0x18]
}

/// Controller style declaring what kind of controller is being used
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u32)]
pub enum ControllerStyle {
    Handheld = 0x1,
    DualJoycon = 0x2,
    LeftJoycon = 0x3,
    RightJoycon = 0x4,
    ProController = 0x5,
    DebugPag = 0x6, // I assume
    GCController = 0x7
}

/// 8 byte struct containig all button inputs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MappedInputs {
    pub buttons: Buttons,
    pub lstick_x: i8,
    pub lstick_y: i8,
    pub rstick_x: i8,
    pub rstick_y: i8
}

//Used to deal with DK's Barrels
pub unsafe extern "C" fn donkey_barrel_bool(boma: *mut BattleObjectModuleAccessor) -> bool {
    let itemmanager = smash2::app::ItemManager::instance().unwrap();
    let barrel_count = smash2::app::ItemManager::get_num_of_ownered_item(itemmanager, (*boma).battle_object_id, smash2::app::ItemKind::Barrel);
    let timer = WorkModule::get_int(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
    if barrel_count == 0 && !WorkModule::is_flag(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE) && timer <= 0 {
        return true;
    }
    return false;
}

//Deals with Inkling's Squid
pub unsafe extern "C" fn inkling_generate_squid_helper(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let rate = MotionModule::rate(fighter.module_accessor);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion_kind), false, -1.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, frame);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, rate);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        }
    }
}

//Metaknight Galaxia Beam Functions
pub unsafe extern "C" fn is_galaxia(object_boma: *mut BattleObjectModuleAccessor) -> bool {
    if smash::app::utility::get_kind(&mut *object_boma) == *WEAPON_KIND_KOOPAJR_CANNONBALL {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
        if owner_kind == *FIGHTER_KIND_METAKNIGHT {
            return true;
        }
    }
    return false;
}

pub unsafe extern "C" fn should_remove_galaxia(weapon: &mut L2CWeaponCommon) -> bool {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL)
    || life <= 0 {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn should_remove_galaxia_on_hit(weapon: &mut L2CWeaponCommon) -> bool {
    if AttackModule::is_infliction_status(weapon.module_accessor, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT)
    || StopModule::is_stop(weapon.module_accessor)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK) {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn galaxia_beam_removal(weapon: &mut L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &NONE_VECTOR, 1.0, 0, -1, false, 0);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}

pub unsafe extern "C" fn galaxia_beam_hit_removal(weapon: &mut L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(weapon.module_accessor, Hash40::new("miiswordsman_hensoku_hit"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &NONE_VECTOR, 1.0, 0, -1, false, 0);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}

//Gets the article owner boma
pub unsafe fn get_owner_boma(weapon: &mut L2CAgentBase) -> *mut BattleObjectModuleAccessor {
    return &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
}

pub unsafe extern "C" fn empty_waza_customize() -> L2CValue {
    0.into()
}

//Shield Specials
pub unsafe extern "C" fn if_shield_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        if kind == *FIGHTER_KIND_NESS {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(),true.into());
            return true.into();
        }
        if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_PZENIGAME].contains(&kind) {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(),true.into());
            return true.into();
        }
    }
    return false.into();
}

//ASDI Check
pub unsafe extern "C" fn asdi_check(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[IS_STOP].get_bool() {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    };
}

//ASDI Function
pub unsafe extern "C" fn asdi_function(fighter: &mut L2CFighterCommon) {
    let mut pos = Vector3f {x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor), z: PostureModule::pos_z(fighter.module_accessor)}; // get current pos
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START) && !StopModule::is_damage(fighter.module_accessor) {
        let asdi_stick_x = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_x(fighter.module_accessor)} else {stick_x}; // get stick x length, uses cstick's value if cstick is on (for Double Stick ASDI)
        let asdi_stick_y = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_y(fighter.module_accessor)} else {stick_y}; // get stick y length, uses cstick's value if cstick is on (for Double Stick ASDI)
        let asdi = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("hit_stop_delay_auto_mul")); // get base asdi distance
        let asdi_x = asdi * asdi_stick_x*lr; // mul asdi stick_x by total asdi
        let asdi_y = asdi * asdi_stick_y; // mul asdi stick_y by total asdi
        pos.x += asdi_x; //add asdi_x to pos_x
        pos.y += asdi_y; //add asdi_y to pos_y
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    };
}

#[repr(C)]
pub struct CreateItemParam {
    pub founder_pos: Vector4f,
    pub item_pos: Vector4f,
    pub item_kind: smash::app::ItemKind,
    pub another_battle_object_id: u32,
    pub variation_kind: i32,
    pub lr_dir: f32,
    pub owner_id: u32,
    pub unk_20: u32,
    pub pokeball_or_assist_kind: i32,
    pub unk_0: u64,
    pub weird_flag: u64,
    pub unk_1_weird: u64,
    pub unk_approx_0: f32,
    pub unk_02: f32
}

pub struct FuseKind(i32);

impl FuseKind {
    pub const FUSE: i32 = 0;
    pub const REFUSE: i32 = 1;
}

pub struct FuseType(i32);

impl FuseType {
    pub const NORMAL: i32 = 0;
    pub const POWER: i32 = 1;
    pub const ELEMENTAL: i32 = 2;
}

pub unsafe extern "C" fn set_arrow_fuse_params(boma: *mut BattleObjectModuleAccessor, item_kind: i32, fuse_kind: i32, trait_type: i32) {
    if (![*ITEM_KIND_NONE, *ITEM_KIND_ASSIST, *ITEM_KIND_LINKARROW].contains(&item_kind) && ![*ITEM_TRAIT_FLAG_NONE, *ITEM_TRAIT_FLAG_SHOOT, *ITEM_TRAIT_FLAG_SWING].contains(&trait_type)) 
    || [*ITEM_KIND_BANANAGUN, *ITEM_KIND_FIREFLOWER].contains(&item_kind) {
        WorkModule::on_flag(boma, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    else {
        WorkModule::off_flag(boma, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    if WorkModule::is_flag(boma, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
        WorkModule::set_int(boma, item_kind, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        if fuse_kind == FuseKind::FUSE {
            if owner_kind == *FIGHTER_KIND_LINK {
                WorkModule::set_int(owner_boma, item_kind, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
            }
            else if owner_kind == *FIGHTER_KIND_KIRBY {
                WorkModule::set_int(owner_boma, item_kind, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
            }
            let item_id = ItemModule::get_have_item_id(owner_boma, 0) as i32;
            WorkModule::set_int(boma, item_id, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
        }
        else if fuse_kind == FuseKind::REFUSE {
            let pos_x = PostureModule::pos_x(boma);
            let pos_y = PostureModule::pos_y(boma);
            let pos_z = PostureModule::pos_z(boma);
            let mut params = CreateItemParam {
                founder_pos: Vector4f{x: pos_x, y: pos_y, z: pos_z, w: 0.0},
                item_pos: Vector4f{x: pos_x, y: pos_y, z: pos_z, w: 0.0},
                item_kind: smash::app::ItemKind(item_kind),
                another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                variation_kind: *ITEM_VARIATION_NONE,
                lr_dir: PostureModule::lr(boma),
                owner_id: (*(boma)).battle_object_id,
                unk_20: 20,
                pokeball_or_assist_kind: *ITEM_KIND_NONE,
                unk_0: 0,
                weird_flag: 0x633F800000,
                unk_1_weird: 1,
                unk_approx_0: 0.0,
                unk_02: 0.0
            };
            let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
            let battle_object = create_item(item_manager, &mut params, false, false, false);
            let item_boma = (*battle_object).module_accessor;
            if ![*ITEM_KIND_HEALBALL, *ITEM_KIND_CHEWING, *ITEM_KIND_BOOMERANG].contains(&item_kind) {
                StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_HAVE, false);
            }
            if item_kind == *ITEM_KIND_LINKBOMB {
                PostureModule::set_scale(item_boma, 1.35, false);
            }
            let item_id = (*(item_boma)).battle_object_id as i32;
            WorkModule::set_int(boma, item_id, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
                WorkModule::set_int(owner_boma, *ITEM_KIND_NONE, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            }
        }
        if item_kind == *ITEM_KIND_BOMBER {
            WorkModule::set_int(boma, FuseType::NORMAL, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(boma, *ITEM_BOMBER_STATUS_KIND_BORN2, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if [*ITEM_KIND_KILLER, *ITEM_KIND_BANANAGUN, *ITEM_KIND_DOLPHINBOMB].contains(&item_kind) {
            WorkModule::set_int(boma, FuseType::POWER, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(boma, *ITEM_STATUS_KIND_THROW, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_FIREFLOWER {
            WorkModule::set_int(boma, FuseType::ELEMENTAL, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(boma, *ITEM_STATUS_KIND_LOST, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_LINKBOMB {
            WorkModule::set_int(boma, FuseType::NORMAL, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(boma, *ITEM_STATUS_KIND_BORN, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else {
            WorkModule::set_int(boma, FuseType::NORMAL, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(boma, *ITEM_STATUS_KIND_THROW, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
    }
}

pub unsafe extern "C" fn set_elemental_fuse(weapon: &mut L2CFighterBase, element: i32, fuse_type: i32, end_status: i32) {
    let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_LINK {
        WorkModule::set_int(owner_boma, element, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    }
    else {
        WorkModule::set_int(owner_boma, element, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    }
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    let mut params = CreateItemParam {
        founder_pos: Vector4f{x: pos_x, y: pos_y, z: pos_z, w: 0.0},
        item_pos: Vector4f{x: pos_x, y: pos_y, z: pos_z, w: 0.0},
        item_kind: smash::app::ItemKind(element),
        another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
        variation_kind: *ITEM_VARIATION_NONE,
        lr_dir: PostureModule::lr(owner_boma),
        owner_id: (*(owner_boma)).battle_object_id,
        unk_20: 20,
        pokeball_or_assist_kind: *ITEM_KIND_NONE,
        unk_0: 0,
        weird_flag: 0x633F800000,
        unk_1_weird: 1,
        unk_approx_0: 0.0,
        unk_02: 0.0
    };
    let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
    let battle_object = create_item(item_manager, &mut params, false, false, false);
    WorkModule::set_int(weapon.module_accessor, element, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
    WorkModule::set_int(weapon.module_accessor, fuse_type, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
    WorkModule::set_int(weapon.module_accessor, end_status, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
    let item_boma = (*battle_object).module_accessor;
    let item_id = (*item_boma).battle_object_id;
    WorkModule::set_int64(weapon.module_accessor, item_id as i64, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_THROW, false);
    LinkModule::remove_model_constraint(item_boma, true);
    if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
        LinkModule::unlink(item_boma, *ITEM_LINK_NO_HAVE);
    }
    if !LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
        VisibilityModule::set_whole(item_boma, true);
        LinkModule::link(item_boma, *ITEM_LINK_NO_HAVE, (*(weapon.module_accessor)).battle_object_id);
        LinkModule::set_model_constraint_pos_ort(item_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("top"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, true);
    }
    WorkModule::on_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
}

pub unsafe extern "C" fn set_boomerang_fuse_params(boma: *mut BattleObjectModuleAccessor, item_kind: i32, fuse_kind: i32, trait_type: i32) {
    if (![*ITEM_KIND_NONE,*ITEM_KIND_ASSIST,*ITEM_KIND_LINKARROW].contains(&item_kind) && ![*ITEM_TRAIT_FLAG_NONE,*ITEM_TRAIT_FLAG_SHOOT,*ITEM_TRAIT_FLAG_SWING].contains(&trait_type))
    || [*ITEM_KIND_FIREFLOWER].contains(&item_kind) {
        WorkModule::on_flag(boma,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    else {
        WorkModule::off_flag(boma,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    if WorkModule::is_flag(boma,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        WorkModule::set_int(boma,item_kind,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
        if fuse_kind == FuseKind::FUSE {
            WorkModule::set_int(owner_boma,item_kind,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            let item_id = ItemModule::get_have_item_id(owner_boma,0) as i32;
            WorkModule::set_int(boma,item_id,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            WorkModule::set_int(owner_boma,item_id,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
        }
        else if fuse_kind == FuseKind::REFUSE {
            let mut params = CreateItemParam {
                founder_pos: Vector4f{x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma), w: 0.0},
                item_pos: Vector4f{x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma), w: 0.0},
                item_kind: smash::app::ItemKind(item_kind),
                another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                variation_kind: *ITEM_VARIATION_NONE,
                lr_dir: PostureModule::lr(boma),
                owner_id: (*(boma)).battle_object_id,
                unk_20: 20,
                pokeball_or_assist_kind: *ITEM_KIND_NONE,
                unk_0: 0,
                weird_flag: 0x633F800000,
                unk_1_weird: 1,
                unk_approx_0: 0.0,
                unk_02: 0.0
            };
            let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
            let battle_object = create_item(item_manager,&mut params,false,false,false);
            let item_boma = (*battle_object).module_accessor;
            if ![*ITEM_KIND_HEALBALL,*ITEM_KIND_CHEWING,*ITEM_KIND_BOOMERANG].contains(&item_kind) {
                StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_HAVE,false);
            }
            if item_kind == *ITEM_KIND_LINKBOMB {
                PostureModule::set_scale(item_boma,1.3,false);
            }
            let item_id = (*(item_boma)).battle_object_id as i32;
            WorkModule::set_int(boma,item_id,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            WorkModule::set_int(owner_boma,item_id,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID);
            if owner_kind == *FIGHTER_KIND_MURABITO
            || owner_kind == *FIGHTER_KIND_SHIZUE {
                WorkModule::set_int(owner_boma,*ITEM_KIND_NONE,FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
            }
        }
        if item_kind == *ITEM_KIND_BOMBER {
            WorkModule::set_int(boma,*ITEM_BOMBER_STATUS_KIND_BORN2,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_FIREFLOWER {
            WorkModule::set_int(boma,*ITEM_STATUS_KIND_LOST,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_LINKBOMB {
            WorkModule::set_int(boma,*ITEM_STATUS_KIND_BORN,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else {
            WorkModule::set_int(boma,*ITEM_STATUS_KIND_THROW,WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
    }
}

pub unsafe extern "C" fn ac_common(fighter: &mut L2CFighterCommon) {
    let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
        let obj_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
        let obj_boma = sv_battle_object::module_accessor(obj_id);
        let obj_kind = smash::app::utility::get_kind(&mut *obj_boma);
        let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(obj_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
        if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
            let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
            let item_id = WorkModule::get_int64(obj_boma, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            let fused_item = if owner_kind == *FIGHTER_KIND_KIRBY {
                WorkModule::get_int(owner_boma, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
            }
            else if [*FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MURABITO].contains(&owner_kind) {
                WorkModule::get_int(owner_boma, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM)
            }
            else if WorkModule::is_flag(obj_boma, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_DEDEDE_SWALLOW) {
                smash::app::utility::get_kind(&mut *item_boma)
            }
            else {
                WorkModule::get_int(owner_boma, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
            };
            WorkModule::set_int(fighter.module_accessor, fused_item, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            if sv_battle_object::is_active(item_id) {
                smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
            }
        }
        else if obj_kind == *WEAPON_KIND_LINK_BOOMERANG {
            let item_id = WorkModule::get_int(obj_boma, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            let fused_item = if StatusModule::status_kind(obj_boma) == *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED {
                smash::app::utility::get_kind(&mut *item_boma)
            }
            else if[*FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MURABITO].contains(&owner_kind) {
                WorkModule::get_int(owner_boma, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM)
            }
            else {
                WorkModule::get_int(owner_boma, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE)
            };
            WorkModule::set_int(fighter.module_accessor, fused_item, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            if sv_battle_object::is_active(item_id) {
                smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
            }
        }
    }
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG) {
        let boomerang_fuse_item_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(boomerang_fuse_item_id);
        if smash::app::sv_battle_object::is_active(boomerang_fuse_item_id) && StatusModule::status_kind(item_boma) == *ITEM_STATUS_KIND_HAVE {
            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, boomerang_fuse_item_id);
        }
    }
}

pub unsafe extern "C" fn find_ascendable_ground(boma: *mut BattleObjectModuleAccessor, pos_x: f32, min_pos_y: f32, pos_y: f32, height: f32) -> f32 {
    let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    if GroundModule::ray_check_hit_pos(boma, &smash::phx::Vector2f{x: pos_x, y: pos_y}, &Vector2f{x: 0.0, y: -100.0}, ground_hit_pos, true) {
        if ground_hit_pos.y < min_pos_y {
            return pos_y;
        }
        return find_ascendable_ground(boma, pos_x, min_pos_y, ground_hit_pos.y-height, height);
    }
    else {
        return pos_y;
    }
}