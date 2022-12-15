//Code Here comes from Championship Edition
#![allow(unused_macros)]
use {
    smash::{
        app::{
            lua_bind::*,
        },
		hash40,
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
    },
    smashline::*,
};

unsafe fn grab_break(module_accessor: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64) {
	if fighter_kind == FIGHTER_KIND_POPO {
		if motion_kind == hash40("catch_dash") {
			if MotionModule::frame(module_accessor) == 0.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ true);
			}
			if MotionModule::frame(module_accessor) == 9.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ false);
			}
		}
		if motion_kind == hash40("catch_turn") {
			if MotionModule::frame(module_accessor) == 0.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ true);
			}
			if MotionModule::frame(module_accessor) == 10.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ false);
			}
		}
	}
	if fighter_kind == FIGHTER_KIND_INKLING {
		if motion_kind == hash40("catch") {
			if MotionModule::frame(module_accessor) == 0.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ true);
			}
			if MotionModule::frame(module_accessor) == 9.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ false);
			}
		}
		if motion_kind == hash40("catch_dash") {
			if MotionModule::frame(module_accessor) == 0.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ true);
			}
			if MotionModule::frame(module_accessor) == 10.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ false);
			}
		}
		if motion_kind == hash40("catch_turn") {
			if MotionModule::frame(module_accessor) == 0.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ true);
			}
			if MotionModule::frame(module_accessor) == 11.0 {
				GrabModule::set_rebound(module_accessor, /*CanCatchRebound*/ false);
			}
		}
	}
}

#[smashline::fighter_frame_callback]
fn grab_break_install(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let fighter_kind = smash::app::utility::get_kind(module_accessor) as i32;
		if smash::app::utility::get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER {
			grab_break(module_accessor, fighter_kind, motion_kind);
		}
	}
}

pub fn install() {
	install_agent_frame_callbacks!(
		grab_break_install
	);
}