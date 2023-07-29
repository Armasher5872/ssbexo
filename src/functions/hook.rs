use super::*;

//Prevention of Moves in Air/Wavedash Logic (Credit to Chrispo)
#[skyline::hook(replace = StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, unk: bool) -> u64 {
	let next_status = status_kind;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if get_kind(boma) == *FIGHTER_KIND_ALL {
		if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
				original!()(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
			} 
			else {
				original!()(boma, status_kind, unk);
			}
		}
		else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
			if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			}
			original!()(boma, status_kind, unk);
		}
		else {
			original!()(boma, status_kind, unk);
		}
	}
    if get_kind(boma) == *FIGHTER_KIND_SAMUSD
    && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
    && SAMUSD_HAS_FLOAT[entry_id as usize] {
        return 0;
    }
	if get_kind(boma) == *FIGHTER_KIND_PICHU
	&& [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind)
	&& !USE_TACKLE[entry_id as usize] {
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
	if get_kind(boma) == *FIGHTER_KIND_GANON
	&& status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
	&& !USE_DROPKICK[entry_id as usize] {
		return 0;
	}
	if get_kind(boma) == *FIGHTER_KIND_MIIFIGHTER
	&& status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
	&& !USE_ONSLAUGHT[entry_id as usize] {
		return 0;
	}
	return original!()(boma, status_kind, unk);
}

//Changes your situation kind if you're wavedashing
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request)]
pub unsafe fn change_status_request_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
	let next_status = status_kind;
	if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
				original!()(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false)
			} 
			else {
				original!()(boma, status_kind, arg3)
			}
		} 
		else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
			if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			}
			original!()(boma, status_kind, arg3)
		}
		else {
			original!()(boma, status_kind, arg3)
		}
	} 
	else {
		original!()(boma, status_kind, arg3)
	}
}

//Hit Module Handle Attack Event, determines where you hit and with what hitbox id
#[skyline::hook(offset = 0x46ae64, inline)]
unsafe fn hit_module_handle_attack_event(ctx: &InlineCtx) {
    let data = *ctx.registers[1].x.as_ref() as *mut u32;
    let attacker_id = *data;
    let collision_id = *data.add(1);
    let battle_object = &mut *get_battle_object_from_id(attacker_id);
    if !battle_object.is_fighter() && !battle_object.is_weapon() {
        return;
    }
    let collision_data = *ctx.registers[27].x.as_ref() as *mut f32;
    let loc_x = *collision_data.add(4);
    let loc_y = *collision_data.add(5);
    let loc_z = *collision_data.add(6);
    LAST_ATTACK_HITBOX_ID = collision_id as i32;
    LAST_ATTACK_HITBOX_LOCATION_X = loc_x;
    LAST_ATTACK_HITBOX_LOCATION_Y = loc_y;
    LAST_ATTACK_HITBOX_LOCATION_Z = loc_z;
}

//Shield Module Send Shield Attack Collision Event, basically does the same thing as 0x46ae64, but on shield
#[skyline::hook(offset = 0x4c7060)]
unsafe fn shield_module_send_shield_attack_collision_event(shield_module: *mut u64, opp_attack_module: *mut u64, collision: *mut u8, group_index: i32, raw_power: f32, real_power: f32, pos_x: f32, lr: f32) {
    call_original!(shield_module, opp_attack_module, collision, group_index, raw_power, real_power, pos_x, lr);
    let attacker_id = *(collision.add(0x24) as *const u32);
	let battle_object = &mut *get_battle_object_from_id(attacker_id);
    if !battle_object.is_fighter() && !battle_object.is_weapon() {
        return;
    }
    let hitbox_id = *(collision.add(0x33) as *const u8);
    let loc_x = *(collision.add(0x10) as *const f32);
    let loc_y = *(collision.add(0x14) as *const f32);
    let loc_z = *(collision.add(0x18) as *const f32);
    LAST_ATTACK_HITBOX_ID = hitbox_id as i32;
    LAST_ATTACK_HITBOX_LOCATION_X = loc_x;
    LAST_ATTACK_HITBOX_LOCATION_Y = loc_y;
    LAST_ATTACK_HITBOX_LOCATION_Z = loc_z;
}

//Special Smash
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield)]
unsafe fn is_valid_just_shield_replace(boma: &mut BattleObjectModuleAccessor) -> bool {
	if SPECIAL_SMASH_STATUS == 2 {
		return false;
	}
	else {
		original!()(boma)
	}
}

//Parry Reflects
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(_boma: &mut BattleObjectModuleAccessor) -> bool {
	return true;
}

//Changes the title screen version
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const skyline::libc::c_char) {
	let original_string = unsafe {from_c_str(string)};
	if original_string.contains("Ver.") {
		let version_string = format!("{} | SSB:EXO (Beta) | Ver. 0.3.5 \0", original_string);
		call_original!(arg, c_str(&version_string));
	}
	else {
		call_original!(arg, string);
	}
}

//Init Settings (Fixes up issues related to moves that can't normally edge cancel)
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut smash::app::BattleObjectModuleAccessor, situation_kind: i32, arg3: i32, arg4: u64, ground_cliff_check_kind: u64, arg6: bool, arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u64 {
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = smash::app::utility::get_kind(boma);
    if smash::app::utility::get_category(boma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        return original!()(boma, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
    }
    if crate::functions::ext::is_edge_cancel(fighter_kind, status_kind) && situation_kind == SITUATION_KIND_GROUND {
        original!()(boma, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if [
        *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_F, 
        *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, *FIGHTER_STATUS_KIND_ITEM_THROW, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, 
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_DAMAGE
    ].contains(&status_kind) {
        original!()(boma, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        original!()(boma, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
        KineticModule::clear_speed_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    }
    else if fighter_kind == *FIGHTER_KIND_KOOPA && [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G].contains(&status_kind) {
        original!()(boma, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if fighter_kind == *FIGHTER_KIND_GAOGAEN && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        original!()(boma, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if fighter_kind == *FIGHTER_KIND_EDGE
    && ([*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind))
    && (StatusModule::prev_status_kind(boma, 0) != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH)
    && situation_kind == SITUATION_KIND_GROUND {
        original!()(boma, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
        KineticModule::clear_speed_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    }
    else {
        original!()(boma, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
}

//GroundModule::correct (Tells which moves can edge cancel)
#[skyline::hook(replace = smash::app::lua_bind::GroundModule::correct)]
unsafe fn correct_hook(boma: &mut smash::app::BattleObjectModuleAccessor, ground_correct_kind: u32) -> u64 {
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    if smash::app::utility::get_category(boma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        original!()(boma, ground_correct_kind);
    }
    if [
        *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_F, 
        *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, *FIGHTER_STATUS_KIND_ITEM_THROW, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, 
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_DAMAGE
    ].contains(&status_kind) {
        original!()(boma, *GROUND_CORRECT_KIND_GROUND as u32)
    }
    else if status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        original!()(boma, 1 as u32);
        KineticModule::clear_speed_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    }
    else if crate::functions::ext::is_edge_cancel(fighter_kind, status_kind) {
        original!()(boma, *GROUND_CORRECT_KIND_GROUND as u32)
    }
	else if [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_SONIC].contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
		original!()(boma, 1 as u32)
	}
    else if fighter_kind == *FIGHTER_KIND_KOOPA && [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G].contains(&status_kind) {
        original!()(boma, *GROUND_CORRECT_KIND_GROUND as u32)
    }
    else if fighter_kind == *FIGHTER_KIND_GAOGAEN && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        original!()(boma, *GROUND_CORRECT_KIND_KEEP as u32)
    }
    else if fighter_kind == *FIGHTER_KIND_EDGE
    && ([*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind))
    && (StatusModule::prev_status_kind(boma, 0) != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH)
    && situation_kind == SITUATION_KIND_GROUND {
        original!()(boma, *GROUND_CORRECT_KIND_GROUND as u32);
        KineticModule::clear_speed_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    }
    else {
        original!()(boma, ground_correct_kind)
    }
}

//Aerial ECB Fixes, mainly for things like Link, Captain, Simon, and Richter
extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil33get_ground_correct_kind_air_transERNS_26BattleObjectModuleAccessorEi"]
    fn get_ground_correct_kind_air_trans(boma: &mut smash::app::BattleObjectModuleAccessor, something: i32) -> i32;
}

#[skyline::hook(replace=get_ground_correct_kind_air_trans)]
unsafe fn get_ground_correct_kind_air_trans_hook(_boma: &mut smash::app::BattleObjectModuleAccessor, _something: i32) -> i32 {
    return *GROUND_CORRECT_KIND_AIR;
}

//Reverse Knockback
#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
	l2c_agent.clear_lua_stack();
	for i in 0..36 {
		if i == 4
		&& SPECIAL_SMASH_STATUS == 1 {
			if i < 362 {
				let positive_angle = i+(180 as u64);
				let negative_angle = i-(180 as u64);
				if i < 180 {
					l2c_agent.push_lua_stack(&mut L2CValue::new_int(positive_angle));
				}
				else {
					l2c_agent.push_lua_stack(&mut L2CValue::new_int(negative_angle));
				}
			}
		}
		else {
			l2c_agent.push_lua_stack(&mut hitbox_params[i as usize].clone());
		}
	}
	original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::ATTACK_ABS)]
unsafe fn attack_abs_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..15).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
	l2c_agent.clear_lua_stack();
	for i in 0..15 {
		if i == 3
		&& SPECIAL_SMASH_STATUS == 1 {
			if i < 362 {
				let positive_angle = i+(180 as u64);
				let negative_angle = i-(180 as u64);
				if i < 180 {
					l2c_agent.push_lua_stack(&mut L2CValue::new_int(positive_angle));
				}
				else {
					l2c_agent.push_lua_stack(&mut L2CValue::new_int(negative_angle));
				}
			}
		}
		else {
			l2c_agent.push_lua_stack(&mut hitbox_params[i as usize].clone());
		}
	}
    original!()(lua_state);
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

//Notify Log Event Collision Hit
#[skyline::hook(offset=0x67a790)]
pub unsafe fn notify_log_event_collision_hit(fighter_manager: u64, attacker_object_id: u32, defender_object_id: u32, move_type: u64, arg5: u64, move_type_again: u64) -> u64 {
	let attacker_boma = &mut *smash::app::sv_battle_object::module_accessor(attacker_object_id);
	let defender_boma = &mut *smash::app::sv_battle_object::module_accessor(defender_object_id);
	let attacker_kind = smash::app::utility::get_kind(attacker_boma);
	let defender_kind = smash::app::utility::get_kind(defender_boma);
	//Ball
	if attacker_kind == *ITEM_KIND_SOCCERBALL {
		//If the ball hits someone and then goes out of bounds, the team that got hit loses the stock
		LAST_TO_HIT_BALL = get_player_number(defender_boma);
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
            || (!SPAWN_SIDE[LAST_TO_HIT_BALL] && pos.x < -3.0))) { 
				//If either we already bounced, or we hit the ball but it was still on our side, KO
				BALL_BOUNCED = Vector3f{x: pos.x, y: 0.0, z: 0.0};
				ALREADY_BOUNCED = false;
			}
			else { 
				//Otherwise, just record that we already bounced
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

//Installation
pub fn install() {
	skyline::install_hook!(change_status_hook);
	skyline::install_hook!(is_valid_just_shield_reflector);
    skyline::install_hook!(is_valid_just_shield_replace);
    skyline::install_hook!(attack_replace);
	skyline::install_hook!(attack_abs_replace);
	skyline::install_hook!(get_int_replace);
	skyline::install_hook!(is_enable_transition_term_replace);
	skyline::install_hook!(notify_log_event_collision_hit);
	skyline::install_hooks!(
        hit_module_handle_attack_event,
        shield_module_send_shield_attack_collision_event,
		change_status_request_hook,
		change_version_string_hook,
		init_settings_hook,
        correct_hook,
        get_ground_correct_kind_air_trans_hook
    );
}