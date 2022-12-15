//Code Here comes from Championship Edition
#![allow(unused_macros)]
use {
	crate::functions::{
		B_CHECK,
		STATUS_KIND,
		get_player_number
	},
    smash::{
        hash40,
        app::lua_bind::*,
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
    },
    smashline::*,
};

unsafe fn air_specials(module_accessor: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64, status_kind: i32, situation_kind: i32) {
	if (fighter_kind == FIGHTER_KIND_SHEIK && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW)
	|| (fighter_kind == FIGHTER_KIND_MEWTWO && (status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2 || motion_kind == hash40("special_hi") || motion_kind == hash40("special_air_hi"))) 
	|| (fighter_kind == FIGHTER_KIND_PITB && status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH)
	|| (fighter_kind == FIGHTER_KIND_SZEROSUIT && status_kind == *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_START)
	|| (fighter_kind == FIGHTER_KIND_LITTLEMAC && status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP)
	|| (fighter_kind == FIGHTER_KIND_MIIFIGHTER && status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START)
	|| (fighter_kind == FIGHTER_KIND_KOOPAJR && status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT) {
		B_CHECK[get_player_number(module_accessor)] = true;
	}
	if situation_kind != SITUATION_KIND_AIR {
		B_CHECK[get_player_number(module_accessor)] = false;
	}
	if B_CHECK[get_player_number(module_accessor)] == false {
		if fighter_kind == FIGHTER_KIND_LITTLEMAC {
			if WorkModule::is_flag(module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) {
				WorkModule::off_flag(module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
			}
		}
	}
}

#[smashline::fighter_frame_callback]
fn special_after_infliction_install(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let fighter_kind = smash::app::utility::get_kind(module_accessor) as i32;
		if smash::app::utility::get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER {
			air_specials(module_accessor, fighter_kind, motion_kind, status_kind, situation_kind);
		}
	}
}

pub fn install() {
	install_agent_frame_callbacks!(
		special_after_infliction_install
	);
}