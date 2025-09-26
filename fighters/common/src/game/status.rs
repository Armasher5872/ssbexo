#![allow(unused_parens)]
use super::*;

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
unsafe extern "C" fn is_enable_transition_term_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, term: i32) -> bool {
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

//Deals with Gordo
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request)]
unsafe extern "C" fn change_status_request_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
	let mut next_status = status_kind;
	let category = smash::app::utility::get_category(boma);
	let kind = smash::app::utility::get_kind(boma);
	if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if next_status == *FIGHTER_STATUS_KIND_CLIFF_WAIT {
            let cliff_id = GroundModule::get_cliff_id_uint32(boma);
            for object_id in get_all_active_battle_object_ids() {
                let object = get_battle_object_from_id(object_id);
                if !object.is_null() {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) {
                        continue;
                    }
                    if WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID) == cliff_id as i32 {
                        next_status = *FIGHTER_STATUS_KIND_CLIFF_ROBBED;
                    }
                }
            }
		}
	}
	else if (category == *BATTLE_OBJECT_CATEGORY_WEAPON && kind == *WEAPON_KIND_DEDEDE_GORDO) {
        if next_status == *WEAPON_DEDEDE_GORDO_STATUS_KIND_ATTACK || next_status == *WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP {
            HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_no_team(boma, true);
        }
    }
	original!()(boma, next_status, arg3)
}

//Change Status Request From Script
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request_from_script)]
unsafe extern "C" fn change_status_request_from_script_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
	let mut next_status = status_kind;
	let category = smash::app::utility::get_category(boma);
	if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if next_status == *FIGHTER_STATUS_KIND_CLIFF_WAIT {
            let cliff_id = GroundModule::get_cliff_id_uint32(boma);
            for object_id in get_all_active_battle_object_ids() {
                let object = get_battle_object_from_id(object_id);
                if !object.is_null() {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) {
                        continue;
                    }
                    if WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID) == cliff_id as i32 {
                        next_status = *FIGHTER_STATUS_KIND_CLIFF_ROBBED;
                    }
                }
            }
		}
	}
	original!()(boma, next_status, arg3)
}

//Credit to HDR
pub unsafe extern "C" fn init_settings_edges(boma: &mut BattleObjectModuleAccessor, _situation: smash::app::SituationKind, _arg3: i32, arg4: u32, _ground_cliff_check_kind: smash::app::GroundCliffCheckKind, _arg6: bool, _arg7: i32, _arg8: i32, _arg9: i32, _arg10: i32) -> u32 {
	/* "fix" forces GroundModule::correct to be called for the statuses we need */
    let mut fix = arg4;
	let category = smash::app::utility::get_category(boma);
    let fighter_kind = smash::app::utility::get_kind(boma);
	let situation_kind = StatusModule::situation_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
	if category == *BATTLE_OBJECT_CATEGORY_FIGHTER && situation_kind == *SITUATION_KIND_GROUND {
		if [
			*FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, 
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING
		].contains(&status_kind) {
			fix = *GROUND_CORRECT_KIND_GROUND as u32;
		}
		if (fighter_kind == *FIGHTER_KIND_FOX && [*FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PIKACHU && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_LUIGI && status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL)
		|| (fighter_kind == *FIGHTER_KIND_NESS && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_CAPTAIN && status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)
		|| (fighter_kind == *FIGHTER_KIND_SHEIK && [*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_ZELDA && [*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PICHU && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind))
        || (fighter_kind == *FIGHTER_KIND_FALCO && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_S_FALL_LANDING, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_YOUNGLINK && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
		|| ([*FIGHTER_KIND_PIT, *FIGHTER_KIND_PITB].contains(&fighter_kind) && status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_LANDING)
		|| (fighter_kind == *FIGHTER_KIND_SZEROSUIT && [*FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_FLIP, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_LANDING, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PLIZARDON && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_RUSH, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_BLOWN].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_DIDDY && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
		|| (fighter_kind == *FIGHTER_KIND_LUCAS && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_AGAIN, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_REFLECT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_SONIC && [*FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_REBOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PIKMIN && status_kind == *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_LANDING)
		|| (fighter_kind == *FIGHTER_KIND_LUCARIO && [*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_TOONLINK && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
		|| (fighter_kind == *FIGHTER_KIND_WOLF && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_MURABITO && status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING)
		|| (fighter_kind == *FIGHTER_KIND_PALUTENA && [*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_GEKKOUGA && status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_LOOP)
		|| (fighter_kind == *FIGHTER_KIND_KOOPAJR && status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING)
		|| ([*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN].contains(&fighter_kind) && [*FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_KAMUI && [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK_END, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_BAYONETTA && [*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_RIDLEY && status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING)
		|| ([*FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER].contains(&fighter_kind) && status_kind == *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING)
		|| (fighter_kind == *FIGHTER_KIND_KROOL && status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_LANDING)
		|| (fighter_kind == *FIGHTER_KIND_GAOGAEN && [*FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_BOUND].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PACKUN && status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING)
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

//Credit to HDR
#[skyline::hook(replace = StatusModule::init_settings)]
unsafe extern "C" fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, kinetic_type: i32, arg4: u32, ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool, keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
    let mut cliff_check_kind = ground_cliff_check_kind;
	let category = smash::app::utility::get_category(boma);     
	let kind = smash::app::utility::get_kind(boma);
	let status_kind = StatusModule::status_kind(boma);
    //Call Edge Cancel init_settings
    let fix = init_settings_edges(boma, situation, kinetic_type, arg4, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10);
	//Set GroundCliffCheckKind here to pass into init_settings
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if ([*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN].contains(&kind) && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind))
        || ([*FIGHTER_KIND_FALCO, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_REFLET].contains(&kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI) {
            cliff_check_kind = smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }
		//Assign Ledge ID
		if [*FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&status_kind) {
			let cliff_id = GroundModule::get_cliff_id_uint32(boma);
			WorkModule::set_int(boma, cliff_id as i32, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
		}
    }
    original!()(boma, situation, kinetic_type, fix, cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10)
}

//GroundModule::correct. The Edge Cancel function (Credit to HDR)
#[skyline::hook(replace = GroundModule::correct)]
unsafe extern "C" fn correct_hook(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {
    let status_kind = StatusModule::status_kind(boma);
	let situation_kind = StatusModule::situation_kind(boma);
	let category = smash::app::utility::get_category(boma);
    let fighter_kind = smash::app::utility::get_kind(boma);
	//All statuses seem to count as "landing" for some reason
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && situation_kind == *SITUATION_KIND_GROUND {
        if [*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
            return original!()(boma, GroundCorrectKind(1));
        }
        if (fighter_kind == *FIGHTER_KIND_YOSHI && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
		|| (fighter_kind == *FIGHTER_KIND_LUIGI && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N)
        || (fighter_kind == *FIGHTER_KIND_CAPTAIN && status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)
		|| (([*FIGHTER_KIND_PEACH, *FIGHTER_KIND_DAISY].contains(&fighter_kind)) && status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END)
		|| (fighter_kind == *FIGHTER_KIND_KOOPA && status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G)
        || (fighter_kind == *FIGHTER_KIND_PICHU && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind))
        || (fighter_kind == *FIGHTER_KIND_GANON && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && !is_armstrong_slots(boma))
		|| (fighter_kind == *FIGHTER_KIND_GAOGAEN && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N)
        || (fighter_kind == *FIGHTER_KIND_EDGE && status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH)
        || (fighter_kind == *FIGHTER_KIND_MIISWORDSMAN && ([*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END].contains(&status_kind) || (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO) == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3 && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW))) {
            return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    original!()(boma, kind)
}

#[skyline::hook(replace=get_ground_correct_kind_air_trans)]
unsafe extern "C" fn get_ground_correct_kind_air_trans_hook(_boma: &mut smash::app::BattleObjectModuleAccessor, _something: i32) -> i32 {
    return *GROUND_CORRECT_KIND_AIR;
}

#[skyline::hook(replace = FighterStatusModuleImpl::set_fighter_status_data)]
unsafe extern "C" fn set_fighter_status_data_hook(boma: &mut BattleObjectModuleAccessor, arg2: bool, treaded_kind: i32, arg4: bool, arg5: bool, arg6: bool, log_mask_flag: u64, status_attr: u32, power_up_attack_bit: u32, arg10: u32) {
    let mut new_status_attr = status_attr;
	let category = smash::app::utility::get_category(boma);
	let kind = smash::app::utility::get_kind(boma);
	let status_kind = StatusModule::status_kind(boma);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        // this handles turnaround special/b-reversible moves
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
			if kind == *FIGHTER_KIND_MIIFIGHTER {
				// if b-reverse flag does not already exist in status_attr bitmask
				if status_attr & *FIGHTER_STATUS_ATTR_START_TURN as u32 == 0 {
					// add b-reverse flag to status_attr bitmask
					new_status_attr = status_attr + *FIGHTER_STATUS_ATTR_START_TURN as u32;
				}
			}
		}
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
			if [*FIGHTER_KIND_LINK, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_TOONLINK].contains(&kind) {
				// if b-reverse flag does not already exist in status_attr bitmask
				if status_attr & *FIGHTER_STATUS_ATTR_START_TURN as u32 == 0 {
					// add b-reverse flag to status_attr bitmask
					new_status_attr = status_attr + *FIGHTER_STATUS_ATTR_START_TURN as u32;
				}
			}
		}

    }
    original!()(boma, arg2, treaded_kind, arg4, arg5, arg6, log_mask_flag, new_status_attr, power_up_attack_bit, arg10)
}

pub fn install() {
	skyline::install_hooks!(
		is_enable_transition_term_replace,
		change_status_request_hook,
		change_status_request_from_script_hook,
		init_settings_hook,
        correct_hook,
        get_ground_correct_kind_air_trans_hook,
		set_fighter_status_data_hook
    );
}