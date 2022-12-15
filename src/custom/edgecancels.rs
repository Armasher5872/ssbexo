#![allow(unused_macros)]
use {
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
    }
};

pub(crate) fn is_edge_cancel(fighter_kind : i32, status_kind : i32) -> bool {
	let edge_cancel = [
        [*FIGHTER_KIND_FOX, *FIGHTER_STATUS_KIND_SPECIAL_S],
        [*FIGHTER_KIND_FOX, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH],
        [*FIGHTER_KIND_FOX, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END],
        [*FIGHTER_KIND_PIKACHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK],
        [*FIGHTER_KIND_PIKACHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK],
        [*FIGHTER_KIND_PIKACHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END],
        [*FIGHTER_KIND_PIKACHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP],
        [*FIGHTER_KIND_PIKACHU, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT],
        [*FIGHTER_KIND_NESS, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END],
        [*FIGHTER_KIND_CAPTAIN, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END],
        [*FIGHTER_KIND_SHEIK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE],
        [*FIGHTER_KIND_SHEIK, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END],
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
        [*FIGHTER_KIND_GANON, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END],
        [*FIGHTER_KIND_MEWTWO, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2],
        [*FIGHTER_KIND_MEWTWO, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3],
        [*FIGHTER_KIND_DIDDY, *FIGHTER_STATUS_KIND_SPECIAL_HI],
        [*FIGHTER_KIND_SONIC, *FIGHTER_STATUS_KIND_ATTACK_DASH],
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
        [*FIGHTER_KIND_MIIFIGHTER, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING],
        [*FIGHTER_KIND_GEKKOUGA, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_LOOP],
        [*FIGHTER_KIND_PALUTENA, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2],
        [*FIGHTER_KIND_PALUTENA, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3],
        [*FIGHTER_KIND_RIDLEY, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING],
        [*FIGHTER_KIND_GAOGAEN, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END],
        [*FIGHTER_KIND_GAOGAEN, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_BOUND],
        [*FIGHTER_KIND_PACKUN, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END]
    ];
	for i in &edge_cancel {
		if fighter_kind == i[0] && status_kind == i[1] {
			return true;
		};
	};
	return false;
}

//Edge Cancelling Part A
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::init_settings)]
unsafe fn init_settings_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, situation_kind: i32, arg3: i32, arg4: u64, ground_cliff_check_kind: u64, arg6: bool, arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u64 {
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let frame = MotionModule::frame(module_accessor);
    if is_edge_cancel(fighter_kind, status_kind) && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if ![*FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind)
	&& fighter_kind != *FIGHTER_KIND_BUDDY {
		original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
	} 
    else if [*FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
		original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
	}
    else if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
		if ControlModule::get_stick_y(module_accessor) >= 0.66 {
			original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
		}
		else {
			original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
		}
	}
    else if [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_DIDDY].contains(&fighter_kind) 
    && (status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH)
    && situation_kind == SITUATION_KIND_GROUND
    && frame >= 8.0 {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
        KineticModule::clear_speed_all(module_accessor)
    }
	else if fighter_kind == FIGHTER_KIND_BUDDY {
		if ([*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind)) 
        && situation_kind == SITUATION_KIND_GROUND {
			original!()(module_accessor, situation_kind, arg3, 7 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
		}
		else {
			original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
        }
	}
    else if fighter_kind == *FIGHTER_KIND_EDGE
    && ([*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind))
    && (StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH)
    && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
        KineticModule::clear_speed_all(module_accessor)
    }
    else {
        original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
}

//Edge Cancelling Part B
#[skyline::hook(replace = smash::app::lua_bind::GroundModule::correct)]
unsafe fn correct_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, ground_correct_kind: u32) -> u64 {
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let frame = MotionModule::frame(module_accessor);
    let situation_kind = StatusModule::situation_kind(module_accessor);
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) {
        original!()(module_accessor, 1 as u32)
    }
    else if is_edge_cancel(fighter_kind, status_kind) {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32)
    }
    else if [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_DIDDY].contains(&fighter_kind) 
    && (status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH)
    && situation_kind == SITUATION_KIND_GROUND
    && frame >= 8.0 {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32);
        KineticModule::clear_speed_all(module_accessor)
    }
    else if fighter_kind == *FIGHTER_KIND_EDGE
    && ([*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind))
    && (StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH)
    && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32);
        KineticModule::clear_speed_all(module_accessor)
    }
    else {
        original!()(module_accessor, ground_correct_kind)
    }
}

pub fn install() {
    skyline::install_hooks!(
        init_settings_replace,
        correct_replace
    );
}