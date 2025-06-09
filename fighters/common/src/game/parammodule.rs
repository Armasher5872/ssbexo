use super::*;

#[skyline::hook(offset=FLOAT_OFFSET)]
unsafe extern "C" fn get_param_float_impl_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
	let boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
	let category = smash::app::utility::get_category(boma_reference);
    let fighter_kind = smash::app::utility::get_kind(boma_reference);
	let sticky = ControlModule::get_stick_y(boma);
	if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if fighter_kind == *FIGHTER_KIND_YOSHI
		&& param_type == hash40("param_special_s")
		&& param_hash == hash40("jump_angle") {
			let max = 80.0;
			let min = 10.0;
			return (max*sticky).abs().clamp(min, max);
		}
		if fighter_kind == *FIGHTER_KIND_MIIFIGHTER
		&& param_type == hash40("param_special_n")
		&& param_hash == hash40("n1_throw_angle") {
			let max = 90.0;
			let min = 0.0;
			return (max*sticky).abs().clamp(min, max);
		}
		if fighter_kind == *FIGHTER_KIND_PALUTENA {
			if param_type == hash40("walk_accel_mul") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 0.4;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.11;
				}
				else {
					return 0.21;
				}
			}
			if param_type == hash40("walk_accel_add") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 0.2;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.05;
				}
				else {
					return 0.105;
				}
			}
			if param_type == hash40("walk_speed_max") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 1.6;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.77;
				}
				else {
					return 1.271;
				}
			}
			if param_type == hash40("dash_speed") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 2.2;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 1.22;
				}
				else {
					return 1.768;
				}
			}
			if param_type == hash40("run_accel_mul") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 0.3;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.03;
				}
				else {
					return 0.12991;
				}
			}
			if param_type == hash40("run_accel_add") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 0.2;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.0;
				}
				else {
					return 0.044;
				}
			}
			if param_type == hash40("run_speed_max") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 2.6;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 1.4;
				}
				else {
					return 2.077;
				}
			}
			if param_type == hash40("jump_initial_y") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 26.0;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 13.0;
				}
				else {
					return 19.745;
				}
			}
			if param_type == hash40("jump_y") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 50.0;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 22.0;
				}
				else {
					return 35.9;
				}
			}
			if param_type == hash40("mini_jump_y") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 13.0;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 21.0;
				}
				else {
					return 17.3;
				}
			}
			if param_type == hash40("jump_aerial_y") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 50.0;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 22.0;
				}
				else {
					return 35.9;
				}
			}
			if param_type == hash40("air_accel_x_mul") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 0.2;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.04;
				}
				else {
					return 0.105;
				}
			}
			if param_type == hash40("air_accel_x_add") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 0.06;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.0;
				}
				else {
					return 0.01;
				}
			}
			if param_type == hash40("air_speed_x_stable") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 1.4;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.75;
				}
				else {
					return 1.0;
				}
			}
			if param_type == hash40("air_brake_x") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 0.05;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.0;
				}
				else {
					return 0.01;
				}
			}
			if param_type == hash40("air_accel_y") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 0.2;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.09;
				}
				else {
					return 0.12;
				}
			}
			if param_type == hash40("air_speed_y_stable") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 1.9;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 1.2;
				}
				else {
					return 1.55;
				}
			}
			if param_type == hash40("damage_fly_top_air_accel_y") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 0.2;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 0.09;
				}
				else {
					return 0.12;
				}
			}
			if param_type == hash40("damage_fly_top_speed_y_stable") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 1.9;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 1.2;
				}
				else {
					return 1.55;
				}
			}
			if param_type == hash40("dive_speed_y") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 3.0;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 2.0;
				}
				else {
					return 2.48;
				}
			}
			if param_type == hash40("weight") {
				if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
					return 50.0;
				}
				else if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
					return 83.0;
				}
				else {
					return 91.0;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_CLOUD {
			if param_type == hash40("walk_speed_max") {
				if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
					return 0.4;
				}
				else {
					return original!()(module_accessor, param_type, param_hash);
				}
			}
			if param_type == hash40("dash_speed") {
				if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
					return 0.8;
				}
				else {
					return original!()(module_accessor, param_type, param_hash);
				}
			}
			if param_type == hash40("run_speed_max") {
				if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
					return 1.3;
				}
				else {
					return original!()(module_accessor, param_type, param_hash);
				}
			}
			if param_type == hash40("jump_speed_x") {
				if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
					return 1.018;
				}
				else {
					return original!()(module_accessor, param_type, param_hash);
				}
			}
			if param_type == hash40("jump_speed_x_max") {
				if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
					return 1.3;
				}
				else {
					return original!()(module_accessor, param_type, param_hash);
				}
			}
			if param_type == hash40("air_speed_x_stable") {
				if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
					return 1.018;
				}
				else {
					return original!()(module_accessor, param_type, param_hash);
				}
			}
			if param_type == hash40("air_accel_y") {
				if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
					return 0.12;
				}
				else {
					return original!()(module_accessor, param_type, param_hash);
				}
			}
			if param_type == hash40("air_speed_y_stable") {
				if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
					return 1.88;
				}
				else {
					return original!()(module_accessor, param_type, param_hash);
				}
			}
			if param_type == hash40("dive_speed_y") {
				if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
					return 2.9;
				}
				else {
					return original!()(module_accessor, param_type, param_hash);
				}
			}
			if param_type == hash40("param_private") {
				let limit_level = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
				let gauge;
				if param_hash == hash40("limit_gauge_damage_add") {
					match limit_level {
						0 => {
							gauge = 5.0;
						},
						1 => {
							gauge = 3.333;
						},
						2 => {
							gauge = 2.222;
						},
						3 => {
							gauge = 1.667;
						},
						_ => {
							gauge = 1.0;
						}
					}
					return gauge;
				}
				if param_hash == hash40("limit_gauge_attack_add") {
					match limit_level {
						0 => {
							gauge = 3.333;
						},
						1 => {
							gauge = 2.5;
						},
						2 => {
							gauge = 1.667;
						},
						3 => {
							gauge = 1.25;
						},
						_ => {
							gauge = 1.0;
						}
					}	
					return gauge;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_INKLING {
			if param_type == hash40("dash_speed") {
				if WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_ON_ROLLER_INK) {
					return original!()(module_accessor, param_type, param_hash);
				}
				else {
					return 1.5;
				}
			}
			if param_type == hash40("run_speed_max") {
				if WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_ON_ROLLER_INK) {
					return original!()(module_accessor, param_type, param_hash);
				}
				else {
					return 1.6;
				}
			}
		}
	}
	original!()(module_accessor, param_type, param_hash)
}

pub fn install() {
	skyline::install_hook!(get_param_float_impl_hook);
}