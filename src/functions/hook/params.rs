#![allow(unused_variables)]
use super::*;

//Param Adjustments (mainly used in things like Bowsers Fireballs and Ness's PSIOU PK Fire)
#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_impl_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
	let boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
	let fighter_kind = boma_reference.kind();
	let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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
		if fighter_kind == *FIGHTER_KIND_PICHU
		&& param_type == hash40("param_special_hi")
		&& param_hash == hash40("special_hi_warp2_angle_") {
			if DISCHARGE_ACTIVE[entry_id] == true {
				return 35;
			}
			else {
				return 360;
			}
		}
	}
	original!()(module_accessor, param_type, param_hash)
}

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn get_param_float_impl_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
	let boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
	let fighter_kind = boma_reference.kind();
	let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let sticky = ControlModule::get_stick_y(boma);
	let status_kind = StatusModule::status_kind(boma);
	if boma_reference.is_fighter() {
        if fighter_kind == *FIGHTER_KIND_DONKEY
		&& param_type == hash40("damage_level3") {
            let status = boma_reference.status();
            if status >= 481 && status <= 489 {
                return original!()(module_accessor, param_type, param_hash)*0.5;
            }
        }
		if fighter_kind == *FIGHTER_KIND_YOSHI
		&& param_type == hash40("param_special_s")
		&& param_hash == hash40("jump_angle") {
			let max = 80.0;
			let min = 10.0;
			return (max*ControlModule::get_stick_x(boma)*PostureModule::lr(boma)).abs().clamp(min, max);
		}
		if fighter_kind == *FIGHTER_KIND_PICHU
		&& param_type == hash40("param_special_hi")
		&& param_hash == hash40("special_hi_warp_spd_add") {
			if DISCHARGE_ACTIVE[entry_id] == true {
				return 6.5;
			}
			else {
				return 9.0;
			}
		}
		if fighter_kind == *FIGHTER_KIND_PZENIGAME
		&& param_type == hash40("ground_brake") {
			if IN_RAIN_DANCE[entry_id] == true {
				return 0.0143;
			}
			else {
				return 0.0572;
			}
		}
		if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU
		&& param_type == hash40("ground_brake") {
			if IN_RAIN_DANCE[entry_id] == true {
				return 0.0197;
			}
			else {
				return 0.07865;
			}
		}
		if fighter_kind == *FIGHTER_KIND_PLIZARDON
		&& param_type == hash40("ground_brake") {
			if IN_RAIN_DANCE[entry_id] == true {
				return 0.0179;
			}
			else {
				return 0.0715;
			}
		}
		if fighter_kind == *FIGHTER_KIND_SONIC
		&& param_type == hash40("ground_brake") {
			if status_kind == *FIGHTER_STATUS_KIND_TURN_RUN {
				return 1.0;
			}
			else {
				return 0.0655;
			}
		}
	}
	else if boma_reference.is_weapon() {
        let owner_module_accessor = &mut *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let entry_id = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *WEAPON_KIND_PICHU_DENGEKI
		&& param_type == hash40("param_degeki")
		&& param_hash == hash40("move_life_") {
			if DISCHARGE_ACTIVE[entry_id] == true {
				return 180.0;
			}
			else {
				return 0.0;
			}
		}
		if fighter_kind == *WEAPON_KIND_PICHU_DENGEKIDAMA
	    && param_type == hash40("param_degekidama")
		&& param_hash == hash40("life_") {
			if DISCHARGE_ACTIVE[entry_id] == true {
				return 180.0;
			}
			else {
				return 0.0;
			}
		}
		if fighter_kind == *WEAPON_KIND_PICHU_KAMINARI
		&& param_type == hash40("param_kaminari") {
			if param_hash == hash40("speed_") {
				if DISCHARGE_ACTIVE[entry_id] == true {
					return 0.0;
				}
				else {
					return -4.9;
				}
			}
			if param_hash == hash40("flying_dist_") {
				if DISCHARGE_ACTIVE[entry_id] == true {
					return 0.0;
				}
				else {
					return 30.0;
				}
			}
			if param_hash == hash40("pass_fall_dist_") {
				if DISCHARGE_ACTIVE[entry_id] == true {
					return 0.0;
				}
				else {
					return 7.5;
				}
			}
			if param_hash == hash40("width_") {
				if DISCHARGE_ACTIVE[entry_id] == true {
					return 0.0;
				}
				else {
					return 1.7;
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_PICHU_CLOUD
		&& param_type == hash40("param_cloud") {
			if param_hash == hash40("speed_") {
				if DISCHARGE_ACTIVE[entry_id] == true {
					return 0.0;
				}
				else {
					return -4.9;
				}
			}
			if param_hash == hash40("width_") {
				if DISCHARGE_ACTIVE[entry_id] == true {
					return 0.0;
				}
				else {
					return 1.7;
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_SNAKE_TRENCHMORTAR_BULLET
		&& param_type == hash40("param_trenchmortarbullet")
		&& param_hash == hash40("speed_x") {
			return ControlModule::get_stick_x(boma) / 1.5 * PostureModule::lr(boma);
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