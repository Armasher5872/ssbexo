#![allow(
	unused_macros,
	unused_mut,
    unused_parens
)]
use {
	crate::custom::momentumtransfer::momentum_install,
    smash::{
        app::{
			lua_bind::*,
			utility::*,
			*
		},
        lib::{
			L2CAgent,
			lua_const::*,
		},
		lua2cpp::L2CFighterCommon,
        phx::*,
    },
	smashline::*,
};

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

//Used in Grabs/Throws
extern "C" {
    #[link_name = "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"]
    pub static FIGHTER_CUTIN_MANAGER: *mut smash::app::FighterCutInManager;
}

//BomaExt, helps with various things
pub trait BomaExt {
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
	unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
	unsafe fn set_front_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn set_back_cliff_hangdata(&mut self, x: f32, y: f32);
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
}

//Frame Info, helps with a few things like Momentum Transfer
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

#[fighter_frame_callback]
pub fn momentum_transfer_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        //Installations
        if let Some(info) = FrameInfo::update_and_get(fighter) {
            if fighter_kind == *FIGHTER_KIND_ALL {
                momentum_install(fighter, &info);
            }
        }
        else {
            panic!("Could not get the FrameInfo for this fighter! Is this even a fighter?")
        }
    };
}

pub fn install() {
    install_agent_frame_callbacks!(momentum_transfer_frame);
}