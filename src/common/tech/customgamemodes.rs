//Credit to Championship Edition and VinegarChicken for the source code
use super::*;

unsafe fn one_hit_mode(module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
	let got_hit = WorkModule::get_int(module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
	if READY_GO_TIMER != 0 
    && get_player_number(module_accessor) == 0 {
		READY_GO_TIMER -= 1;
	}
    //When someone gets hit, start a timer
	if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_SLIP, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
		WorkModule::inc_int(module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);	
	} 
	else {
		WorkModule::set_int(module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
	}
	if got_hit == 2 { 
		//If the timer reaches 2, check to see if anyone else has been hit
		for i in 0..TOTAL_FIGHTER + 1 {
			if got_hit != 0 {
				if i as usize != get_player_number(module_accessor) {
					WorkModule::set_int(module_accessor, 3, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
					//If anyone has, don't do anything
					break;
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
			if pos.x < dead_range(lua_state).x { 
				//Right
				pos.x = dead_range(lua_state).y;
			}
			if pos.x > dead_range(lua_state).y { 
				//Left
				pos.x = dead_range(lua_state).x;
			}
			if pos.y > dead_range(lua_state).z { 
				//Up
				pos.y = dead_range(lua_state).w;
			}
			if pos.y < dead_range(lua_state).w { 
				//Down
				pos.y = dead_range(lua_state).z;
			} 
			let correct_kind = smash::app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
			GroundModule::set_correct(module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
			PostureModule::set_pos(module_accessor, &pos);
			GroundModule::set_correct(module_accessor, correct_kind);
		}
		AttackModule::set_no_dead_all(module_accessor, true, false);
		WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
		//Invisible wall preventing fighters from entering each others territory
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
			if BALL_BOUNCED.y == 0.0 { 
				//If the ball bounced on the ground, KO everyone who was on the same side as the ball
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
			else { 
				//If the ball went out of bounds, KO everyone who was on the same side as whoever last hit the ball
				if SPAWN_SIDE[get_player_number(module_accessor)] == SPAWN_SIDE[LAST_TO_HIT_BALL] {
					for i in 0..3 {
						if BALL_VICTIMS[i] == 9 {
							BALL_VICTIMS[i] = get_player_number(module_accessor) as i32;
							break;
						}
					}
				}
			}
			if get_player_number(module_accessor) as i32 == TOTAL_FIGHTER - 1 { 
				//Once all victims have gotten a chance to be put on the list, KO them
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
				//Then get rid of the soccerball and reset these values so multiple stocks aren't taken
				smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, BALL_ID);
				BALL_BOUNCED = Vector3f{x: 0.0, y: 0.0, z: 9999.0};
			}
		}
		if !WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO) { 
			//The frame the game starts, randomly pick a player to give the ball
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
		if READY_GO_TIMER != 0 { 
			//Lock everyone in place while waiting
			if READY_GO_TIMER == 180 { 
				//Give each player a temporary spawn position that matches one of the previously used spawn positions, and doesn't match any of the other players temporary spawn positions
				let mut new_pos = SPAWN_POS[smash::app::sv_math::rand(hash40("fighter"), TOTAL_FIGHTER) as usize];
				let mut pos_shuffle = false;
				while pos_shuffle == false {
					//Allow the loop to end, then compare new_pos with the randomly selected positions of all previous fighters. If there's a match, pick a new position and force the loop to restart
					pos_shuffle = true;
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
			if READY_GO_TIMER == 1 { 
				//After everyone has respawned, randomly choose a player who was just KO'd and give them the ball
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

unsafe fn basketball_mode(module_accessor: &mut smash::app::BattleObjectModuleAccessor, lua_state: u64, fighter: &mut L2CFighterCommon) {
	//Disables Blastzones
	let mut pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
	if pos.x < dead_range(lua_state).x 
    || pos.x > dead_range(lua_state).y 
    || pos.y > dead_range(lua_state).z 
    || pos.y < dead_range(lua_state).w {
		if pos.x < dead_range(lua_state).x { 
			//Right
			pos.x = dead_range(lua_state).y;
		}
		if pos.x > dead_range(lua_state).y { 
			//Left
			pos.x = dead_range(lua_state).x;
		}
		if pos.y > dead_range(lua_state).z { 
			//Up
			pos.y = dead_range(lua_state).w;
		}
		if pos.y < dead_range(lua_state).w { 
			//Down
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

unsafe fn status_kind_damage(fighter: &mut L2CFighterCommon, module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, globals: &mut L2CValue) {
	if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_TREAD_DAMAGE].contains(&status_kind) {
		if status_kind != *FIGHTER_STATUS_KIND_TREAD_DAMAGE {
            //Fast Fall during Tumble
			if [*FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
				if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
					if (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0
                    && (ControlModule::get_stick_y(module_accessor) < -0.66) 
                    && (KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= -0.5) {
						WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					}
				}
			}
			//Hitstunless
			if SPECIAL_SMASH_HEAD == 2 {
				CancelModule::enable_cancel(module_accessor);
			}
			//Smashketball
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
				WorkModule::set_flag(module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
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
		if SPECIAL_SMASH_HEAD != 0 || SPECIAL_SMASH_BODY != 0 || SPECIAL_SMASH_STATUS != 0 {
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA);
			EffectModule::kill_kind(module_accessor, Hash40::new("sys_curry_shot"), true, true);
			if StatusModule::prev_status_kind(module_accessor, 1) == *FIGHTER_STATUS_KIND_DEAD {
				ItemModule::remove_all(module_accessor);
			}
		}
	}
}

#[smashline::fighter_frame_callback]
fn custom_fighter_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let fighter_kind = smash::app::utility::get_kind(module_accessor) as i32;
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let mut globals = fighter.globals_mut().clone();
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
			special_mode(module_accessor, fighter_kind, status_kind, fighter, fighter_information);
			status_kind_damage(fighter, module_accessor, status_kind, &mut globals);
			WorkModule::set_flag(module_accessor, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
		}
	}
}

pub fn install() {
	install_agent_frame_callbacks!(custom_fighter_functions);
}