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

//Used for Blastzone Looping
extern "C" {
	#[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
	pub fn dead_range(lua_state: u64) -> Vector4f;
}

extern "C" {
	#[link_name = "\u{1}_ZN3app17sv_camera_manager12camera_rangeEv"]
	pub fn camera_range() -> Vector4f;
}

//Preview
extern "C" {
	#[link_name = "\u{1}_ZN3app9curryshot15is_preview_modeEv"]
	pub fn is_preview_mode() -> bool;
}

//Rotation Stuff
extern "C" {
	#[link_name = "\u{1}_ZN3app8lua_bind28PostureModule__rot_y_lr_implEPNS_26BattleObjectModuleAccessorE"]
	pub fn imported_rot_y_lr(boma: &mut BattleObjectModuleAccessor) -> f32;
}

//Luigi FighterSpecializer
extern "C" {
	#[link_name = "\u{1}_ZN3app24FighterSpecializer_Luigi14delete_plungerERNS_7FighterEb"]
	pub fn delete_plunger(instance: &mut smash::app::Fighter, param: bool) -> u64;
}

//Checks the version string
extern "C" {
	pub fn change_version_string(arg: u64, string: *const c_char);
}

//Full Smash Attack Check
pub unsafe fn attack_4_hold(fighter: &mut L2CFighterCommon) {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let cbm_vec1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 0.2};
    let cbm_vec2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.8};
    if frame > 59.0 {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    }
    if fighter_kind == *FIGHTER_KIND_NESS {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("ness_catch"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("ness_eye"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("ness_head"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("ness_talk"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("ness_patterna"), true);
        ColorBlendModule::set_main_color(fighter.module_accessor, /* Brightness */&cbm_vec1, /* Diffuse */&cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ness_black_face"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
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
        const SpecialN = 0x1000;
        const SpecialS = 0x2000;
        const SpecialHi = 0x4000;
        const SpecialLw = 0x8000;
        const TurnDash = 0x80000;
        const Jump = 0x100000;
        const JumpButton = 0x200000;
    }
    pub struct Cat4: i32 {
        const SpecialNCommand = 0x1;
        const SpecialN2Command = 0x2;
        const AttackCommand1 = 0x40;
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
    unsafe fn jump_cancel(&mut self) -> bool;
    unsafe fn gimmick_flash(&mut self);
    unsafe fn special_cancel(&mut self) -> i32;
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn dacsa_check(&mut self) -> i32;
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
    unsafe fn special_cancel(&mut self) -> i32 {
        let fighter = crate::functions::util::get_fighter_common_from_accessor(self);
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if fighter.is_cat_flag(Cat4::SpecialN2Command) {
                return 1;
            }
            else if fighter.is_cat_flag(Cat4::SpecialNCommand) {
                return 2;
            }
            else if fighter.is_cat_flag(Cat4::AttackCommand1) {
                return 3;
            }
            else if fighter.is_cat_flag(Cat1::SpecialN) {
                return 4;
            }
            else if fighter.is_cat_flag(Cat1::SpecialS) {
                return 5;
            }
            else if fighter.is_cat_flag(Cat1::SpecialHi) {
                return 6;
            }
            else if fighter.is_cat_flag(Cat1::SpecialLw) {
                return 7;
            }
            else {
                return 0;
            }
        }
        return 0;
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