#![allow(unused_macros)]
use {
	skyline::hooks::{
		InlineCtx,
		Region,
		getRegionAddress
    },
    smash::{
        app::{
			lua_bind::*,
			utility::*,
			*
		},
		hash40,
        lib::{
			L2CAgent,
			lua_const::*,
		},
		lua2cpp::L2CFighterCommon,
        phx::*,
    },
	smashline::*,
};

//Universal Variables
pub static mut AERIAL_KIND: [i32; 9] = [0; 9]; //Tracks a player's C-Stick aerial, used for correcting the return value of ControlModule::get_attack_air_kind
pub static mut ALREADY_BOUNCED: bool = false; //Tracks if the ball has bounced at least once since being thrown
pub static mut ASDI : f32 = 2.5; //15 for testing purposes, should be 3
pub static mut ASDI_START: [bool; 8] = [false; 8];
pub static mut B_CHECK: [bool; 9] = [false; 9]; //Tracks if a fighter used a certain special move in the air
pub static mut BALL_BOUNCED: Vector3f = Vector3f{x: 0.0, y: 0.0, z: 9999.0}; //Tracks stats about the volleyball to determine who to KO
pub static mut BALL_ID: u32 = 0; //The battle object ID of the ball itself
pub static mut BALL_OWNER: i32 = 9; //Which player will start with the ball
pub static mut BALL_VICTIMS: [i32; 4] = [9; 4]; //Which players are to be KOd
pub static mut CAN_ADD: [bool; 8] = [false; 8];
pub static mut CAN_CANCEL: [bool; 8] = [false; 8];
pub static mut CAN_CANCEL_TIMER: [i32; 8] = [0; 8];
pub const CMD_CAT1: i32 = 0x20; //u64
pub const CMD_CAT2: i32 = 0x21; //u64
pub static mut CURRENT_MOMENTUM: f32 = 1.0;
pub static mut CURRENT_MOMENTUM_SPECIALS: f32 = 7.0;
pub static mut DASH_GRAB_SPEED: [f32; 8] = [0.0; 8];
pub static mut DIR_MULT: f32 = 57.295776842880464966688235343549; //Very fun number that turns direction that spits out ControlModule::get_stick_dir(boma) as an angle in degrees
pub static mut FIGHTER_BOOL_1: [bool; 9] = [false; 9];
pub static mut FIGHTER_BOOL_2: [bool; 9] = [false; 9];
pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub const FIGHTER_KIND: i32 = 0x2;
pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut FIRST_BOUNCE: bool = false; //Allows the throwing player to bounce the ball on their own side once
pub static mut FIGHTER_SPECIAL_STATE: [bool; 8] = [false; 8];
//pub static mut FLOAT_OFFSET: usize = 0x4dedc0;
pub static mut FULL_SMASH_ATTACK: [bool; 8] = [false; 8];
pub static mut GOT_HIT: [i32; 9] = [0; 9]; //Tracks if a player got hit during One-Hit mode
pub static mut GROUND_VEL: f32 = 5.0;
pub static mut HIGH_SPAWN_POS: Vector3f = Vector3f{x: 0.0, y: 0.0, z: 1.0}; //Determines where to spawn the right net
pub static mut HITFLOW: [bool; 8] = [false; 8];
pub static mut HIT_PLAYER: i32 = -1; //Tracks which players need to be respawned
pub static mut HOLD_SHIELD: [bool; 8] = [false; 8];
pub static mut INT_OFFSET: usize = 0x4ded80;
pub static mut IS_WAVEDASH: [bool; 8] = [false; 8];
pub static mut ITEM_MANAGER_ADDR: usize = 0;
pub static mut JUMP_SPEED_RATIO: f32 = 3.0;
pub static mut JUMP_SPEED_MAX_MUL: f32 = 14.0;
pub const JUMP_SQUAT_FRAME: i32 = 0x0006;
pub static mut JUMPSQUAT_VELOCITY: f32 = 2.0;
pub static mut LAST_TO_HIT_BALL: usize = 9; //The last player to have hit the ball
pub static mut LOW_SPAWN_POS: Vector3f = Vector3f{x: 0.0, y: 0.0, z: 1.0}; //Determines where to spawn the left net
pub const PREV_STATUS_KIND: i32 = 0xA;
pub static mut RAR_LENIENCY: f32 = 6.0;
pub static mut READY_GO: [bool; 9] = [false; 9]; //Returns false for exactly one frame after is_ready_go becomes true, used to initiate certain events exactly once at the start of a match
pub static mut READY_GO_TIMER: i32 = 0; //Determines how many frames to suspend all players while respawning in Tennis and One-Hit modes
pub static mut SHIELD_SPECIAL: [bool; 8] = [false; 8];
pub const SITUATION_KIND: i32 = 0x16;
pub static mut SIZE0: [f32; 9] = [0.0; 9];
pub static mut SIZE1: [f32; 9] = [0.0; 9];
pub static mut SIZE2: [f32; 9] = [0.0; 9];
pub static mut SIZE3: [f32; 9] = [0.0; 9];
pub static mut SPAWN_POS: [Vector3f; 9] = [Vector3f{x: 0.0, y: 0.0, z: 0.0}; 9]; //Tracks what position to spawn the nets in in Basketball mode, and where to respawn players in Volleyball and One-Hit mode
pub static mut SPAWN_SIDE: [bool; 9] = [false; 9]; //Tracks if a player's spawn position was on the right or left
pub static mut SPECIAL_SMASH_BODY: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_GRAVITY: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_HEAD: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_RATE: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_SIZE: i32 = 0; //Checks which mode was selected in the "Size" slot
pub static mut SPECIAL_SMASH_STATUS: i32 = 0; //Etc.
pub static mut SPECIAL_ZOOM_GFX: [i32; 8] = [0; 8];
pub const STATUS_KIND: i32 = 0xB; //i32
pub static mut STOCK_COUNT: [u64; 9] = [3; 9]; //Keeps track of stock count (may not work outside of standard rules)
pub static TAP_MAX: i32 = 25;
pub static mut TAP_NUM : [i32; 8] = [6; 8];
pub static mut TAP_WAIT : [i32; 8] = [6; 8];
pub static mut TEMP_SPAWN_POS: [Vector3f; 9] = [Vector3f{x: 0.0, y: 0.0, z: 0.0}; 9]; //Used to randomize spawn pos in Volleyball mode
pub static mut TOTAL_FIGHTER: i32 = 1; //Tracks how many fighters are present
pub static mut USED_FS: [bool; 9] = [false; 9]; //Flags when you just used a Final Smash in Special Smash
pub static mut WAVEDASH_DONE: [bool; 8] = [false; 8];

//Bowser Variables
pub static mut KOOPA_EXCELLENT_SMASH: [bool; 8] = [false; 8];
pub static mut KOOPA_EXCELLENT_SMASH_GFX: [i32; 8] = [0; 8];
pub static mut KOOPA_GOOD_SMASH: [bool; 8] = [false; 8];
pub static mut KOOPA_GOOD_SMASH_GFX: [i32; 8] = [0; 8];
pub static mut KOOPA_GREAT_SMASH: [bool; 8] = [false; 8];
pub static mut KOOPA_GREAT_SMASH_GFX: [i32; 8] = [0; 8];
pub static mut KOOPA_OK_SMASH: [bool; 8] = [false; 8];
pub static mut KOOPA_OK_SMASH_GFX: [i32; 8] = [0; 8];

//Captain Falcon Variables
pub static mut BOOST_INSTALL_ACTIVE: [bool; 8] = [false; 8];
pub static mut BOOST_INSTALL_GFX_COUNTER: [i32; 8] = [0; 8];
pub static mut BOOST_INSTALL_MOTION_RATE: [f32; 8] = [1.0; 8];
pub static mut BOOST_INSTALL_TIMER: [i32; 8] = [0; 8];
pub static mut FALCON_PUNCH_HIT: [bool; 8] = [false; 8];
pub static mut FALCON_PUNCH_TURN_COUNT: [f32; 8] = [0.0; 8];
pub static mut KIRBY_FALCON_PUNCH_TURN_COUNT: [f32; 8] = [0.0; 8];
pub static mut HYPE_HIT: [bool; 8] = [false; 8];

//Dark Samus
pub static mut CHARGE_SHOT_TIMER: [i32; 8] = [0; 8];
pub static mut HAS_FIRE_CHARGE_SHOT: [bool; 8] = [false; 8];
pub static mut SAMUSD_CHECK_FLOAT: [i32; 8] = [0; 8];
pub static mut SAMUSD_FLOAT: [i32; 8] = [0; 8]; //Logs Float Time
pub static mut SAMUSD_FLOAT_GFX_COUNTER: [i32; 8] = [0; 8];
pub static mut SAMUSD_FLOAT_MAX: i32 = 120; //Frames a character can float (E.G. in frames, 120 = 2 seconds)
pub static mut SAMUSD_HAS_FLOAT: [bool; 8] = [false; 8];
pub static mut SAMUSD_START_FLOAT: [bool; 8] = [false; 8];
pub static mut SAMUSD_X: [f32; 8] = [0.0; 8]; //Logs Horizontal speed
pub static mut SAMUSD_X_ACCEL_MUL: f32 = 0.065; //Air Accel Mul
pub static mut SAMUSD_X_MAX: f32 = 0.95; //Max Horizontal movespeed
pub static mut SAMUSD_Y: [f32; 8] = [0.0; 8]; //Logs Vertical speed
pub static mut SAMUSD_Y_MAX: f32 = 0.95; //Max Vertical movespeed

//Donkey Kong Variables
pub static mut BARREL_ACTIVE: [bool; 8] = [false; 8];
pub static mut BARREL_TIMER: [i32; 8] = [0; 8];
pub static mut BARREL_DIRECTION: [i32; 8] = [0; 8];
pub static mut DONKEY_DASH_ATTACK_JUMP: [i32; 8] = [0; 8];
pub static mut DONKEY_DASH_ATTACK_POWER_DOWN: [bool; 8] = [false; 8];
pub static mut DONKEY_GIANT_PUNCH_STAGE: [i32; 8] = [0; 8];
pub static mut SPEED_X: [f32; 8] = [8.0; 8];
pub static mut SPEED_Y: [f32; 8] = [8.0; 8];

//Falco Variables
pub static mut REFLECTOR_KNOCKBACK: [i32; 8] = [100; 8];
pub static mut REFLECTOR_ANGLE: [u64; 8] = [60; 8];

//King K Rool Variables
pub static mut KROOL_HAS_UAIR: [bool; 8] = [false; 8];
pub static mut KROOL_UP_SPECIAL_CANCEL: [bool; 8] = [false; 8];

//Lucario Variables
pub static mut MEGA_EVOLVE: [bool; 8] = [false; 8];

//Lucina Variables
pub static mut DID_ASTRA_2_S: [bool; 8] = [false; 8];
pub static mut DID_ASTRA_5_HI: [bool; 8] = [false; 8];
pub static mut LANDING_HIT: [bool; 8] = [false; 8];
pub static mut LUCINA_GFX_COUNTER: [i32; 8] = [0; 8];
pub static mut USE_SWORDSMAN_DASH: [bool; 8] = [true; 8];
pub static mut USE_UP_SPECIAL: [bool; 8] = [false; 8];

//Mewtwo Variables
pub static mut CAN_CHLOEDASH: [bool; 8] = [false; 8];
pub static mut CHLOEDASH_OFF_GFX_TIMER: [i32; 8] = [0; 8];
pub static mut CHLOEDASH_ON_GFX_TIMER: [i32; 8] = [0; 8];
pub static mut CHLOEDASH_TIMER: [i32; 8] = [0; 8];
pub static mut CHLOEDASHING_ENABLED: [i32; 8] = [0; 8];
pub static mut SPEED_ADD: [bool; 8] = [false; 8];

//Mii Brawler Variables
pub static mut USE_ONSLAUGHT: [bool; 8] = [true; 8];

//Ness Variables
pub static mut OFFENSE_UP_ACTIVE: [bool; 8] = [false; 8];
pub static mut OFFENSE_UP_TIMER: [i32; 8] = [0; 8];
pub static mut OFFENSE_UP_GFX_COUNTER: [i32; 8] = [0; 8];
pub static mut PK_FLASH_TIMER: [i32; 8] = [0; 8];

//Ridley Variables
pub static mut POGO_GROUND_BOUNCE: [bool; 8] = [false; 8];
pub static mut POGO_OPPONENT_BOUNCE: [bool; 8] = [false; 8];
pub static mut RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT: [i32; 8] = [0; 8];
pub static mut RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV : [Vector2f; 8] = [Vector2f{x:0.0,y:0.0}; 8];

//Roy Variables
pub static mut ROY_GFX_COUNTER: [i32; 8] = [0; 8];

//Senator Armstrong Variables
pub static mut FIRE_PUNCH_TURN_COUNT: [f32; 8] = [0.0; 8];
pub static mut KIRBY_FIRE_PUNCH_TURN_COUNT: [f32; 8] = [0.0; 8];

//Snake Variables
pub static mut SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE : [bool; 8] = [false; 8];
pub static mut SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED : [bool; 8] = [false; 8];
pub static mut SNAKE_INT_ATTACK_S4_COMBO_COUNT : [i32; 8] = [0; 8];

//Sonic Variables
pub static mut FAIR_HIT: [bool; 8] = [false; 8];
pub static mut HOMING_ATTACK_HIT: [bool; 8] = [false; 8];
pub static mut PARRIED: [i32; 8] = [0; 8];
pub static mut PARRY_TIMER: [i32; 8] = [0; 8];
pub static mut SONIC_BOOST: [f32; 8] = [0.0; 8];
pub static mut SONIC_BOOST_GFX_COUNTER: [i32; 8] = [0; 8];
pub static mut SONIC_BOOST_SPEED: [f32; 8] = [0.0; 8];
pub static mut BOUNCE_BRACELET_POWER: [f32; 8] = [3.0; 8];

pub static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

/*
pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];
*/

//Gets a boma
pub unsafe fn get_boma(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
	let boma = smash::app::sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(entry_id));
	return boma;
}

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

//Used in Grabs/Throws
extern "C" {
    #[link_name = "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"]
    pub static FIGHTER_CUTIN_MANAGER: *mut smash::app::FighterCutInManager;
}

//Related to Param Edits
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

//Related to Param Edits
#[skyline::hook(offset=0x3f0028, inline)]
pub unsafe fn offset_dump(ctx: &InlineCtx) {
	let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
	println!("Function Offset: {:#X}", ctx.registers[8].x.as_ref() - text);
}

//Checks if you're hitting the floor
pub(crate) unsafe fn ray_check_pos(module_accessor: &mut smash::app::BattleObjectModuleAccessor, x_distance : f32, y_distance: f32, ignore_plat: bool) -> u64 {
	GroundModule::ray_check(module_accessor, &Vector2f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor)}, &Vector2f{x: x_distance, y: y_distance}, ignore_plat)
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

//Universal Fighter Reset
#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        BARREL_ACTIVE[entry_id] = false;
		BARREL_DIRECTION[entry_id] = 0;
        BARREL_TIMER[entry_id] = 0;
		BOOST_INSTALL_ACTIVE[entry_id] = false;
		BOOST_INSTALL_GFX_COUNTER[entry_id] = 0;
		BOOST_INSTALL_MOTION_RATE[entry_id] = 1.0;
		BOOST_INSTALL_TIMER[entry_id] = 0;
        CAN_CANCEL[entry_id] = false;
        CAN_CANCEL_TIMER[entry_id] = 0;
		DASH_GRAB_SPEED[entry_id] = 0.0;
		DID_ASTRA_2_S[entry_id] = false;
		DID_ASTRA_5_HI[entry_id] = false;
		DONKEY_DASH_ATTACK_JUMP[entry_id] = 0;
		DONKEY_DASH_ATTACK_POWER_DOWN[entry_id] = false;
		FALCON_PUNCH_HIT[entry_id] = false;
		FALCON_PUNCH_TURN_COUNT[entry_id] = 0.0;
		FIGHTER_SPECIAL_STATE[entry_id] = false;
		FIRE_PUNCH_TURN_COUNT[entry_id] = 0.0;
		FULL_SMASH_ATTACK[entry_id] = true;
		HITFLOW[entry_id] = false;
		HYPE_HIT[entry_id] = false;
		KIRBY_FALCON_PUNCH_TURN_COUNT[entry_id] = 0.0;
        KIRBY_FIRE_PUNCH_TURN_COUNT[entry_id] = 0.0;
		KROOL_HAS_UAIR[entry_id] = false;
        KROOL_UP_SPECIAL_CANCEL[entry_id] = false;
		KOOPA_EXCELLENT_SMASH[entry_id] = false;
		KOOPA_EXCELLENT_SMASH_GFX[entry_id] = 0;
		KOOPA_GOOD_SMASH[entry_id] = false;
		KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
		KOOPA_GREAT_SMASH[entry_id] = false;
		KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
		KOOPA_OK_SMASH[entry_id] = false;
		KOOPA_OK_SMASH_GFX[entry_id] = 0;
		LANDING_HIT[entry_id] = true;
		MEGA_EVOLVE[entry_id] = false;
		OFFENSE_UP_ACTIVE[entry_id] = false;
		OFFENSE_UP_GFX_COUNTER[entry_id] = 0;
		OFFENSE_UP_TIMER[entry_id] = 0;
		PK_FLASH_TIMER[entry_id] = 0;
		REFLECTOR_KNOCKBACK[entry_id] = 100;
		REFLECTOR_ANGLE[entry_id] = 60;
		SHIELD_SPECIAL[entry_id] = false;
		SPECIAL_ZOOM_GFX[entry_id] = 0;
		SPEED_X[entry_id] = 0.0;
		SPEED_Y[entry_id] = 0.0;
		USE_SWORDSMAN_DASH[entry_id] = true;
		USE_UP_SPECIAL[entry_id] = true;
    }
}

//Prevention of Moves in Air
#[skyline::hook(replace = StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, unk: bool) -> u64 {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if get_kind(boma) == *FIGHTER_KIND_MIIFIGHTER
	&& status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
	&& !USE_ONSLAUGHT[entry_id as usize] {
		return 0;
	}
	if get_kind(boma) == *FIGHTER_KIND_LUCINA {
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
		&& !USE_SWORDSMAN_DASH[entry_id as usize] {
			return 0;
		}
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
		&& !USE_UP_SPECIAL[entry_id as usize] {
			return 0;
		}
	}
	return original!()(boma, status_kind, unk);
}

//Param Adjustments
#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
	let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
	let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let ret = original!()(module_accessor, param_type, param_hash);
	let fighter_kind = get_kind(boma);
	if smash::app::utility::get_category(boma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
		return ret;
	}
	//PK Fire
	if fighter_kind == *FIGHTER_KIND_NESS {
		if param_hash == hash40("life") {
			if OFFENSE_UP_ACTIVE[entry_id] == true {
				return 60;
			}
			else {
				return ret;
			}
		}
		if param_hash == hash40("pillar_life") {
			if OFFENSE_UP_ACTIVE[entry_id] == true {
				return 1;
			}
			else {
				return ret;
			}
		}
		else {
			return ret;
		}
	}
	else {
		return ret;
	}
}

//Marth/Lucina Counter Transition
pub unsafe extern "C" fn special_lw_mot_helper(fighter: &mut L2CFighterCommon) {
    let cont = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    let mot;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_air_lw_hit");
        correct = *GROUND_CORRECT_KIND_AIR;
    }
    else {
        mot = Hash40::new("special_lw_hit");
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
    }
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(correct));
    if !cont {
        MotionModule::change_motion(fighter.module_accessor, mot, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, mot, -1.0, 1.0, 0.0);
    }
}

//Installation
pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
		/*
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
		*/
    }
	install_agent_resets!(fighter_reset);
	skyline::install_hook!(get_param_int_replace);
	skyline::install_hook!(change_status_hook);
	skyline::install_hook!(offset_dump);
}