use super::*;

macro_rules! apply_button_mappings {
    ($controller:ident, $mappings:ident, $(($button:ident, $mapped:ident, $kind:ident, $output:expr))*) => {{
        let mut buttons = Buttons::empty();
        $(
                if $controller.current_buttons.$button() && (*$mappings).$mapped == InputKind::$kind {
                    buttons |= $output;
                }
        )*
        buttons
    }}
}

#[repr(C)]
struct SomeControllerStruct {
    padding: [u8; 0x10],
    controller: &'static mut Controller
}

#[repr(C)]
struct ControlModuleInternal {
    vtable: *mut u8,
    controller_index: i32,
    buttons: Buttons,
    stick_x: f32,
    stick_y: f32,
    padding: [f32; 2],
    unk: [u32; 8],
    clamped_lstick_x: f32,
    clamped_lstick_y: f32,
    padding2: [f32; 2],
    clamped_rstick_x: f32,
    clamped_rstick_y: f32,
}

//Prevention of Moves in Air (Credit to Chrispo)
#[skyline::hook(replace = StatusModule::change_status_request_from_script)]
unsafe fn change_status_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, unk: bool) -> u64 {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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
	if get_kind(boma) == *FIGHTER_KIND_MIIFIGHTER
	&& status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
	&& !USE_ONSLAUGHT[entry_id as usize] {
		return 0;
	}
	return original!()(boma, status_kind, unk);
}

//On Flag Hook (Credit to Chrispo)
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::on_flag)]
unsafe fn on_flag_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: c_int) -> () {
	if boma.is_fighter() { 
		if int == *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI {
			let entry_id =  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			//Removal of SH macro via hooking on_flag. FULL_HOP_ENABLE_DELAY allows fullhop button to not give shorthops. 
			if (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI)) && !(FULL_HOP_ENABLE_DELAY[entry_id] > 0) {
				original!()(boma, int)
			} 
			else {
				println!("SH height banned");
			}
		}
		else {
			original!()(boma, int)
		}
	}
	else {
		original!()(boma, int)
	}
}

//Deals with DK's Barrels, Gordo, and CC
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request)]
unsafe fn change_status_request_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
	let mut next_status = status_kind;
    let frame = MotionModule::frame(boma);
    if (boma.is_weapon() && boma.kind() == *WEAPON_KIND_DEDEDE_GORDO) {
        if next_status == *WEAPON_DEDEDE_GORDO_STATUS_KIND_ATTACK || next_status == *WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP {
            HitModule::set_no_team(boma, true);
        }
    }
	else if (boma.is_item() && boma.kind() == *ITEM_KIND_BARREL) {
        if next_status == *ITEM_STATUS_KIND_BORN || next_status == *ITEM_STATUS_KIND_LOST {
            let bounce_mul = Vector3f { x: -0.25, y: -0.25, z: -0.25 };
            KineticModule::mul_speed(boma, &bounce_mul, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            PostureModule::reverse_lr(boma);
            AttackModule::clear_all(boma);
            next_status = *ITEM_STATUS_KIND_FALL;
            TeamModule::set_hit_team(boma, *TEAM_NONE);
        }
    }
	original!()(boma, next_status, arg3)
}

//Hit Module Handle Attack Event, determines where you hit and with what hitbox id
//13.0.2 Offset: 0x46ae84
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

//Shield Module Send Shield Attack Collision Event, basically does the same thing as 0x46ae64, but on shield. Also dictates hard shield breaks
//13.0.2 Offset: 0x4c7080
#[skyline::hook(offset = 0x4c7060)]
unsafe fn shield_module_send_shield_attack_collision_event(shield_module: *mut u64, opp_attack_module: *mut u64, collision: *mut u8, group_index: i32, raw_power: f32, real_power: f32, pos_x: f32, lr: f32) {
    let defender_boma = *(shield_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let defender_status_kind = StatusModule::status_kind(defender_boma);
    let attacker_id = *(collision.add(0x24) as *const u32);
	let attacker_battle_object = &mut *get_battle_object_from_id(attacker_id);
    let attacker_boma = attacker_battle_object.module_accessor;
    let shield_damage = WorkModule::get_int(attacker_boma, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE) as f32;
    let motion_rate: f32;
    if !attacker_battle_object.is_fighter() && !attacker_battle_object.is_weapon() {
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
    if (real_power+shield_damage) >= 30.0 && [*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD].contains(&defender_status_kind) {
        StatusModule::change_status_request_from_script(defender_boma, *FIGHTER_STATUS_KIND_FURAFURA, false);
    }
    if real_power == 0.0 {
        motion_rate = 1.0;
    } 
    else {
        motion_rate = (1.0-(0.02*real_power)).clamp(0.5, 1.0);
    };
    if defender_status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF
    && FighterUtil::is_valid_just_shield(defender_boma) {
        WorkModule::set_float(attacker_boma, motion_rate, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
        if attacker_battle_object.is_situation(*SITUATION_KIND_AIR) {
            StatusModule::change_status_request_from_script(attacker_boma, *FIGHTER_STATUS_KIND_REBOUND_JUMP, false);
        }
        else {
            StatusModule::change_status_request_from_script(attacker_boma, *FIGHTER_STATUS_KIND_REBOUND, false);
        }
    }
    call_original!(shield_module, opp_attack_module, collision, group_index, raw_power, real_power, pos_x, lr);
}

//Attack Module Set Attack, makes it so random tripping doesn't happen if the move doesn't have a 100% trip chance (Credit to HDR)
//13.0.2 Offset: 0x3dc180
#[skyline::hook(offset = 0x3dc160)]
unsafe fn attack_module_set_attack(module: u64, id: i32, group: i32, data: &mut smash2::app::AttackData) {
    let boma = *(module as *mut *mut BattleObjectModuleAccessor).add(1);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if data.slip < 1.0 {
        data.slip = -1.0;
    }
    if (data.sub_shield as f32) < 0.0 || (data.sub_shield as f32) > 50.0 {
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    }
    else {
        WorkModule::set_int(boma, data.sub_shield as i32, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    }
    if (*boma).is_item() && (*boma).kind() == *ITEM_KIND_DEKU && PFUSHIGISOU_IS_ACTIVE_BOMB[entry_id] {
        data.power = 8.0;
        data.r_eff = 100;
        data.r_add = 20;
        data.size = 8.0;
        data.no_reaction_search = 0;
        data.r_fix = 0;
        data.attr = smash2::phx::Hash40::new("collision_attr_normal");
    }
    call_original!(module, id, group, data);
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

//Changes the title screen version
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const skyline::libc::c_char) {
	let original_string = unsafe {from_c_str(string)};
	if original_string.contains("Ver.") {
		let version_string = format!("{} / SSB:EXO (Beta) | Ver. 0.3.6 \0", original_string);
		call_original!(arg, c_str(&version_string));
	}
	else {
		call_original!(arg, string);
	}
}

//(Credit to HDR)
pub unsafe fn init_settings_edges(boma: &mut BattleObjectModuleAccessor, _situation: smash::app::SituationKind, _arg3: i32, arg4: u32, _ground_cliff_check_kind: smash::app::GroundCliffCheckKind, _arg6: bool, _arg7: i32, _arg8: i32, _arg9: i32, _arg10: i32) -> u32 {
	/* "fix" forces GroundModule::correct to be called for the statuses we need */
    let mut fix = arg4;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);
	if boma.is_fighter() && boma.is_situation(*SITUATION_KIND_GROUND) {
		if [
			*FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, 
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING
		].contains(&status_kind) {
			fix = *GROUND_CORRECT_KIND_GROUND as u32;
		}
		if (fighter_kind == *FIGHTER_KIND_YOSHI && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_LW_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_FOX && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PIKACHU && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_LUIGI && boma.is_status(*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL))
		|| (fighter_kind == *FIGHTER_KIND_NESS && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_CAPTAIN && boma.is_status(*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END))
		|| (fighter_kind == *FIGHTER_KIND_SHEIK && [*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_ZELDA && [*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PICHU && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind))
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
    let fighter_kind = boma.kind();
	//All statuses seem to count as "landing" for some reason
    if boma.is_fighter()
    && boma.is_situation(*SITUATION_KIND_GROUND) {
        if [*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
            return original!()(boma, GroundCorrectKind(1));
        }
        if (fighter_kind == *FIGHTER_KIND_YOSHI && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
		|| (fighter_kind == *FIGHTER_KIND_LUIGI && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N)
        || (fighter_kind == *FIGHTER_KIND_CAPTAIN && status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)
		|| (([*FIGHTER_KIND_PEACH, *FIGHTER_KIND_DAISY].contains(&fighter_kind)) && status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END)
		|| (fighter_kind == *FIGHTER_KIND_KOOPA && status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G)
        || (fighter_kind == *FIGHTER_KIND_PICHU && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind))
        || (fighter_kind == *FIGHTER_KIND_GANON && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
		|| (fighter_kind == *FIGHTER_KIND_GAOGAEN && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N)
        || (fighter_kind == *FIGHTER_KIND_EDGE && status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH)
        || (fighter_kind == *FIGHTER_KIND_MIISWORDSMAN && ([*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END].contains(&status_kind) || (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO) == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3 && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW)))) {
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

//Used for reverse knockback
#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    if SPECIAL_SMASH_STATUS == 1 {
        let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        for (i, x) in hitbox_params.iter_mut().enumerate().take(36) {
            if i == 4 {
                if i < 362 {
                    let positive_angle = (i+180) as f32;
                    let negative_angle = (i-180) as f32;
                    if i < 180 {
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(positive_angle));
                    }
                    else {
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(negative_angle));
                    }
                }
            }
            else {
                l2c_agent.push_lua_stack(x);
            }
        }
    }
	original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::ATTACK_ABS)]
unsafe fn attack_abs_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    if SPECIAL_SMASH_STATUS == 1 {
        let mut hitbox_params: Vec<L2CValue> = (0..15).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        for (i, x) in hitbox_params.iter_mut().enumerate().take(15) {
            if i == 3 {
                if i < 362 {
                    let positive_angle = (i+180) as f32;
                    let negative_angle = (i-180) as f32;
                    if i < 180 {
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(positive_angle));
                    }
                    else {
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(negative_angle));
                    }
                }
            }
            else {
                l2c_agent.push_lua_stack(x);
            }
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

//Notify Log Event Collision Hit, dictates several things when you've hit the opponent
//13.0.2 Offset: 0x67a7b0
#[skyline::hook(offset=0x67a790)]
unsafe fn notify_log_event_collision_hit(fighter_manager: u64, attacker_object_id: u32, defender_object_id: u32, move_type: u64, arg5: u64, move_type_again: u64) -> u64 {
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
        WorkModule::set_flag(attacker_boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
	}
	//GGST COUNTER!/Little Mac Star Punch Counterhit Detection
	if COUNTERHIT_CHECK[get_player_number(defender_boma)]
	&& attacker_boma.is_fighter() {
		if attacker_kind != *FIGHTER_KIND_LITTLEMAC 
		&& [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&attacker_status_kind) {
			//Counterhit Detection
			COUNTERHIT_SUCCESS[get_player_number(attacker_boma)] = true;
			COUNTERHIT_CHECK[get_player_number(defender_boma)] = false;
		}
	}
	LAST_DAMAGE[get_player_number(defender_boma)] = DamageModule::damage(defender_boma, 0);
	original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::get_int)]
unsafe fn get_int_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, int: i32) -> u64 {
	let fighter_kind = smash::app::utility::get_kind(module_accessor);
	if SPECIAL_SMASH_BODY == 3 
    && fighter_kind == *ITEM_KIND_SOCCERBALL {
		let pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
		if pos.x < camera_range().x + 10.0 
        || pos.x > camera_range().y - 10.0 
        || pos.y < camera_range().w + 10.0 { 
			//If we do know who it was, trigger the ball KO sequence
			if WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED) {
				BALL_BOUNCED = Vector3f{x: pos.x, y: 0.0, z: 0.0};
			}
			else {
				BALL_BOUNCED = Vector3f{x: pos.x, y: 1.0, z: 0.0};
			}
		}
		if GroundModule::get_touch_flag(module_accessor) == *GROUND_TOUCH_FLAG_DOWN as u64 {
			if WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED)
            || (WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE) && ((SPAWN_SIDE[LAST_TO_HIT_BALL] && pos.x > 3.0) 
            || (!SPAWN_SIDE[LAST_TO_HIT_BALL] && pos.x < -3.0))) { //If either we already bounced, or we hit the ball but it was still on our side, KO
				BALL_BOUNCED = Vector3f{x: pos.x, y: 0.0, z: 0.0};
                WorkModule::set_flag(module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
			}
			else { 
                //Otherwise, just record that we already bounced
				WorkModule::set_flag(module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
			}
            WorkModule::set_flag(module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
		}
	}
	original!()(module_accessor, int)
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
unsafe fn is_enable_transition_term_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, term: i32) -> bool {
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

//13.0.2 Offset: 0x1D39FE0
#[skyline::hook(offset = 0x1d39500)]
unsafe fn get_button_label_by_operation_kind(hashed_string: &mut HashedString, operation: u8, arg: bool) {
    if operation == crate::functions::ext::InputKind::JumpMini as u8 {
        for (index, byte) in "mnu_opt_btn_key_short_hop\0".as_bytes().iter().enumerate() {
            hashed_string.contents[index] = *byte;
        }
        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_short_hop");
    } else if operation == crate::functions::ext::InputKind::FullHop as u8 {
        for (index, byte) in "mnu_opt_btn_key_tilt_attack\0".as_bytes().iter().enumerate() {
            hashed_string.contents[index] = *byte;
        }
        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_tilt_attack");
    } else {
        return call_original!(hashed_string, operation, arg)
    }
}

//13.0.2 Offset: 0x1d334c8
#[skyline::hook(offset = 0x1d329e8, inline)]
unsafe fn add_footstool_to_gc(ctx: &skyline::hooks::InlineCtx) {
    let button = *ctx.registers[25].w.as_ref();
    if ![0x3, 0x4, 0x5, 0x8].contains(&button) {
        let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);

        if input_list_vector.len() < 8 {
            input_list_vector.push(crate::functions::ext::InputKind::AppealHi as u8);
            input_list_vector.push(crate::functions::ext::InputKind::JumpMini as u8);
            input_list_vector.push(crate::functions::ext::InputKind::SmashAttack as u8);
            input_list_vector.push(crate::functions::ext::InputKind::FullHop as u8);
        }
    }
}

//13.0.2 Offset: 0x1D331D8
#[skyline::hook(offset = 0x1d326f8, inline)]
unsafe fn add_footstool_to_fk(ctx: &skyline::hooks::InlineCtx) {
    let button = *ctx.registers[25].w.as_ref();
    if [0x4, 0x5, 0x6, 0x9].contains(&button) {
        return;
    }
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);

    if input_list_vector.len() < 8 {
        input_list_vector.push(crate::functions::ext::InputKind::AppealHi as u8);
        input_list_vector.push(crate::functions::ext::InputKind::JumpMini as u8);
        input_list_vector.push(crate::functions::ext::InputKind::SmashAttack as u8);
        input_list_vector.push(crate::functions::ext::InputKind::FullHop as u8);
    }
}

//13.0.2 Offset: 0x1D33CB8
#[skyline::hook(offset = 0x1d3395c, inline)]
unsafe fn add_footstool_to_jc(ctx: &skyline::hooks::InlineCtx) {
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);

    if input_list_vector.len() < 8 {
        input_list_vector.push(crate::functions::ext::InputKind::AppealHi as u8);
        input_list_vector.push(crate::functions::ext::InputKind::JumpMini as u8);
        input_list_vector.push(crate::functions::ext::InputKind::SmashAttack as u8);
        input_list_vector.push(crate::functions::ext::InputKind::FullHop as u8);
    }
}

//13.0.2 Offset: 0x1D3592C
#[skyline::hook(offset = 0x1d34e4c, inline)]
unsafe fn add_more_buttons(ctx: &mut skyline::hooks::InlineCtx) {
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);
    // panic!("{}", input_list_vector.len());
    *ctx.registers[25].x.as_mut() = input_list_vector.len() as u64;
}

//13.0.2 Offset: 0x16d85dc
#[skyline::hook(offset = 0x16d948c, inline)]
unsafe fn packed_packet_creation(ctx: &mut skyline::hooks::InlineCtx) {
    *ctx.registers[22].x.as_mut() = 0x2;
}

//13.0.2 Offset: 0x16d8610
#[skyline::hook(offset = 0x16d94c0, inline)]
unsafe fn write_packet(ctx: &mut skyline::hooks::InlineCtx) {
    let raw = *ctx.registers[19].x.as_ref();

    let mapped_inputs = *((raw + 0x49508) as *const MappedInputs);
    let mut packet = 0u64;

    *(&mut packet as *mut u64 as *mut i8) = mapped_inputs.lstick_x;
    *(&mut packet as *mut u64 as *mut i8).add(1) = mapped_inputs.lstick_y;

    let buttons = (mapped_inputs.buttons.bits() as u64) << 16;
    packet |= buttons;

    *(&mut packet as *mut u64 as *mut i8).add(6) = mapped_inputs.rstick_x;
    *(&mut packet as *mut u64 as *mut i8).add(7) = mapped_inputs.rstick_y;

    *ctx.registers[8].x.as_mut() = packet;
}

#[skyline::hook(offset = crate::functions::offsets::map_controls())]
unsafe fn map_controls_hook(
    mappings: *mut ControllerMapping,
    player_idx: i32,
    out: *mut MappedInputs,
    controller_struct: &mut SomeControllerStruct,
    arg: bool
) {
    let entry_count = (*mappings.add(player_idx as usize))._34[0];
    let ret = original!()(mappings, player_idx, out, controller_struct, arg);
    let controller = &mut controller_struct.controller;

    //println!("entry_count vs. current: {} vs. {}", entry_count, (*mappings.add(player_idx as usize))._34[0]);

    if (*out).buttons.contains(Buttons::CStickOn) && (*mappings.add(player_idx as usize))._34[0] != entry_count {
        (*out).rstick_x = (controller.left_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.left_stick_y * (i8::MAX as f32)) as i8;
        (*out).buttons |= Buttons::CStickOverride;
    } else {
        (*out).rstick_x = (controller.right_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.right_stick_y * (i8::MAX as f32)) as i8;
    }

    let mappings = mappings.add(player_idx as usize);

    if controller.style == ControllerStyle::GCController {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, gc_r, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, gc_z, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, gc_z, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (a, gc_a, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (b, gc_b, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (x, gc_x, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (y, gc_y, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l, SmashAttack, Buttons::AttackAll)
                (r, gc_r, SmashAttack, Buttons::AttackAll)
                (zl, gc_z, SmashAttack, Buttons::AttackAll)
                (zr, gc_z, SmashAttack, Buttons::AttackAll)
                (a, gc_a, SmashAttack, Buttons::AttackAll)
                (b, gc_b, SmashAttack, Buttons::AttackAll)
                (x, gc_x, SmashAttack, Buttons::AttackAll)
                (y, gc_y, SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l,   AppealHi, Buttons::AppealHi)
                (r, gc_r,   AppealHi, Buttons::AppealHi)
                (zl, gc_z,  AppealHi, Buttons::AppealHi)
                (zr, gc_z,  AppealHi, Buttons::AppealHi)
                (a, gc_a,   AppealHi, Buttons::AppealHi)
                (b, gc_b,   AppealHi, Buttons::AppealHi)
                (x, gc_x,   AppealHi, Buttons::AppealHi)
                (y, gc_y,   AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l,   FullHop, Buttons::FullHop | Buttons::Jump)
                (r, gc_r,   FullHop, Buttons::FullHop | Buttons::Jump)
                (zl, gc_z,  FullHop, Buttons::FullHop | Buttons::Jump)
                (zr, gc_z,  FullHop, Buttons::FullHop | Buttons::Jump)
                (a, gc_a,   FullHop, Buttons::FullHop | Buttons::Jump)
                (b, gc_b,   FullHop, Buttons::FullHop | Buttons::Jump)
                (x, gc_x,   FullHop, Buttons::FullHop | Buttons::Jump)
                (y, gc_y,   FullHop, Buttons::FullHop | Buttons::Jump)
        );
        /*
        if (*mappings.add(player_idx as usize)).gc_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
            }
        }
        */
    } else if controller.style == ControllerStyle::LeftJoycon || controller.style == ControllerStyle::RightJoycon {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, joy_shoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, joy_zshoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, joy_zshoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (left_sl, joy_sl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (left_sr, joy_sr, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (right_sl, joy_sl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (right_sr, joy_sr, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   SmashAttack, Buttons::AttackAll)
                (r, joy_shoulder,   SmashAttack, Buttons::AttackAll)
                (zl, joy_zshoulder, SmashAttack, Buttons::AttackAll)
                (zr, joy_zshoulder, SmashAttack, Buttons::AttackAll)
                (left_sl, joy_sl,   SmashAttack, Buttons::AttackAll)
                (left_sr, joy_sr,   SmashAttack, Buttons::AttackAll)
                (right_sl, joy_sl,  SmashAttack, Buttons::AttackAll)
                (right_sr, joy_sr,  SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   AppealHi, Buttons::AppealHi)
                (r, joy_shoulder,   AppealHi, Buttons::AppealHi)
                (zl, joy_zshoulder, AppealHi, Buttons::AppealHi)
                (zr, joy_zshoulder, AppealHi, Buttons::AppealHi)
                (left_sl, joy_sl,   AppealHi, Buttons::AppealHi)
                (left_sr, joy_sr,   AppealHi, Buttons::AppealHi)
                (right_sl, joy_sl,  AppealHi, Buttons::AppealHi)
                (right_sr, joy_sr,  AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   FullHop, Buttons::FullHop | Buttons::Jump)
                (r, joy_shoulder,   FullHop, Buttons::FullHop | Buttons::Jump)
                (zl, joy_zshoulder, FullHop, Buttons::FullHop | Buttons::Jump)
                (zr, joy_zshoulder, FullHop, Buttons::FullHop | Buttons::Jump)
                (left_sl, joy_sl,   FullHop, Buttons::FullHop | Buttons::Jump)
                (left_sr, joy_sr,   FullHop, Buttons::FullHop | Buttons::Jump)
                (right_sl, joy_sl,  FullHop, Buttons::FullHop | Buttons::Jump)
                (right_sr, joy_sr,  FullHop, Buttons::FullHop | Buttons::Jump)
        );

        if controller.style == ControllerStyle::LeftJoycon {
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_right, joy_up, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_up, joy_left, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_down, joy_right, JumpMini, Buttons::JumpMini | Buttons::Jump)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down, SmashAttack, Buttons::AttackAll)
                    (dpad_right, joy_up, SmashAttack, Buttons::AttackAll)
                    (dpad_up, joy_left, SmashAttack, Buttons::AttackAll)
                    (dpad_down, joy_right, SmashAttack, Buttons::AttackAll)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down,   AppealHi, Buttons::AppealHi)
                    (dpad_right, joy_up,    AppealHi, Buttons::AppealHi)
                    (dpad_up, joy_left,     AppealHi, Buttons::AppealHi)
                    (dpad_down, joy_right,  AppealHi, Buttons::AppealHi)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down,   FullHop, Buttons::FullHop | Buttons::Jump)
                    (dpad_right, joy_up,    FullHop, Buttons::FullHop | Buttons::Jump)
                    (dpad_up, joy_left,     FullHop, Buttons::FullHop | Buttons::Jump)
                    (dpad_down, joy_right,  FullHop, Buttons::FullHop | Buttons::Jump)
            );
        } else {
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (y, joy_up, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (b, joy_left, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (x, joy_right, JumpMini, Buttons::JumpMini | Buttons::Jump)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down, SmashAttack, Buttons::AttackAll)
                    (y, joy_up, SmashAttack, Buttons::AttackAll)
                    (b, joy_left, SmashAttack, Buttons::AttackAll)
                    (x, joy_right, SmashAttack, Buttons::AttackAll)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down,   AppealHi, Buttons::AppealHi)
                    (y, joy_up,     AppealHi, Buttons::AppealHi)
                    (b, joy_left,   AppealHi, Buttons::AppealHi)
                    (x, joy_right,  AppealHi, Buttons::AppealHi)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down,   FullHop, Buttons::FullHop | Buttons::Jump)
                    (y, joy_up,     FullHop, Buttons::FullHop | Buttons::Jump)
                    (b, joy_left,   FullHop, Buttons::FullHop | Buttons::Jump)
                    (x, joy_right,  FullHop, Buttons::FullHop | Buttons::Jump)
            );
        }
        /*
        if (*mappings.add(player_idx as usize)).joy_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
            }
        }*/
    } else {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, pro_r, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, pro_zl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, pro_zr, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (a, pro_a, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (b, pro_b, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (x, pro_x, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (y, pro_y, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l, SmashAttack, Buttons::AttackAll)
                (r, pro_r, SmashAttack, Buttons::AttackAll)
                (zl, pro_zl, SmashAttack, Buttons::AttackAll)
                (zr, pro_zr, SmashAttack, Buttons::AttackAll)
                (a, pro_a, SmashAttack, Buttons::AttackAll)
                (b, pro_b, SmashAttack, Buttons::AttackAll)
                (x, pro_x, SmashAttack, Buttons::AttackAll)
                (y, pro_y, SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l,      AppealHi, Buttons::AppealHi)
                (r, pro_r,      AppealHi, Buttons::AppealHi)
                (zl, pro_zl,    AppealHi, Buttons::AppealHi)
                (zr, pro_zr,    AppealHi, Buttons::AppealHi)
                (a, pro_a,      AppealHi, Buttons::AppealHi)
                (b, pro_b,      AppealHi, Buttons::AppealHi)
                (x, pro_x,      AppealHi, Buttons::AppealHi)
                (y, pro_y,      AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l,      FullHop, Buttons::FullHop | Buttons::Jump)
                (r, pro_r,      FullHop, Buttons::FullHop | Buttons::Jump)
                (zl, pro_zl,    FullHop, Buttons::FullHop | Buttons::Jump)
                (zr, pro_zr,    FullHop, Buttons::FullHop | Buttons::Jump)
                (a, pro_a,      FullHop, Buttons::FullHop | Buttons::Jump)
                (b, pro_b,      FullHop, Buttons::FullHop | Buttons::Jump)
                (x, pro_x,      FullHop, Buttons::FullHop | Buttons::Jump)
                (y, pro_y,      FullHop, Buttons::FullHop | Buttons::Jump)
        );
        /*
        if (*mappings.add(player_idx as usize)).pro_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
            }
        }*/
    }

    // Check if the button combos are being pressed and then force Stock Share + AttackRaw/SpecialRaw depending on input

    if controller.current_buttons.l()
    && controller.current_buttons.r()
    && controller.current_buttons.a()
    && (controller.current_buttons.minus() || controller.current_buttons.plus())
    {
        controller.current_buttons.set_plus(false);
        controller.current_buttons.set_minus(false);
        controller.just_down.set_plus(false);
        controller.just_down.set_minus(false);

        if controller.current_buttons.y() {
            (*out).buttons = Buttons::StockShare | Buttons::AttackRaw;
        } else if controller.current_buttons.x() {
            (*out).buttons = Buttons::StockShare | Buttons::SpecialRaw;
        } else {
            controller.current_buttons.set_plus(true);
            controller.current_buttons.set_minus(true);
            controller.just_down.set_plus(true);
            controller.just_down.set_minus(true);
        }
    }
}

#[skyline::hook(offset = crate::functions::offsets::analog_trigger_l(), inline)]
unsafe fn analog_trigger_l(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[9].x.as_ref() & 0x40 != 0 {
        *ctx.registers[11].x.as_mut() = 0;
    } else {
        *ctx.registers[11].w.as_mut() = 0x27FF;
    }
}

#[skyline::hook(offset = crate::functions::offsets::analog_trigger_r(), inline)]
unsafe fn analog_trigger_r(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[8].x.as_ref() & 0x80 != 0 {
        *ctx.registers[11].x.as_mut() = 0;
    } else {
        *ctx.registers[11].w.as_mut() = 0x27FF;
    }
}

static mut LAST_ALT_STICK: [f32; 2] = [0.0, 0.0];

//13.0.2 Offset: 0x3f7240
#[skyline::hook(offset = 0x3f7220)]
unsafe fn parse_inputs(this: &mut ControlModuleInternal) {
    const NEUTRAL: f32 = 0.2;
    const CLAMP_MAX: f32 = 120.0;

    // println!("this: {:#x}", this as *mut ControlModuleInternal as u64);

    if this.controller_index == -1 {
        return call_original!(this);
    }

    //println!("this.controller_index: {}", this.controller_index);
    // assert!(this.controller_index <= 7);

    let inputs = get_mapped_controller_inputs(this.controller_index as usize);

    let clamp_mul = 1.0 / CLAMP_MAX;

    // let raw_lstick_x = ((inputs.lstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
    // let raw_lstick_y = ((inputs.lstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);

    // let raw_lstick_x = if raw_lstick_x.abs() >= NEUTRAL { raw_lstick_x } else { 0.0 };
    // let raw_lstick_y = if raw_lstick_y.abs() >= NEUTRAL { raw_lstick_y } else { 0.0 };

    let raw_rstick_x = ((inputs.rstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
    let raw_rstick_y = ((inputs.rstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);

    LAST_ALT_STICK[0] = if raw_rstick_x.abs() >= NEUTRAL { raw_rstick_x } else { 0.0 };
    LAST_ALT_STICK[1] = if raw_rstick_y.abs() >= NEUTRAL { raw_rstick_y } else { 0.0 };

    call_original!(this)
}

//13.0.2 Offset: 0x6b9c7c
#[skyline::hook(offset = 0x6b9c5c, inline)]
unsafe fn after_exec(ctx: &skyline::hooks::InlineCtx) {
    let module = *ctx.registers[19].x.as_ref();
    let internal_class = *(module as *const u64).add(0x110 / 0x8);
    *(internal_class as *mut f32).add(0x40 / 0x4) = LAST_ALT_STICK[0];
    *(internal_class as *mut f32).add(0x44 / 0x4) = LAST_ALT_STICK[1];
}

//13.0.2 Offset: 0x16d7034
#[skyline::hook(offset = 0x16d7ee4, inline)]
unsafe fn handle_incoming_packet(ctx: &mut skyline::hooks::InlineCtx) {
    let packet = *ctx.registers[15].x.as_ref();

    let mut inputs = MappedInputs {
        buttons: Buttons::empty(),
        lstick_x: 0,
        lstick_y: 0,
        rstick_x: 0,
        rstick_y: 0
    };

    let raw_buttons = ((packet >> 16) & 0xFFFF_FFFF) as u32;
    let lstick_x = (packet & 0xFF) as i8;
    let lstick_y = ((packet & 0xFF00) >> 8) as i8;
    let rstick_x = ((packet >> 0x30) & 0xFF) as i8;
    let rstick_y = ((packet >> 0x38) & 0xFF) as i8;

    inputs.buttons = Buttons::from_bits_unchecked(raw_buttons as _);
    inputs.lstick_x = lstick_x;
    inputs.lstick_y = lstick_y;
    inputs.rstick_x = rstick_x;
    inputs.rstick_y = rstick_y;

    *ctx.registers[13].x.as_mut() = std::mem::transmute(inputs);
}

//Fix throws not respecting the cstick, especially dk cargo throw
#[skyline::hook(replace = L2CFighterCommon_IsThrowStick)]
unsafe extern "C" fn is_throw_stick(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut out = fighter.local_func__fighter_status_catch_1();
    let stick_x = fighter.stick_x() * PostureModule::lr(fighter.boma());
    let stick_y = fighter.stick_y();
    if stick_x > fighter.get_param_float("common", "attack_lw3_stick_x") {
        out["f"] = true.into();
    } else if stick_x < -fighter.get_param_float("common", "attack_lw3_stick_x") {
        out["b"] = true.into();
    }
    if stick_y > fighter.get_param_float("common", "attack_hi4_stick_y") {
        out["hi"] = true.into();
    } else if stick_y < fighter.get_param_float("common", "attack_lw4_stick_y") {
        out["lw"] = true.into();
    }
    out
}

//The following hooks are used to ignore setting the barrel's team, which resolves the issue of the Barrel Item being able to hit the item thrower for 1 frame. This is here since item status scripts aren't currently editable

#[skyline::hook(replace=TeamModule::set_hit_team)]
unsafe fn set_hit_team_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    if (boma.kind() == *ITEM_KIND_BARREL) {
        return;
    }
}

#[skyline::hook(replace=TeamModule::set_hit_team_second)]
unsafe fn set_hit_team_second_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    if (boma.is_item() && boma.kind() == *ITEM_KIND_BARREL) {
        return;
    }
}

#[skyline::hook(replace=TeamModule::set_team)]
unsafe fn set_team_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32, arg3: bool) {
    if (boma.is_item() && boma.kind() == *ITEM_KIND_BARREL) {} 
    else {
        original!()(boma, arg2, arg3);
    }
}

#[skyline::hook(replace=TeamModule::set_team_second)]
unsafe fn set_team_second_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    if (boma.is_item() && boma.kind() == *ITEM_KIND_BARREL) {
        return;
    }
}

#[skyline::hook(replace=TeamModule::set_team_owner_id)]
unsafe fn set_team_owner_id_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    if (boma.is_item() && boma.kind() == *ITEM_KIND_BARREL) {
        return;
    }
}

//A hook regarding the generation/visiblity of articles. Used to allow entry articles to generate
//13.0.2 Offset: 0x3a6670
#[skyline::hook(offset = 0x3a6650)]
unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
    let barrel_kind = *WEAPON_KIND_DONKEY_DKBARREL;
    let waddledee_kind = *WEAPON_KIND_DEDEDE_WADDLEDEE;
    if weapon_kind == barrel_kind {
        return 1;
    }
    if weapon_kind == waddledee_kind {
        return 2;
    }
    println!("Weapon: {weapon_kind} Entry: {entry_id} Barrels: {barrel_kind}");
    call_original!(weapon_kind, entry_id)
}

//Removes the death swap from PT
//13.0.2 Offset: 0xf96330
#[skyline::hook(offset = 0xf96310)]
unsafe fn ptrainer_death_swap() {}

//Permits parry reflecting
#[skyline::hook(replace=FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector_hook(_boma: &mut BattleObjectModuleAccessor) -> bool {
    true.into()
}

//Disables Training Mode Reset from resetting music (Credit to HDR)
//13.0.2 Offset: 0x23ed7f0
#[skyline::from_offset(0x23ecb70)]
unsafe fn music_function1(arg: u64);

//13.0.2 Offset: 0x23ee0a0
#[skyline::from_offset(0x23ed420)]
unsafe fn music_function2(arg: u64, arg2: u64);

//13.0.2 Offset: 0x1509fd4
#[skyline::hook(offset = 0x1509dc4, inline)]
unsafe fn training_reset_music1(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function1(*ctx.registers[0].x.as_ref());
    }
}

//13.0.2 Offset: 0x14f99cc
#[skyline::hook(offset = 0x14f97bc, inline)]
unsafe fn training_reset_music2(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function2(*ctx.registers[0].x.as_ref(), *ctx.registers[1].x.as_ref());
    }
}

//Credit to Claude
#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe fn const_allot_hook(unk: *const u8, constant: *const c_char, mut value: u32) {
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_LUIGI_STATUS_KIND_NUM") {
        value = 0x1F3;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_MIISWORDSMAN_STATUS_KIND_NUM") {
        value = 0x1FF;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_SONIC_STATUS_KIND_NUM") {
        value = 0x1F9;
    }
    original!()(unk,constant,value)
}

//13.0.2 Offset: 0x15db0b0
#[skyline::hook(offset = 0x15daea0)]
pub unsafe fn create_item(item_manager: *mut smash::app::ItemManager, create_item_param: *mut CreateItemParam, unk: bool, unk2: bool, unk3: bool) -> *mut BattleObject {
    if (*create_item_param).variation_kind > 7 {
        (*create_item_param).variation_kind = 0;
    }
    original!()(item_manager, create_item_param, unk, unk2, unk3)
}

#[skyline::hook(replace=FighterUtil::is_valid_auto_catch_item)]
pub unsafe fn is_valid_auto_catch_item_hook(module_accessor: &mut BattleObjectModuleAccessor, is_possible: bool) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_LINK {
        if WorkModule::is_flag(module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM) {
            return true;
        }
        else {
            original!()(module_accessor, is_possible)
        }
    }
    else {
        original!()(module_accessor, is_possible)
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(is_throw_stick);
    }
}

//Installation
pub fn install() {
	unsafe {
        //13.0.2 Version: skyline::patching::Patch::in_text(0x1D3592C).nop();
        skyline::patching::Patch::in_text(0x1d34e4c).nop();
        //Removes phantoms, 13.0.2 Version: skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32);
        skyline::patching::Patch::in_text(0x3e6ce8).data(0x14000012u32);
        //Resets projectile lifetime on parry, rather than using remaining lifetime, 13.0.2 Version skyline::patching::Patch::in_text(0x33bdfd8).nop(); skyline::patching::Patch::in_text(0x33bdfdc).data(0x2a0a03e1);
        skyline::patching::Patch::in_text(0x33bd358).nop();
        skyline::patching::Patch::in_text(0x33bd35c).data(0x2a0a03e1);
        //Assists with training mode music reset change, 13.0.2 Version skyline::patching::Patch::in_text(0x14f99cc).nop().unwrap(); skyline::patching::Patch::in_text(0x1509fd4).nop().unwrap();
        skyline::patching::Patch::in_text(0x14f97bc).nop().unwrap();
        skyline::patching::Patch::in_text(0x1509dc4).nop().unwrap();
    }
    skyline::install_hook!(change_status_hook);
    skyline::install_hook!(is_valid_just_shield_replace);
    skyline::install_hook!(attack_replace);
	skyline::install_hook!(attack_abs_replace);
	skyline::install_hook!(get_int_replace);
	skyline::install_hook!(is_enable_transition_term_replace);
	skyline::install_hook!(notify_log_event_collision_hit);
	skyline::install_hook!(on_flag_hook);
    skyline::install_hook!(ptrainer_death_swap);
    skyline::install_hook!(const_allot_hook);
	skyline::install_hooks!(
        hit_module_handle_attack_event,
        shield_module_send_shield_attack_collision_event,
		change_status_request_hook,
		change_version_string_hook,
		init_settings_hook,
        correct_hook,
        get_ground_correct_kind_air_trans_hook,
        attack_module_set_attack,
		get_button_label_by_operation_kind,
        add_footstool_to_gc,
        add_footstool_to_fk,
        add_footstool_to_jc,
        add_more_buttons,
		map_controls_hook,
        analog_trigger_l,
        analog_trigger_r,
        packed_packet_creation,
        write_packet,
        parse_inputs,
        handle_incoming_packet,
        after_exec,
        set_hit_team_hook,
        set_hit_team_second_hook,
        set_team_second_hook,
        set_team_hook,
        set_team_owner_id_hook,
        get_article_use_type_mask,
        is_valid_just_shield_reflector_hook,
        training_reset_music1,
        training_reset_music2,
        create_item,
        is_valid_auto_catch_item_hook
    );
	skyline::nro::add_hook(nro_hook).unwrap();
}