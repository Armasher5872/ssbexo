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

//Attack Module Set Attack, makes it so random tripping doesn't happen if the move doesn't have a 100% trip chance (Credit to HDR)
#[skyline::hook(offset = 0x3dc160)]
unsafe fn attack_module_set_attack(module: u64, id: i32, group: i32, data: &mut smash2::app::AttackData) {
    let boma = *(module as *mut *mut BattleObjectModuleAccessor).add(1);
    if data.slip < 1.0 {
        data.slip = -1.0;
    }
    call_original!(module, id, group, data)
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

//(Credit to HDR)
pub unsafe fn init_settings_edges(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, arg3: i32, arg4: u32, ground_cliff_check_kind: smash::app::GroundCliffCheckKind, arg6: bool, arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u32 {
	/* "fix" forces GroundModule::correct to be called for the statuses we need */
    let mut fix = arg4;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);
	if boma.is_fighter()
	&& boma.is_situation(*SITUATION_KIND_GROUND) {
		if [
			*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_F, 
			*FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, 
			*FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING
		].contains(&status_kind) {
			fix = *GROUND_CORRECT_KIND_GROUND as u32;
		}
		if (fighter_kind == FIGHTER_KIND_YOSHI && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_LW_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_FOX && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| ([*FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_PICHU].contains(&fighter_kind) && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_LUIGI && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_NESS && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_CAPTAIN && boma.is_status(*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END))
		|| (fighter_kind == *FIGHTER_KIND_SHEIK && [*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_ZELDA && [*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_FALCO && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_S_FALL_LANDING, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_YOUNGLINK && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI))
		|| (fighter_kind == *FIGHTER_KIND_MEWTWO && [*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind))
		|| ([*FIGHTER_KIND_PIT, *FIGHTER_KIND_PITB].contains(&fighter_kind) && boma.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_S_LANDING))
		|| (fighter_kind == *FIGHTER_KIND_SZEROSUIT && [*FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_FLIP, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_LANDING, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PLIZARDON && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_RUSH, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_BLOWN].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_DIDDY && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI))
		|| (fighter_kind == *FIGHTER_KIND_LUCAS && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_AGAIN, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_REFLECT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_SONIC && [*FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_REBOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PIKMIN && boma.is_status(*FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_LANDING))
		|| (fighter_kind == *FIGHTER_KIND_LUCARIO && [*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_TOONLINK && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI))
		|| (fighter_kind == *FIGHTER_KIND_WOLF && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_MURABITO && boma.is_status(*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING))
		|| (fighter_kind == *FIGHTER_KIND_PALUTENA && [*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_GEKKOUGA && boma.is_status(*FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_LOOP))
		|| (fighter_kind == *FIGHTER_KIND_KOOPAJR && boma.is_status(*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING))
		|| ([*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN].contains(&fighter_kind) && [*FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_KAMUI && [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK_END, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_BAYONETTA && [*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_RIDLEY && boma.is_status(*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING))
		|| ([*FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER].contains(&fighter_kind) && boma.is_status(*FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING))
		|| (fighter_kind == *FIGHTER_KIND_KROOL && boma.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_LANDING))
		|| (fighter_kind == *FIGHTER_KIND_GAOGAEN && [*FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_BOUND].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PACKUN && boma.is_status(*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING))
		|| (fighter_kind == *FIGHTER_KIND_DOLLY && [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_EDGE && [*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind) && StatusModule::prev_status_kind(boma, 0) != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH)
		|| (fighter_kind == *FIGHTER_KIND_DEMON && [*FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_AIR_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_MIIFIGHTER && [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_LANDING, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S2_LANDING, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_MIISWORDSMAN && [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_END, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END].contains(&status_kind)) {
			fix = *GROUND_CORRECT_KIND_GROUND as u32;
		}
	}
	return fix
}

//(Credit to HDR)
#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, kinetic_type: i32, arg4: u32, ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool, keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
    let mut cliff_check_kind = ground_cliff_check_kind;                     
    //Call Edge Cancel init_settings
    let fix = init_settings_edges(boma, situation, kinetic_type, arg4, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10);
	//Set GroundCliffCheckKind here to pass into init_settings
    if boma.is_fighter() {
        if ([*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN].contains(&boma.kind()) && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END]))
        || ([*FIGHTER_KIND_FALCO, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_REFLET].contains(&boma.kind()) && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)) {
            cliff_check_kind = smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }
    }
    original!()(boma, situation, kinetic_type, fix, cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10)
}

//GroundModule::correct. The Edge Cancel function (Credit to HDR)
#[skyline::hook(replace=GroundModule::correct)]
unsafe fn correct_hook(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let fighter_kind = boma.kind();
	//All statuses seem to count as "landing" for some reason
    if boma.is_fighter()
    && boma.is_situation(*SITUATION_KIND_GROUND) {
        if [*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
            return original!()(boma, GroundCorrectKind(1));
        }
        if (fighter_kind == *FIGHTER_KIND_YOSHI && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
		|| (([*FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_PICHU].contains(&fighter_kind)) && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_LUIGI && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N)
        || (fighter_kind == *FIGHTER_KIND_CAPTAIN && status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)
		|| (([*FIGHTER_KIND_PEACH, *FIGHTER_KIND_DAISY].contains(&fighter_kind)) && status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END)
		|| (fighter_kind == *FIGHTER_KIND_KOOPA && status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G)
        || (fighter_kind == *FIGHTER_KIND_GANON && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
		|| (fighter_kind == *FIGHTER_KIND_GAOGAEN && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N)
        || (fighter_kind == *FIGHTER_KIND_MIISWORDSMAN && ([*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END].contains(&status_kind) || (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO) == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3 && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW))))
        || (fighter_kind == *FIGHTER_KIND_EDGE && status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH) {
            return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    original!()(boma, kind)
}

//Aerial ECB Fixes, mainly for things like Link, Captain, Simon, and Richter (Credit to HDR)
extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil33get_ground_correct_kind_air_transERNS_26BattleObjectModuleAccessorEi"]
    fn get_ground_correct_kind_air_trans(boma: &mut smash::app::BattleObjectModuleAccessor, something: i32) -> i32;
}

#[skyline::hook(replace=get_ground_correct_kind_air_trans)]
unsafe fn get_ground_correct_kind_air_trans_hook(_boma: &mut smash::app::BattleObjectModuleAccessor, _something: i32) -> i32 {
    return *GROUND_CORRECT_KIND_AIR;
}

//Reverse Knockback, used in Custom Gamemodes
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

//Gravity, used in Custom Gamemodes
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

//Notify Log Event Collision Hit Replace, dictates several things when you've hit the opponent
#[skyline::hook(offset=0x67a790)]
pub unsafe fn notify_log_event_collision_hit(fighter_manager: u64, attacker_object_id: u32, defender_object_id: u32, move_type: u64, arg5: u64, move_type_again: u64) -> u64 {
	let attacker_boma = &mut *smash::app::sv_battle_object::module_accessor(attacker_object_id);
	let defender_boma = &mut *smash::app::sv_battle_object::module_accessor(defender_object_id);
	let attacker_kind = smash::app::utility::get_kind(attacker_boma);
	let defender_kind = smash::app::utility::get_kind(defender_boma);
	let attacker_status_kind = StatusModule::status_kind(attacker_boma);
	//Turbo
	if SPECIAL_SMASH_HEAD == 1 {
		CancelModule::enable_cancel(attacker_boma);
	}
	//Ball
	if attacker_kind == *ITEM_KIND_SOCCERBALL {
		LAST_TO_HIT_BALL = get_player_number(defender_boma); //If the ball hits someone and then goes out of bounds, the team that got hit loses the stock
	}
	if defender_kind == *ITEM_KIND_SOCCERBALL {
		LAST_TO_HIT_BALL = get_player_number(attacker_boma);
		ALREADY_BOUNCED = false;
	}
	//GGST COUNTER!/Little Mac Star Punch Counterhit Detection
	if COUNTERHIT_CHECK[get_player_number(defender_boma)]
	&& attacker_boma.is_fighter() {
		if attacker_kind != *FIGHTER_KIND_LITTLEMAC 
		&& [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&attacker_status_kind) {
			//Counterhit Detection
			COUNTERHIT_SUCCESS[get_player_number(attacker_boma)] = true;
			COUNTERHIT_CHECK[get_player_number(defender_boma)] = false;
			AttackModule::set_add_reaction_frame_revised(attacker_boma, 0, 10.0, false);
			AttackModule::set_add_reaction_frame_revised(attacker_boma, 1, 10.0, false);
			AttackModule::set_add_reaction_frame_revised(attacker_boma, 2, 10.0, false);
			AttackModule::set_add_reaction_frame_revised(attacker_boma, 3, 10.0, false);
			AttackModule::set_add_reaction_frame_revised(attacker_boma, 4, 10.0, false);
			AttackModule::set_add_reaction_frame_revised(attacker_boma, 5, 10.0, false);
			AttackModule::set_add_reaction_frame_revised(attacker_boma, 6, 10.0, false);
			AttackModule::set_add_reaction_frame_revised(attacker_boma, 7, 10.0, false);
		}
	}
	LAST_DAMAGE[get_player_number(defender_boma)] = DamageModule::damage(defender_boma, 0);
	original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::get_int)]
pub unsafe fn get_int_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, int: i32) -> u64 {
	let fighter_kind = smash::app::utility::get_kind(module_accessor);
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

//Link Events. Enables proper transition into grabs
#[skyline::hook(offset = 0x993ec0)]
pub unsafe extern "C" fn donkey_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let module_accessor = fighter.battle_object.module_accessor;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            capture_event.result = true;
            capture_event.node = smash2::phx::Hash40::new("throw");
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_CATCH_PULL, false);
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
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
        get_ground_correct_kind_air_trans_hook,
        attack_module_set_attack,
        donkey_link_event
    );
}