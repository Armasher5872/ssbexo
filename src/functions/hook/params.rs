use super::*;

//Param Adjustments
#[skyline::hook(offset = INT_OFFSET)]
unsafe extern "C" fn get_param_int_impl_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
	let boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
	let fighter_kind = boma_reference.kind();
	if boma_reference.is_fighter() {
		if fighter_kind == *FIGHTER_KIND_DONKEY
		&& (param_type == hash40("wall_jump_type") || param_type == hash40("attach_wall_type")) {
			if boma_reference.is_status_one_of(&[*FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_SPECIAL_S]) {
				return 0;
			}
			else {
				return 1;
			}
		}
	}
	original!()(module_accessor, param_type, param_hash)
}

#[skyline::hook(offset=FLOAT_OFFSET)]
unsafe extern "C" fn get_param_float_impl_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
	let boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
	let fighter_kind = boma_reference.kind();
	let sticky = ControlModule::get_stick_y(boma);
	let status_kind = StatusModule::status_kind(boma);
	if boma_reference.is_fighter() {
		if param_type == hash40("ground_brake") {
			if status_kind == *FIGHTER_STATUS_KIND_DASH {
				return original!()(module_accessor, param_type, param_hash)*1.5;
			}
		}
        if fighter_kind == *FIGHTER_KIND_DONKEY
		&& param_type == hash40("damage_level3") {
            if status_kind >= 481 && status_kind <= 489 {
                return original!()(module_accessor, param_type, param_hash)*0.5;
            }
        }
		if fighter_kind == *FIGHTER_KIND_YOSHI
		&& param_type == hash40("param_special_s")
		&& param_hash == hash40("jump_angle") {
			let max = 80.0;
			let min = 10.0;
			return (max*sticky).abs().clamp(min, max);
		}
		/*
		if fighter_kind == *FIGHTER_KIND_SONIC
		&& param_type == hash40("ground_brake") {
			if status_kind == *FIGHTER_STATUS_KIND_TURN_RUN {
				return 1.0;
			}
			else {
				return 0.0655;
			}
		}
		*/
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
	}
	original!()(module_accessor, param_type, param_hash)
}

pub fn install() {
	skyline::install_hooks!(
		get_param_int_impl_hook,
		get_param_float_impl_hook,
	);
}