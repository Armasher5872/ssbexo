#![allow(unused_parens)]
use super::*;

//Prevention of Moves in Air (Credit to Chrispo)
#[skyline::hook(replace = StatusModule::change_status_request_from_script)]
unsafe extern "C" fn change_status_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, unk: bool) -> u64 {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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

//Deals with DK's Barrels, Gordo, and CC
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request)]
unsafe extern "C" fn change_status_request_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
	let mut next_status = status_kind;
    if (boma.is_weapon() && boma.kind() == *WEAPON_KIND_DEDEDE_GORDO) {
        if next_status == *WEAPON_DEDEDE_GORDO_STATUS_KIND_ATTACK || next_status == *WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP {
            HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_no_team(boma, true);
        }
    }
	else if boma.is_item() {
		if boma.kind() == *ITEM_KIND_BARREL {
			if next_status == *ITEM_STATUS_KIND_BORN || next_status == *ITEM_STATUS_KIND_LOST {
				let bounce_mul = Vector3f { x: -0.25, y: -0.25, z: -0.25 };
				KineticModule::mul_speed(boma, &bounce_mul, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
				PostureModule::reverse_lr(boma);
				AttackModule::clear_all(boma);
				next_status = *ITEM_STATUS_KIND_FALL;
				TeamModule::set_hit_team(boma, *TEAM_NONE);
			}
		}
		if boma.kind() == *ITEM_KIND_SNAKEGRENADE {
			TeamModule::set_hit_team(boma, *TEAM_NONE);
		}
	}
	original!()(boma, next_status, arg3)
}

//Credit to HDR
pub unsafe extern "C" fn init_settings_edges(boma: &mut BattleObjectModuleAccessor, _situation: smash::app::SituationKind, _arg3: i32, arg4: u32, _ground_cliff_check_kind: smash::app::GroundCliffCheckKind, _arg6: bool, _arg7: i32, _arg8: i32, _arg9: i32, _arg10: i32) -> u32 {
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
		if (fighter_kind == *FIGHTER_KIND_FOX && [*FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PIKACHU && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_LUIGI && boma.is_status(*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL))
		|| (fighter_kind == *FIGHTER_KIND_NESS && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_CAPTAIN && boma.is_status(*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END))
		|| (fighter_kind == *FIGHTER_KIND_SHEIK && [*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_ZELDA && [*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_PICHU && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind))
        || (fighter_kind == *FIGHTER_KIND_FALCO && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_S_FALL_LANDING, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind))
		|| (fighter_kind == *FIGHTER_KIND_YOUNGLINK && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI))
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
unsafe extern "C" fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, kinetic_type: i32, arg4: u32, ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool, keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
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
unsafe extern "C" fn correct_hook(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {
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
unsafe extern "C" fn get_ground_correct_kind_air_trans_hook(_boma: &mut smash::app::BattleObjectModuleAccessor, _something: i32) -> i32 {
    return *GROUND_CORRECT_KIND_AIR;
}

pub fn install() {
	skyline::install_hooks!(
		change_status_hook,
		is_enable_transition_term_replace,
		change_status_request_hook,
		init_settings_hook,
        correct_hook,
        get_ground_correct_kind_air_trans_hook
    );
}