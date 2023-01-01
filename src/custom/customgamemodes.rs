//Credit to Championship Edition and VinegarChicken for the source code
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_unsafe)]
use {
    crate::functions::{
        ALL_FIGHTERS_LAST_STOCK,
        ALREADY_BOUNCED,
        BALL_BOUNCED,
        BALL_ID,
        BALL_OWNER,
        BALL_VICTIMS,
        CMD_CAT2,
        FIGHTER_CUTIN_MANAGER_ADDR,
        FIGHTER_MANAGER_ADDR,
        FIRST_BOUNCE,
        GOT_HIT,
        HIGH_SPAWN_POS,
        HIT_PLAYER,
        ITEM_MANAGER_ADDR,
        LAST_TO_HIT_BALL,
        LOW_SPAWN_POS,
        READY_GO,
        READY_GO_TIMER,
        SPAWN_POS,
        SPAWN_SIDE,
        SPECIAL_SMASH_BODY,
        SPECIAL_SMASH_GRAVITY,
        SPECIAL_SMASH_HEAD,
        SPECIAL_SMASH_RATE,
        SPECIAL_SMASH_SIZE,
        SPECIAL_SMASH_STATUS,
        STATUS_KIND,
        STOCK_COUNT,
        TEMP_SPAWN_POS,
        TOTAL_FIGHTER,
        USED_FS,
        camera_range,
        dead_range,
        get_boma,
        get_player_number,
        imported_rot_y_lr,
        is_preview_mode,
        offset_dump
    },
    skyline::{
        nn::ro::LookupSymbol,
        from_c_str
    },
    smash::{
        app::{
            BattleObjectModuleAccessor,
            lua_bind::*,
            self,
            smashball,
            sv_animcmd::*,
            sv_battle_object::*,
            sv_information,
            sv_math,
            *
        },
        crc32::*,
        hash40,
        lib::{
            L2CAgent,
            L2CInnerFunctionBase,
            L2CTable,
            L2CTable_meta,
            L2CValue,
            L2CValueInner,
            L2CValueType,
            L2CValueType::*,
            lua_const::{
                self,
                *
            },
			*
        },
        lua2cpp::{
			L2CFighterCommon,
			L2CAgentBase,
            self,
            *
        },
        phx::{
			Hash40,
			*
		}
    },
    smashline::*,
    smash_script::*,
    std::{
        ffi::{
            CStr,
            CString
        },
        mem,
        os::raw::c_char,
        string::String
    }
};

pub unsafe fn is_damage_check(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    if SPECIAL_SMASH_HEAD == 1 {
        if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI) || [
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_THROWN,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_FINAL,
            *FIGHTER_STATUS_KIND_SLEEP,
            *FIGHTER_STATUS_KIND_ESCAPE_B,
            *FIGHTER_STATUS_KIND_ESCAPE_F,
            *FIGHTER_STATUS_KIND_ESCAPE,
            *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
            *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND,
            *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD,
            *FIGHTER_STATUS_KIND_SWALLOWED,
            *FIGHTER_STATUS_KIND_AIR_LASSO,
            *FIGHTER_STATUS_KIND_CATCHED_REFLET,
            *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_BURY,
            *FIGHTER_STATUS_KIND_BURY_WAIT,
            *FIGHTER_STATUS_KIND_ICE,
            *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
            *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
            *FIGHTER_STATUS_KIND_DOWN_STAND,
            *FIGHTER_STATUS_KIND_DOWN_WAIT,
            *FIGHTER_STATUS_KIND_DOWN_EAT,
            *FIGHTER_STATUS_KIND_LAY_DOWN,
            *FIGHTER_STATUS_KIND_DOWN,
            *FIGHTER_STATUS_KIND_DOWN_SPOT,
            *FIGHTER_STATUS_KIND_PASSIVE,
            *FIGHTER_STATUS_KIND_PASSIVE_WALL,
            *FIGHTER_STATUS_KIND_PASSIVE_CEIL,
            *FIGHTER_STATUS_KIND_PASSIVE_FB                        //Extra checking idk 
        ].contains(&StatusModule::status_kind(module_accessor)) || FighterStopModuleImpl::is_damage_stop(module_accessor) {
            return true;
        }
        else {
            return false;
        }
    }
    else {
        return false;
    }   
}

pub unsafe fn enable_tilts_force(module_accessor: &mut BattleObjectModuleAccessor) {
    if SPECIAL_SMASH_HEAD == 1 {
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE\
          9instance_E\u{0}"
                .as_bytes()
                .as_ptr(), );
        let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
        }   
    }
}

pub unsafe fn enable_specials_force(module_accessor: &mut BattleObjectModuleAccessor) {
    if SPECIAL_SMASH_HEAD == 1 {
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE\
          9instance_E\u{0}"
                .as_bytes()
                .as_ptr(), );
        let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 && !is_damage_check(module_accessor) && !smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 && !is_damage_check(module_accessor) && !smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 && !is_damage_check(module_accessor) && !smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && !is_damage_check(module_accessor)  && ![*FIGHTER_KIND_PTRAINER, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_PIKMIN].contains(&smash::app::utility::get_kind(module_accessor)) && !smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
        }   
    }
}

pub unsafe fn enable_smash_atk_force(module_accessor: &mut BattleObjectModuleAccessor) {
    if SPECIAL_SMASH_HEAD == 1 {
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE\
          9instance_E\u{0}"
                .as_bytes()
                .as_ptr(), );
        let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4, true);
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);   WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
        }
    }
}

pub unsafe fn enable_all(module_accessor: &mut BattleObjectModuleAccessor) {
    if SPECIAL_SMASH_HEAD == 1 {
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
        WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    }
}

unsafe fn one_hit_mode(module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
	if READY_GO_TIMER != 0 
    && get_player_number(module_accessor) == 0 {
		READY_GO_TIMER -= 1;
	}
    //When someone gets hit, start a timer
	if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_SLIP, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
		GOT_HIT[get_player_number(module_accessor)] += 1;			
	} 
	else {
		GOT_HIT[get_player_number(module_accessor)] = 0;
	}
	if GOT_HIT[get_player_number(module_accessor)] == 2 { //If the timer reaches 2, check to see if anyone else has been hit
		for i in 0..TOTAL_FIGHTER + 1 {
			if GOT_HIT[i as usize] != 0 {
				if i as usize != get_player_number(module_accessor) {
					GOT_HIT[i as usize] = 3;
					break; //If anyone has, don't do anything
				}
			}
            //If no one has been hit, kill the one player who has and tell everyone else to reset their positions
			else if i == TOTAL_FIGHTER {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
			}
		}
	}
	if HIT_PLAYER != -1 
    && HIT_PLAYER != get_player_number(module_accessor) as i32 {
		DamageModule::add_damage(module_accessor, DamageModule::damage(module_accessor, 0) * -1.0, 0); //Reset opponents to 0% if they were hit by something like a fox laser
	}
	if READY_GO_TIMER != 0 {
		let spawn_vec = Vector2f{x: SPAWN_POS[get_player_number(module_accessor)].x, y: SPAWN_POS[get_player_number(module_accessor)].y};
		let correct_kind = smash::app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
		GroundModule::set_correct(module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
		PostureModule::set_pos_2d(module_accessor, &spawn_vec);
		StatusModule::set_situation_kind(module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), false);
		if status_kind != *FIGHTER_STATUS_KIND_WAIT 
        && STOCK_COUNT[get_player_number(module_accessor)] != 0 {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
		}
		PostureModule::set_lr(module_accessor, SPAWN_POS[get_player_number(module_accessor)].z);
		GroundModule::set_correct(module_accessor, correct_kind);
		PostureModule::update_rot_y_lr(module_accessor);
	}
	if status_kind == *FIGHTER_STATUS_KIND_DEAD {
		HIT_PLAYER = get_player_number(module_accessor) as i32;
	}
	if StatusModule::prev_status_kind(module_accessor, 1) == *FIGHTER_STATUS_KIND_DEAD {
		if STOCK_COUNT[get_player_number(module_accessor)] != 0 {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
		}
		READY_GO_TIMER = 181;		
		HIT_PLAYER = -1;
	}
}

unsafe fn tennis_mode(module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, lua_state: u64) {
	let fighter_kind = smash::app::utility::get_kind(module_accessor);
	if fighter_kind != *FIGHTER_KIND_NANA {
		if READY_GO_TIMER != 0 
        && get_player_number(module_accessor) == 0 {
			READY_GO_TIMER -= 1;
		}
		//Disable blast zones
		let mut pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
		if pos.x < dead_range(lua_state).x 
        || pos.x > dead_range(lua_state).y 
        || pos.y > dead_range(lua_state).z 
        || pos.y < dead_range(lua_state).w {
			if pos.x < dead_range(lua_state).x { //Right
				pos.x = dead_range(lua_state).y;
			}
			if pos.x > dead_range(lua_state).y { //Left
				pos.x = dead_range(lua_state).x;
			}
			if pos.y > dead_range(lua_state).z { //Up
				pos.y = dead_range(lua_state).w;
			}
			if pos.y < dead_range(lua_state).w { //Down
				pos.y = dead_range(lua_state).z;
			} 
			let correct_kind = smash::app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
			GroundModule::set_correct(module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
			PostureModule::set_pos(module_accessor, &pos);
			GroundModule::set_correct(module_accessor, correct_kind);
		}
		AttackModule::set_no_dead_all(module_accessor, true, false);
		WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
		//Invisible wall preventing fighters from entering each others' territory
		if SPAWN_SIDE[get_player_number(module_accessor)] 
        && PostureModule::pos_x(module_accessor) < 3.0 {
			let new_vec = Vector3f{x: 3.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
			PostureModule::set_pos(module_accessor, &new_vec);
		}
		else if !SPAWN_SIDE[get_player_number(module_accessor)] 
        && PostureModule::pos_x(module_accessor) > -3.0 {
			let new_vec = Vector3f{x: -3.0, y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
			PostureModule::set_pos(module_accessor, &new_vec);
		}
		//The process of KOing an opponent
		if BALL_BOUNCED.z != 9999.0 {
			if BALL_BOUNCED.y == 0.0 { //If the ball bounced on the ground, KO everyone who was on the same side as the ball
				if (BALL_BOUNCED.x > 3.0 
                    && SPAWN_SIDE[get_player_number(module_accessor)]) 
                    || (BALL_BOUNCED.x < -3.0 
                    && !SPAWN_SIDE[get_player_number(module_accessor)]) {
					for i in 0..3 {
						if BALL_VICTIMS[i] == 9 {
							BALL_VICTIMS[i] = get_player_number(module_accessor) as i32;
							break;
						}
					}
				}	
			}
			else { //If the ball went out of bounds, KO everyone who was on the same side as whoever last hit the ball
				if SPAWN_SIDE[get_player_number(module_accessor)] == SPAWN_SIDE[LAST_TO_HIT_BALL] {
					for i in 0..3 {
						if BALL_VICTIMS[i] == 9 {
							BALL_VICTIMS[i] = get_player_number(module_accessor) as i32;
							break;
						}
					}
				}
			}
			if get_player_number(module_accessor) as i32 == TOTAL_FIGHTER - 1 { //Once all victims have gotten a chance to be put on the list, KO them
				for i in 0..3 {
					if BALL_VICTIMS[i] != 9 {
						StatusModule::change_status_request_from_script(&mut *get_boma(BALL_VICTIMS[i]), *FIGHTER_STATUS_KIND_DEAD, true);
					}
				}
				LookupSymbol(
					&mut ITEM_MANAGER_ADDR,
					"_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E\u{0}"
						.as_bytes()
						.as_ptr(),
				);
				let item_manager = *(ITEM_MANAGER_ADDR as *mut *mut smash::app::ItemManager);
				smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, BALL_ID); //Then get rid of the soccerball and reset these values so multiple stocks aren't taken
				BALL_BOUNCED = Vector3f{x: 0.0, y: 0.0, z: 9999.0};
			}
		}
		if !READY_GO[get_player_number(module_accessor)] { //The frame the game starts, randomly pick a player to give the ball
			READY_GO_TIMER = 0;
			if get_player_number(module_accessor) == 0 {
				BALL_OWNER = smash::app::sv_math::rand(hash40("fighter"), TOTAL_FIGHTER);
			}
			if get_player_number(module_accessor) as i32 == BALL_OWNER {
				ItemModule::have_item(module_accessor, smash::app::ItemKind(*ITEM_KIND_SOCCERBALL), 0, 0, false, false);
				BALL_ID = ItemModule::get_have_item_id(module_accessor, 0) as u32;
			}
			LAST_TO_HIT_BALL = 9;
			ALREADY_BOUNCED = false;
			FIRST_BOUNCE = false;
		}
		if READY_GO_TIMER != 0 { //Lock everyone in place while waiting
			if READY_GO_TIMER == 180 { 
				let mut new_pos = SPAWN_POS[smash::app::sv_math::rand(hash40("fighter"), TOTAL_FIGHTER) as usize]; //Give each player a temporary spawn position that matches one of the previously used spawn positions, and doesn't match
				//any of the other players' temporary spawn positions
				let mut pos_shuffle = false;
				while pos_shuffle == false {
					pos_shuffle = true; //Allow the loop to end, then compare new_pos with the randomly selected positions of all previous fighters. If there's a match, pick a new position and force the loop to restart
					for i in 0..TOTAL_FIGHTER {
						if new_pos.x == TEMP_SPAWN_POS[i as usize].x 
                        && new_pos.y == TEMP_SPAWN_POS[i as usize].y 
                        && new_pos.z == TEMP_SPAWN_POS[i as usize].z {
							new_pos = SPAWN_POS[smash::app::sv_math::rand(hash40("fighter"), TOTAL_FIGHTER) as usize];
							pos_shuffle = false;
						}
					}
				}
				TEMP_SPAWN_POS[get_player_number(module_accessor)] = new_pos;
			}
			if READY_GO_TIMER == 179 {
				SPAWN_POS[get_player_number(module_accessor)] = TEMP_SPAWN_POS[get_player_number(module_accessor)];
				TEMP_SPAWN_POS[get_player_number(module_accessor)] = Vector3f{x: 0.0, y: 0.0, z: 0.0};
			}
			let spawn_vec = Vector2f{x: SPAWN_POS[get_player_number(module_accessor)].x, y: SPAWN_POS[get_player_number(module_accessor)].y};
			let correct_kind = smash::app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
			GroundModule::set_correct(module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
			PostureModule::set_pos_2d(module_accessor, &spawn_vec);
			StatusModule::set_situation_kind(module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), false);
			if status_kind != *FIGHTER_STATUS_KIND_WAIT 
            && STOCK_COUNT[get_player_number(module_accessor)] != 0 {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			}
			SPAWN_SIDE[get_player_number(module_accessor)] = PostureModule::pos_x(module_accessor) > 0.0;
			PostureModule::set_lr(module_accessor, SPAWN_POS[get_player_number(module_accessor)].z);
			GroundModule::set_correct(module_accessor, correct_kind);
			PostureModule::update_rot_y_lr(module_accessor);
			LAST_TO_HIT_BALL = 9;
			if READY_GO_TIMER == 1 { //After everyone has respawned, randomly choose a player who was just KOd and give them the ball
				if get_player_number(module_accessor) == 0 {
					BALL_OWNER = 9;
					while BALL_OWNER == 9 {
						let owner = BALL_VICTIMS[smash::app::sv_math::rand(hash40("fighter"), 4) as usize];
						if owner != 9 && STOCK_COUNT[owner as usize] != 0 {
							BALL_OWNER = owner;
						}
					}
					BALL_VICTIMS = [9, 9, 9, 9];
				}
				if get_player_number(module_accessor) as i32 == BALL_OWNER {
					ItemModule::have_item(module_accessor, smash::app::ItemKind(*ITEM_KIND_SOCCERBALL), 0, 0, false, false);
					BALL_ID = ItemModule::get_have_item_id(module_accessor, 0) as u32;
				}
			}
			ALREADY_BOUNCED = false;
			FIRST_BOUNCE = false;
		}
		if get_player_number(module_accessor) as i32 == BALL_OWNER {
			if ItemModule::is_have_item(module_accessor, 0) {
				LAST_TO_HIT_BALL = get_player_number(module_accessor);
			}
		}
		if StatusModule::prev_status_kind(module_accessor, 1) == *FIGHTER_STATUS_KIND_DEAD {
			if STOCK_COUNT[get_player_number(module_accessor)] != 0 {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			}
			READY_GO_TIMER = 181;
		}
	}
}

unsafe fn fun_di_mode(module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter: &mut L2CFighterCommon) {
	if [*FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
		let hitstun = WorkModule::get_int(module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_FRAME) - WorkModule::get_int(module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME);
		if hitstun == 0 {
			L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
		}
	}
}

unsafe fn smash4_parry(module_accessor: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, status_kind: i32, fighter: &mut L2CFighterCommon, globals: &mut L2CValue) {
	if let L2CValueType::Void = globals["perfect_shield"].val_type {
		globals["perfect_shield"] = false.into();
		globals["shield_health"] = 50.0.into();
	}
	if status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE 
    && StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_STATUS_KIND_GUARD {
		if !globals["perfect_shield"].get_bool() {
            fighter.clear_lua_stack();
            lua_args!(fighter, smash::app::FighterUtil::get_just_shield_se(fighter_kind));
            smash::app::sv_animcmd::PLAY_SE(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
		}
		globals["perfect_shield"] = true.into();
		WorkModule::mul_float(module_accessor, 0.66, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_SETOFF_MUL);
		WorkModule::set_float(module_accessor, globals["shield_health"].get_num(), *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
	}
	else {
		globals["shield_health"] = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD).into();
	}
	if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF {
		if globals["perfect_shield"].get_bool() {
			CancelModule::enable_cancel(module_accessor);
		}
		globals["perfect_shield"] = false.into();
	}
}

//Reverse Knockback
#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
	if SPECIAL_SMASH_STATUS == 1 {
		l2c_agent.clear_lua_stack();
		for i in 0..36 {
			if i == 4 {
				let new_damage = i+180;
				l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_damage));
			}
			else {
				l2c_agent.push_lua_stack(&mut hitbox_params[i as usize].clone());
			}
		}
		original!()(lua_state);
	}
	original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::ATTACK_ABS)]
unsafe fn attack_abs_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..15).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    if SPECIAL_SMASH_STATUS == 1 {
		l2c_agent.clear_lua_stack();
		for i in 0..15 {
			if i == 3 {
				let new_damage = i+180;
				l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_damage));
			}
			else {
				l2c_agent.push_lua_stack(&mut hitbox_params[i as usize].clone());
			}
		}
		original!()(lua_state);
	}
    original!()(lua_state);
}

unsafe fn basketball_mode(module_accessor: &mut smash::app::BattleObjectModuleAccessor, lua_state: u64, fighter: &mut L2CFighterCommon) {
	let mut pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
	if pos.x < dead_range(lua_state).x 
    || pos.x > dead_range(lua_state).y 
    || pos.y > dead_range(lua_state).z 
    || pos.y < dead_range(lua_state).w {
		if pos.x < dead_range(lua_state).x { //Right
			pos.x = dead_range(lua_state).y;
		}
		if pos.x > dead_range(lua_state).y { //Left
			pos.x = dead_range(lua_state).x;
		}
		if pos.y > dead_range(lua_state).z { //Up
			pos.y = dead_range(lua_state).w;
		}
		if pos.y < dead_range(lua_state).w { //Down
			pos.y = dead_range(lua_state).z;
		} 
		let correct_kind = smash::app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
		GroundModule::set_correct(module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
		PostureModule::set_pos(module_accessor, &pos);
		GroundModule::set_correct(module_accessor, correct_kind);
	}
	AttackModule::set_no_dead_all(module_accessor, true, false);
	WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
	if get_player_number(module_accessor) == 0 {
		if EffectModule::is_exist_common(module_accessor, Hash40{hash: hash40("sys_jump_aerial")}) == false {
			let high_offset = Vector3f{x: HIGH_SPAWN_POS.x - PostureModule::pos_x(module_accessor), y: HIGH_SPAWN_POS.y - PostureModule::pos_y(module_accessor), z: 0.0};
			let low_offset = Vector3f{x: LOW_SPAWN_POS.x - PostureModule::pos_x(module_accessor), y: LOW_SPAWN_POS.y - PostureModule::pos_y(module_accessor), z: 0.0};
			let lr = imported_rot_y_lr(module_accessor) / 90.0;
			macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("trans"), 0, high_offset.y, high_offset.x * lr, 0, 0, 90, 2.0, 0, 0, 0, 0, 0, 0, true);
			macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("trans"), 0, low_offset.y, low_offset.x * lr, 0, 0, 90, 2.0, 0, 0, 0, 0, 0, 0, true);
		}	
	}
}

unsafe fn status_kind_damage(module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, globals: &mut L2CValue) {
	if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_TREAD_DAMAGE].contains(&status_kind) {
		if status_kind != *FIGHTER_STATUS_KIND_TREAD_DAMAGE {
            //Fast Fall during Tumble
			if [*FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
				if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 
                    && (ControlModule::get_stick_y(module_accessor) < -0.66) 
                    && (KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= -0.5) {
						WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					}
				}
			}
			if SPECIAL_SMASH_HEAD == 2 {
				CancelModule::enable_cancel(module_accessor);
			}
			if SPECIAL_SMASH_BODY == 6 {
				if (SPAWN_SIDE[get_player_number(module_accessor)] && PostureModule::pos_x(module_accessor) > LOW_SPAWN_POS.x - 15.0 && PostureModule::pos_x(module_accessor) < LOW_SPAWN_POS.x + 15.0 && PostureModule::pos_y(module_accessor) > LOW_SPAWN_POS.y - 15.0 && PostureModule::pos_y(module_accessor) < LOW_SPAWN_POS.y + 15.0) 
				|| (!SPAWN_SIDE[get_player_number(module_accessor)] && PostureModule::pos_x(module_accessor) > HIGH_SPAWN_POS.x - 15.0 && PostureModule::pos_x(module_accessor) < HIGH_SPAWN_POS.x + 15.0 && PostureModule::pos_y(module_accessor) > HIGH_SPAWN_POS.y - 15.0 && PostureModule::pos_y(module_accessor) < HIGH_SPAWN_POS.y + 15.0) {
					WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DEAD, true);
				}
			}
		}
	}
}

unsafe fn special_mode(module_accessor: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, status_kind: i32, fighter: &mut L2CFighterCommon, fighter_information: &mut app::FighterInformation) {
	if is_preview_mode() {
		if fighter_kind == *FIGHTER_KIND_MARIO {
			VisibilityModule::set_whole(module_accessor, false);
		}
		if PostureModule::scale(module_accessor) == 1.5 {
			SPECIAL_SMASH_SIZE = 1;
		}
		else if PostureModule::scale(module_accessor) == 0.35 {
			SPECIAL_SMASH_SIZE = 2;
		}
		else {
			SPECIAL_SMASH_SIZE = 0;
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FLOWER) {
			SPECIAL_SMASH_HEAD = 1;
		}
		else if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
			SPECIAL_SMASH_HEAD = 2;
		}
		else {
			SPECIAL_SMASH_HEAD = 0;
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAL) {
			SPECIAL_SMASH_BODY = 1;
		}
		else if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPYCLOAK) {
			SPECIAL_SMASH_BODY = 2;
		}
		else if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF) {
			SPECIAL_SMASH_BODY = 3;
		}
		else if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT) {
			SPECIAL_SMASH_BODY = 4;
		}
		else if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SCREW) {
			SPECIAL_SMASH_BODY = 5;
		}
		else if ItemModule::is_attach_item(module_accessor, smash::app::ItemKind(*ITEM_KIND_BACKSHIELD)) {
			SPECIAL_SMASH_BODY = 6;
		}
		else {
			SPECIAL_SMASH_BODY = 0;
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA) {
			smash::app::sv_animcmd::FT_DISABLE_CURRY_FACE(fighter.lua_state_agent);
			SPECIAL_SMASH_STATUS = 1;
		}
		else if ItemModule::is_attach_item(module_accessor, smash::app::ItemKind(*ITEM_KIND_BADGE)) {
			SPECIAL_SMASH_STATUS = 2;
		}
		else {
			SPECIAL_SMASH_STATUS = 0;
		}
		if MotionModule::whole_rate(module_accessor) == 0.5 {
			SPECIAL_SMASH_RATE = 1;
		}
		else if MotionModule::whole_rate(module_accessor) == 1.2 {
			SPECIAL_SMASH_RATE = 2;
		}
		else {
			SPECIAL_SMASH_RATE = 0;
		}
	}
	else if !sv_information::is_ready_go() {
		WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA);
		PostureModule::set_scale(module_accessor, 1.0, true);
		EffectModule::kill_kind(module_accessor, Hash40::new("sys_curry_shot"), true, true);
		WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAL);
		if fighter_kind != *FIGHTER_KIND_SNAKE {
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPYCLOAK);
		}
		ItemModule::remove_all(module_accessor);
		smash::app::lua_bind::FighterInformation::gravity(fighter_information);
		if HIGH_SPAWN_POS.z == 1.0 
        || LOW_SPAWN_POS.z == 1.0 {
			HIGH_SPAWN_POS = Vector3f{x: 30.0, y: 0.0, z: 0.0};
			LOW_SPAWN_POS = Vector3f{x: -30.0, y: 0.0, z: 0.0};
		}
		if PostureModule::pos_x(module_accessor) > HIGH_SPAWN_POS.x - 30.0 {
			HIGH_SPAWN_POS = Vector3f{x: PostureModule::pos_x(module_accessor) + 30.0, y: PostureModule::pos_y(module_accessor) + 50.0, z: 0.0};
		}
		if PostureModule::pos_x(module_accessor) < LOW_SPAWN_POS.x + 30.0 {
			LOW_SPAWN_POS = Vector3f{x: PostureModule::pos_x(module_accessor) - 30.0, y: PostureModule::pos_y(module_accessor) + 50.0, z: 0.0};
		}
		SPAWN_SIDE[get_player_number(module_accessor)] = PostureModule::pos_x(module_accessor) > 0.0;
		SPAWN_POS[get_player_number(module_accessor)] = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::lr(module_accessor)};
	}
	else {
		HIGH_SPAWN_POS.z = 1.0;
		LOW_SPAWN_POS.z = 1.0;
		if (fighter_kind == *FIGHTER_KIND_EFLAME && ([*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_STANDBY].contains(&status_kind)))
		|| (fighter_kind == *FIGHTER_KIND_ELIGHT && ([*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_STANDBY].contains(&status_kind))) {
			if (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FLOWER) && SPECIAL_SMASH_HEAD == 1) 
			|| (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) && SPECIAL_SMASH_HEAD == 2)
			|| (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF) && SPECIAL_SMASH_BODY == 3)
			|| (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT) && SPECIAL_SMASH_BODY == 4)
			|| (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SCREW) && SPECIAL_SMASH_BODY == 5)
			|| (ItemModule::is_attach_item(module_accessor, smash::app::ItemKind(*ITEM_KIND_BACKSHIELD)) && SPECIAL_SMASH_BODY == 6)
			|| (ItemModule::is_attach_item(module_accessor, smash::app::ItemKind(*ITEM_KIND_BADGE)) && SPECIAL_SMASH_STATUS == 2) {
				for i in 0..TOTAL_FIGHTER {
					ItemModule::remove_all(&mut *get_boma(i));
				}
				WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA);
				USED_FS[get_player_number(module_accessor)] = false;
			}
		}
		if SPECIAL_SMASH_BODY == 3 {
			tennis_mode(module_accessor, status_kind, fighter.lua_state_agent);
		}
		if SPECIAL_SMASH_BODY == 4 {
			fun_di_mode(module_accessor, status_kind, fighter);
		}
		if SPECIAL_SMASH_BODY == 5 {
			one_hit_mode(module_accessor, status_kind);
		}
		if SPECIAL_SMASH_BODY == 6 {
			basketball_mode(module_accessor, fighter.lua_state_agent, fighter);
		}
		if SPECIAL_SMASH_STATUS == 1 {
			smash::app::sv_animcmd::FT_DISABLE_CURRY_FACE(fighter.lua_state_agent);
		}
		if SPECIAL_SMASH_STATUS == 2 {
			let mut globals = fighter.globals_mut().clone();
			smash4_parry(module_accessor, fighter_kind, status_kind, fighter, &mut globals);
		}
		if SPECIAL_SMASH_SIZE == 1 {
			AttackModule::set_attack_power_mul_scale(module_accessor, 1.0);
			PostureModule::set_scale(module_accessor, 1.0, true);
		}
		if SPECIAL_SMASH_SIZE == 2 {
			AttackModule::set_attack_power_mul_scale(module_accessor, 1.0);
			PostureModule::set_scale(module_accessor, 1.0, true);
		}
		if SPECIAL_SMASH_HEAD != 0 
        || SPECIAL_SMASH_BODY != 0 
        || SPECIAL_SMASH_STATUS != 0 {
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA);
			EffectModule::kill_kind(module_accessor, Hash40::new("sys_curry_shot"), true, true);
			if StatusModule::prev_status_kind(module_accessor, 1) == *FIGHTER_STATUS_KIND_DEAD {
				ItemModule::remove_all(module_accessor);
			}
		}
	}
}

#[skyline::hook(replace=smash::app::lua_bind::FighterInformation::gravity)]
unsafe fn gravity_replace(fighter_information: &mut smash::app::FighterInformation) -> f32 {
	let ret = original!()(fighter_information);
	if ret == 1.33 {
		SPECIAL_SMASH_GRAVITY = 1;
	}
	else if ret == 0.66 {
		SPECIAL_SMASH_GRAVITY = 2;
	}
	else {
		SPECIAL_SMASH_GRAVITY = 0;
	}
	return 1.0;
}


#[skyline::hook(offset=0x67a790)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: u64, attacker_object_id: u32, defender_object_id: u32, move_type: u64, arg5: u64, move_type_again: u64) -> u64 {
	let attacker_boma = &mut *smash::app::sv_battle_object::module_accessor(attacker_object_id);
	let defender_boma = &mut *smash::app::sv_battle_object::module_accessor(defender_object_id);
	let attacker_kind = smash::app::utility::get_kind(attacker_boma);
	let defender_kind = smash::app::utility::get_kind(defender_boma);
	//Ball
	if attacker_kind == *ITEM_KIND_SOCCERBALL {
		LAST_TO_HIT_BALL = get_player_number(defender_boma); //If the ball hits someone and then goes out of bounds, the team that got hit loses the stock
	}
	if defender_kind == *ITEM_KIND_SOCCERBALL {
		LAST_TO_HIT_BALL = get_player_number(attacker_boma);
		ALREADY_BOUNCED = false;
	}
	original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::get_int)]
pub unsafe fn get_int_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, int: i32) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	if SPECIAL_SMASH_BODY == 3 
    && fighter_kind == *ITEM_KIND_SOCCERBALL {
		let mut pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
		if pos.x < camera_range().x + 10.0 
        || pos.x > camera_range().y - 10.0 
        || pos.y < camera_range().w + 10.0 { 
			//If we do know who it was, trigger the ball KO sequence
			if ALREADY_BOUNCED {
				BALL_BOUNCED = Vector3f{x: pos.x, y: 0.0, z: 0.0};
			}
			else {
				BALL_BOUNCED = Vector3f{x: pos.x, y: 1.0, z: 0.0};
			}
		}
		if GroundModule::get_touch_flag(module_accessor) == *GROUND_TOUCH_FLAG_DOWN as u64 {
			if ALREADY_BOUNCED 
            || (FIRST_BOUNCE && ((SPAWN_SIDE[LAST_TO_HIT_BALL] && pos.x > 3.0) 
            || (!SPAWN_SIDE[LAST_TO_HIT_BALL] && pos.x < -3.0))) { //If either we already bounced, or we hit the ball but it was still on our side, KO
				BALL_BOUNCED = Vector3f{x: pos.x, y: 0.0, z: 0.0};
				ALREADY_BOUNCED = false;
			}
			else { //Otherwise, just record that we already bounced
				ALREADY_BOUNCED = true;
			}	
			FIRST_BOUNCE = true;
		}
	}
	original!()(module_accessor, int)
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, term: i32) -> bool {
	let situation_kind = StatusModule::situation_kind(module_accessor);
	let ret = original!()(module_accessor, term);
	if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if READY_GO_TIMER != 0 {
			return false;
		}
		if SPECIAL_SMASH_BODY == 3 {
			if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW 
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH {
				return ret && situation_kind == *SITUATION_KIND_AIR;
			}
		}
		return ret;
	}
	return ret;
}

#[smashline::fighter_frame_callback]
fn custom_fighter_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let fighter_kind = smash::app::utility::get_kind(module_accessor) as i32;
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let mut globals = fighter.globals_mut().clone();
		let cat2 = fighter.global_table[CMD_CAT2].get_int() as i32;
        let special: [i32; 225] = [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_2, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_L, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_N, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_EAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END, *ITEM_PACMANKEY_STATUS_KIND_SPECIAL_HAVE, *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_S, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_BLOW, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PULL, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_PASS, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_WAIT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_FAILURE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_FALL_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_JUMP_CANCEL, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_FIRE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BOMB, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_FOOD, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_ITEM, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP];
		LookupSymbol(
			&mut FIGHTER_MANAGER_ADDR,
			"_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
				.as_bytes()
				.as_ptr(),
		);
		LookupSymbol(
			&mut FIGHTER_CUTIN_MANAGER_ADDR,
			"_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
				.as_bytes()
				.as_ptr(),
		);
		let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
		let fighter_information = &mut *smash::app::lua_bind::FighterManager::get_fighter_information(fighter_manager, smash::app::FighterEntryID(get_player_number(module_accessor) as i32));
		STOCK_COUNT[get_player_number(module_accessor)] = smash::app::lua_bind::FighterInformation::stock_count(fighter_information);
		if smash::app::utility::get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER {
			if sv_information::is_ready_go() == false {
				if get_player_number(module_accessor) as i32 == 0 {
					TOTAL_FIGHTER = 1;
				}
				else {
					if fighter_kind != *FIGHTER_KIND_NANA {
						TOTAL_FIGHTER += 1;
					}
				}
			}
			else if is_preview_mode() {
				TOTAL_FIGHTER = 1;
			}
			else {
				for i in 0..TOTAL_FIGHTER {
					if STOCK_COUNT[i as usize] > 1 {
						ALL_FIGHTERS_LAST_STOCK = false;
						break;
					}
					else {
						ALL_FIGHTERS_LAST_STOCK = true;
					}
				}
			}
			//Turbo
			if SPECIAL_SMASH_HEAD == 1 {
				for i in 0..8 {
					if AttackModule::is_attack(module_accessor, i, false) || special.contains(&status_kind) || status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR|| status_kind == *FIGHTER_STATUS_KIND_THROW && MotionModule::frame(module_accessor) >= 1.0 {
						CancelModule::enable_cancel(module_accessor);
					}
				}
				if !is_damage_check(module_accessor){
					enable_specials_force(module_accessor);
					enable_smash_atk_force(module_accessor);
					enable_tilts_force(module_accessor);
					enable_all(module_accessor);
				}
				if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH].contains(&status_kind) 
				|| special.contains(&status_kind) {
					WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
					WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
					WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
				}
			}
			special_mode(module_accessor, fighter_kind, status_kind, fighter, fighter_information);
			status_kind_damage(module_accessor, status_kind, situation_kind, cat2, &mut globals);
            READY_GO[get_player_number(module_accessor)] = sv_information::is_ready_go();
		}
	}
}

pub fn install() {
	smashline::install_agent_frame_callbacks!(custom_fighter_functions);
	skyline::install_hook!(get_int_replace);
	skyline::install_hook!(is_enable_transition_term_replace);
	skyline::install_hook!(notify_log_event_collision_hit_replace);
	skyline::install_hook!(attack_replace);
	skyline::install_hook!(attack_abs_replace);
}