//Code Here comes from Championship Edition
#![allow(unused_macros)]
use {
    smash::{
        app::{
            lua_bind::{
                PostureModule,
                *
            }
        },
		hash40,
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
        phx::Vector3f
    },
    smashline::*,
};

unsafe fn b_reverse(module_accessor: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64, globals: &mut smash::lib::L2CValue) {
	if fighter_kind == FIGHTER_KIND_SNAKE {
		if motion_kind == hash40("special_hi_start") || motion_kind == hash40("special_air_hi_start") {
			if globals["assignment_check"].get_bool() == false {
				if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) > -0.66 {
					globals["momentum"] = true.into();
				}
				globals["assignment_check"] = true.into();
			}
			if MotionModule::frame(module_accessor) <= 3.0 && MotionModule::frame(module_accessor) > 0.0 {
				if globals["turnaround"].get_bool() {
					PostureModule::reverse_lr(module_accessor);
					PostureModule::update_rot_y_lr(module_accessor);
					globals["turnaround"] = false.into();
				}
				if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) <= -0.66 {
					if globals["turnaround"].get_bool() == false {
						PostureModule::reverse_lr(module_accessor);
						PostureModule::update_rot_y_lr(module_accessor);
					}
					if globals["momentum"].get_bool() {
						let reverse_mul = Vector3f{x: -1.0, y: 1.0, z: 1.0};
						KineticModule::mul_speed(module_accessor, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
						globals["momentum"] = false.into();
					}
				}
			}
		}
		else {
			if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) <= -0.66 {
				globals["reverse_timer"] = 0.0.into();
				globals["turnaround"] = true.into();
			}
			else {
				if globals["reverse_timer"].get_num() >= 4.0 {
					globals["turnaround"] = false.into();
				}
				globals["reverse_timer"] = (globals["reverse_timer"].get_num() + 1.0).into()
			}
			globals["assignment_check"] = false.into();
			globals["momentum"] = false.into();
		}
	}
	if fighter_kind == FIGHTER_KIND_KROOL {
		if motion_kind == hash40("special_air_lw") {
			if globals["assignment_check"].get_bool() == false {
				if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) > -0.66 {
					globals["momentum"] = true.into();
				}
				globals["assignment_check"] = true.into();
			}
			if MotionModule::frame(module_accessor) <= 3.0 {
				if globals["turnaround"].get_bool() {
					PostureModule::reverse_lr(module_accessor);
					PostureModule::update_rot_y_lr(module_accessor);
					globals["turnaround"] = false.into();
				}
				if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) <= -0.66 {
					if globals["turnaround"].get_bool() == false {
						PostureModule::reverse_lr(module_accessor);
						PostureModule::update_rot_y_lr(module_accessor);
					}
					if globals["momentum"].get_bool() {
						let reverse_mul = Vector3f{x: -1.0, y: 1.0, z: 1.0};
						KineticModule::mul_speed(module_accessor, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
						globals["momentum"] = false.into();
					}
				}
			}
		}
		else {
			if (PostureModule::lr(module_accessor) * ControlModule::get_stick_x(module_accessor)) <= -0.66 {
				globals["reverse_timer"] = 0.0.into();
				globals["turnaround"] = true.into();
			}
			else {
				if globals["reverse_timer"].get_num() >= 4.0 {
					globals["turnaround"] = false.into();
				}
				globals["reverse_timer"] = (globals["reverse_timer"].get_num() + 1.0).into()
			}
			globals["assignment_check"] = false.into();
			globals["momentum"] = false.into();
		}
	}
}

#[smashline::fighter_frame_callback]
fn b_reverse_install(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let fighter_kind = smash::app::utility::get_kind(module_accessor) as i32;
		let mut globals = fighter.globals_mut().clone();
		if let smash::lib::L2CValueType::Void = globals["globals_set"].val_type {
			globals["parry_lag_set"] = false.into();
			globals["parry_lag"] = (-1.0).into();
			globals["real_parry_lag"] = (-1.0).into();
			globals["reset_frame"] = (-1.0).into();
			globals["reset_motion"] = (hash40("invalid")).into();
			globals["rewound"] = false.into();
			globals["rewind_timer"] = 0.into();
			globals["rewind_start"] = false.into();
			globals["prewind_gravity"] = 0.0.into();
			globals["last_nonhitlag_rate"] = 0.0.into();
			globals["reverse_timer"] = 0.0.into();
			globals["assignment_check"] = false.into();
			globals["turnaround"] = false.into();
			globals["momentum"] = false.into();
			globals["seed_check"] = false.into();
			globals["seed_id"] = 0.into();
			globals["seed_changed"] = false.into();
			globals["crit_ok"] = 0.into();
			globals["microdash_check"] = false.into();
			globals["microdash_success"] = false.into();
			globals["grab_check"] = false.into();
			globals["shield_check"] = false.into();
			globals["globals_set"] = true.into();
		}
		if smash::app::utility::get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER {
			b_reverse(module_accessor, fighter_kind, motion_kind, &mut globals);
		}
	}
}

pub fn install() {
	install_agent_frame_callbacks!(
		b_reverse_install
	);
}