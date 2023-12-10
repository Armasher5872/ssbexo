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
	else if boma_reference.is_weapon() {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let entry_id = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *WEAPON_KIND_NESS_PK_FIRE
		&& param_type == hash40("param_pkfire") {
			if param_hash == hash40("life") {
				if OFFENSE_UP_ACTIVE[entry_id] == true {
					return 60;
				}
				else {
					return 20;
				}
			}
			if param_hash == hash40("pillar_life") {
				if OFFENSE_UP_ACTIVE[entry_id] == true {
					return 0;
				}
				else {
					return 100;
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_MEWTWO_SHADOWBALL
		&& param_type == hash40("param_shadowball")
		&& param_hash == hash40("life") {
			if STORED_POWER_ENABLED[entry_id] == 1 {
				return 120;
			}
			else {
				return 80;
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
	let status_checks = [
		*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, 
		*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, 
		*FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, 
		*FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR, *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD_START, *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD, *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL, 
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_LANDING, 
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT, 
		*FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LADDER, 
		*FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_S3, *FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_LW3
	];
	let smashes = [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4];
	let mut power = ((*AttackModule::attack_data(boma, 0, false)).power)*AttackModule::get_attack_power_mul_pattern(boma);
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
		if fighter_kind == *FIGHTER_KIND_MEWTWO {
			if param_type == hash40("ground_brake") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 0.04;
				}
				else {
					return 0.0754;
				}
			}
			if param_type == hash40("dash_speed") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 1.4;
				}
				else {
					return 1.65;
				}
			}
			if param_type == hash40("run_speed_max") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 1.6;
				}
				else {
					return 1.95;
				}
			}
			if param_type == hash40("jump_initial_y") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 20.0;
				}
				else {
					return 14.0;
				}
			}
			if param_type == hash40("air_accel_y") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 0.2;
				}
				else {
					return 0.08;
				}
			}
			if param_type == hash40("air_speed_y_stable") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 2.0;
				}
				else {
					return 1.5;
				}
			}
			if param_type == hash40("dive_speed_y") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 3.0;
				}
				else {
					return 2.4;
				}
			}
			if param_type == hash40("weight") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 115.0;
				}
				else {
					return 79.0;
				}
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
		if fighter_kind == FIGHTER_KIND_LITTLEMAC 
		&& param_type == hash40("param_special_n") {
			if param_hash == hash40("special_n_hit_damage_mul_") {
				if MAC_HITSTUN[get_player_number(boma_reference)] == 0 {
					let taken_damage = DamageModule::damage(boma, 0)-LAST_DAMAGE[get_player_number(boma_reference)];
					return -34.0/taken_damage;
				}
				if status_kind == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
					return -34.0;
				}
				return 0.0;
			}
			if param_hash == hash40("special_n_atk_damage_mul_") {
				for i in 0..TOTAL_FIGHTER {
					if COUNTERHIT_CHECK[get_player_number(&mut *get_boma(i))] && get_attacker_number(&mut *get_boma(i)) == get_player_number(boma_reference) && status_checks.contains(&status_kind) {
						if smashes.contains(&status_kind) {
							COUNTERHIT_SUCCESS[get_player_number(boma_reference)] = true;
						}
						COUNTERHIT_CHECK[get_player_number(&mut *get_boma(i))] = false;
						return 34.0/power;
					}
				}
				return 0.0;
			}
		}
	}
	else if boma_reference.is_weapon() {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let entry_id = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *WEAPON_KIND_KOOPA_BREATH {
			if param_type == hash40("param_special_n") {
				if param_hash == hash40("fire_speed_mul_max") {
					if CAN_FIREBALL[entry_id] == true {
						return 1.5;
					}
					else {
						return 1.2;
					}
				}
				if param_hash == hash40("fire_speed_mul_min") {
					if CAN_FIREBALL[entry_id] == true {
						return 1.5;
					}
					else {
						return 0.15;
					}
				}
				if param_hash == hash40("fire_speed_min") {
					if CAN_FIREBALL[entry_id] == true {
						return 0.57;
					}
					else {
						return 0.2;
					}
				}
			}
			if param_type == hash40("param_breath") {
				if param_hash == hash40("life") {
					if CAN_FIREBALL[entry_id] == true {
						return 70.0;
					}
					else {
						return 12.0;
					}
				}
				if param_hash == hash40("hit_frames") {
					if CAN_FIREBALL[entry_id] == true {
						return 70.0;
					}
					else {
						return 12.0;
					}
				}
				if param_hash == hash40("min_speed") {
					if CAN_FIREBALL[entry_id] == true {
						return 0.8;
					}
					else {
						return 0.5;
					}
				}
				if param_hash == hash40("max_speed") {
					if CAN_FIREBALL[entry_id] == true {
						return 0.8;
					}
					else {
						return 3.0;
					}
				}
			}
		}
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
		if fighter_kind == *WEAPON_KIND_MEWTWO_SHADOWBALL
		&& param_type == hash40("param_shadowball")
		&& param_hash == hash40("angle") {
			if STORED_POWER_ENABLED[entry_id] == 1 {
				if sticky > 0.5 {
					return 45.0;
				}
				else if sticky < -0.5 {
					return -45.0;
				}
			}
			else {
				return 0.0;
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