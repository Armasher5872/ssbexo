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

//Statuses that can edge cancel
pub(crate) fn is_edge_cancel(fighter_kind : i32, status_kind : i32) -> bool {
	let edge_cancel = [
        [*FIGHTER_KIND_YOSHI, *FIGHTER_STATUS_KIND_SPECIAL_N],
        [*FIGHTER_KIND_YOSHI, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1],
        [*FIGHTER_KIND_YOSHI, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_2],
        [*FIGHTER_KIND_YOSHI, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_LW_LANDING],
        [*FIGHTER_KIND_FOX, *FIGHTER_STATUS_KIND_SPECIAL_S],
        [*FIGHTER_KIND_FOX, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH],
        [*FIGHTER_KIND_FOX, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END],
        [*FIGHTER_KIND_PIKACHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK],
        [*FIGHTER_KIND_PIKACHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK],
        [*FIGHTER_KIND_PIKACHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END],
        [*FIGHTER_KIND_PIKACHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP],
        [*FIGHTER_KIND_PIKACHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END],
        [*FIGHTER_KIND_LUIGI, *FIGHTER_STATUS_KIND_SPECIAL_S],
        [*FIGHTER_KIND_LUIGI, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM],
        [*FIGHTER_KIND_LUIGI, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END],
        [*FIGHTER_KIND_LUIGI, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL],
        [*FIGHTER_KIND_NESS, *FIGHTER_STATUS_KIND_SPECIAL_S],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END],
        [*FIGHTER_KIND_NESS, *FIGHTER_STATUS_KIND_SPECIAL_LW],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END],
        [*FIGHTER_KIND_CAPTAIN, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END],
        [*FIGHTER_KIND_SHEIK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE],
        [*FIGHTER_KIND_SHEIK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END],
        [*FIGHTER_KIND_SHEIK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK],
        [*FIGHTER_KIND_SHEIK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN],
        [*FIGHTER_KIND_SHEIK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING],
        [*FIGHTER_KIND_ZELDA, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2],
        [*FIGHTER_KIND_ZELDA, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3],
        [*FIGHTER_KIND_PICHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK],
        [*FIGHTER_KIND_PICHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK],
        [*FIGHTER_KIND_PICHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END],
        [*FIGHTER_KIND_PICHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP],
        [*FIGHTER_KIND_PICHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END],
        [*FIGHTER_KIND_FALCO, *FIGHTER_STATUS_KIND_SPECIAL_S],
        [*FIGHTER_KIND_FALCO, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH],
        [*FIGHTER_KIND_FALCO, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END],
        [*FIGHTER_KIND_YOUNGLINK, *FIGHTER_STATUS_KIND_SPECIAL_HI],
        [*FIGHTER_KIND_MEWTWO, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2],
        [*FIGHTER_KIND_MEWTWO, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3],
        [*FIGHTER_KIND_SZEROSUIT, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_FLIP],
        [*FIGHTER_KIND_SZEROSUIT, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK],
        [*FIGHTER_KIND_SZEROSUIT, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_LANDING],
        [*FIGHTER_KIND_SZEROSUIT, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK_LANDING],
        [*FIGHTER_KIND_PLIZARDON, *FIGHTER_STATUS_KIND_SPECIAL_S],
        [*FIGHTER_KIND_PLIZARDON, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_RUSH],
        [*FIGHTER_KIND_PLIZARDON, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END],
        [*FIGHTER_KIND_PLIZARDON, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_BLOWN],
        [*FIGHTER_KIND_DIDDY, *FIGHTER_STATUS_KIND_SPECIAL_HI],
        [*FIGHTER_KIND_LUCAS, *FIGHTER_STATUS_KIND_AIR_LASSO],
        [*FIGHTER_KIND_LUCAS, *FIGHTER_STATUS_KIND_SPECIAL_S],
        [*FIGHTER_KIND_LUCAS, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK],
        [*FIGHTER_KIND_LUCAS, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_AGAIN],
        [*FIGHTER_KIND_LUCAS, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_REFLECT],
        [*FIGHTER_KIND_LUCAS, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END],
        [*FIGHTER_KIND_SONIC, *FIGHTER_STATUS_KIND_SPECIAL_S],
        [*FIGHTER_KIND_SONIC, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD],
        [*FIGHTER_KIND_SONIC, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END],
        [*FIGHTER_KIND_SONIC, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN],
        [*FIGHTER_KIND_SONIC, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_REBOUND],
        [*FIGHTER_KIND_SONIC, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP],
        [*FIGHTER_KIND_LUCARIO, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH],
        [*FIGHTER_KIND_LUCARIO, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END],
        [*FIGHTER_KIND_TOONLINK, *FIGHTER_STATUS_KIND_SPECIAL_HI],
        [*FIGHTER_KIND_WOLF, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH],
        [*FIGHTER_KIND_WOLF, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END],
        [*FIGHTER_KIND_GEKKOUGA, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_LOOP],
        [*FIGHTER_KIND_PALUTENA, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2],
        [*FIGHTER_KIND_PALUTENA, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3],
        [*FIGHTER_KIND_KOOPAJR, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING],
        [*FIGHTER_KIND_RYU, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F],
        [*FIGHTER_KIND_RYU, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B],
        [*FIGHTER_KIND_RYU, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK],
        [*FIGHTER_KIND_KEN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F],
        [*FIGHTER_KIND_KEN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B],
        [*FIGHTER_KIND_KEN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK],
        [*FIGHTER_KIND_KAMUI, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK],
        [*FIGHTER_KIND_KAMUI, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK_END],
        [*FIGHTER_KIND_KAMUI, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B],
        [*FIGHTER_KIND_KAMUI, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F],
        [*FIGHTER_KIND_KAMUI, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING],
        [*FIGHTER_KIND_KAMUI, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING],
        [*FIGHTER_KIND_BAYONETTA, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D],
        [*FIGHTER_KIND_BAYONETTA, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING],
        [*FIGHTER_KIND_RIDLEY, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING],
        [*FIGHTER_KIND_GAOGAEN, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END],
        [*FIGHTER_KIND_GAOGAEN, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_BOUND],
        [*FIGHTER_KIND_PACKUN, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END],
        [*FIGHTER_KIND_DOLLY, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B],
        [*FIGHTER_KIND_DOLLY, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK],
        [*FIGHTER_KIND_DOLLY, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND],
        [*FIGHTER_KIND_DOLLY, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING],
        [*FIGHTER_KIND_DOLLY, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK],
        [*FIGHTER_KIND_DOLLY, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING],
        [*FIGHTER_KIND_DEMON, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP],
        [*FIGHTER_KIND_DEMON, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_AIR_END],
        [*FIGHTER_KIND_MIIFIGHTER, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START],
        [*FIGHTER_KIND_MIIFIGHTER, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK],
        [*FIGHTER_KIND_MIIFIGHTER, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_LANDING],
        [*FIGHTER_KIND_MIIFIGHTER, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING],
        [*FIGHTER_KIND_MIISWORDSMAN, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_END],
        [*FIGHTER_KIND_MIISWORDSMAN, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END]
    ];
	for i in &edge_cancel {
		if fighter_kind == i[0] && status_kind == i[1] {
			return true;
		};
	};
	return false;
}

bitflags! {
    pub struct Cat1: i32 {
        const TurnDash = 0x80000;
        const Jump = 0x100000;
        const JumpButton = 0x200000;
    }
}

#[derive(Copy, Clone)]
pub enum CommandCat {
    Cat1(Cat1)
}

impl Into<CommandCat> for Cat1 {
    fn into(self) -> CommandCat {
        CommandCat::Cat1(self)
    }
}

impl Cat1 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe {
            Cat1::from_bits_unchecked(ControlModule::get_command_flag_cat(boma, 0))
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
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat)
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